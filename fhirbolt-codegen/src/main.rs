use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::str;

use chrono::Utc;
use proc_macro2::TokenStream;
use zip::read::{ZipArchive, ZipFile};

use fhirbolt_codegen::{generate_all, model::Bundle, SourceFile};

const BUILD_FHIR_RELEASES: &[&str] = &["R4", "R4B"];

const FHIR_DEFINITIONS_JSON_DOWNLOAD_URL: &str = "http://hl7.org/fhir/{}/definitions.json.zip";

const MODEL_OUTPUT_DIRECTORY: &str = "fhirbolt-model/src/generated";
const SERDE_OUTPUT_DIRECTORY: &str = "fhirbolt-serde/src/model/generated";
const TYPE_HINTS_OUTPUT_DIRECTORY: &str = "fhirbolt-shared/src/serde_helpers/type_hints/generated";

fn tmp_dir(fhir_release: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("target")
        .join("tmp")
        .join("fhirbolt")
        .join(format!("{}", fhir_release.to_lowercase()));

    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }
    dir
}

fn fhir_definitions_json_zip(fhir_release: &str) -> PathBuf {
    tmp_dir(fhir_release).join("definitions.json.zip")
}

fn fhir_definitions_json_download_url(fhir_release: &str) -> String {
    str::replace(
        FHIR_DEFINITIONS_JSON_DOWNLOAD_URL,
        "{}",
        &fhir_release.to_lowercase(),
    )
}

fn download_fhir_definitions(fhir_release: &str) -> PathBuf {
    let definitions_json_zip = fhir_definitions_json_zip(fhir_release);
    let definitions_json_download_url = fhir_definitions_json_download_url(fhir_release);

    if definitions_json_zip.exists() {
        println!(
            "Download skipped! {} FHIR definitions already present.",
            fhir_release
        );
    } else {
        println!("Downloading {} FHIR definitions...", fhir_release);
        let bytes = reqwest::blocking::get(definitions_json_download_url)
            .and_then(|r| r.bytes())
            .expect(&format!("Error downloading {} definitions", fhir_release));
        fs::write(&definitions_json_zip, bytes)
            .expect(&format!("Error writing \"{:?}\"", definitions_json_zip));
        println!("Download successful!");
    }

    definitions_json_zip
}

fn open_zip_archive(zip_path: &Path) -> ZipArchive<File> {
    println!("Opening {:?}...", zip_path);
    let zip_file = File::open(zip_path).expect("Error opening zip file");
    ZipArchive::new(zip_file).unwrap()
}

fn read_file_from_zip_archive(zip_archive: &mut ZipArchive<File>, name: &str) -> String {
    println!("Reading '{}' from zip archive...", name);

    let name = if zip_archive.file_names().find(|n| *n == name).is_some() {
        name.to_owned()
    } else {
        format!("definitions.json/{}", name)
    };

    let mut file_content = String::new();
    <ZipFile as Read>::read_to_string(&mut zip_archive.by_name(&name).unwrap(), &mut file_content)
        .unwrap();
    file_content
}

fn read_types_from_zip_archive(zip_archive: &mut ZipArchive<File>) -> String {
    read_file_from_zip_archive(zip_archive, "profiles-types.json")
}

fn read_resources_from_zip_archive(zip_archive: &mut ZipArchive<File>) -> String {
    read_file_from_zip_archive(zip_archive, "profiles-resources.json")
}

fn parse_bundle(json: &str) -> Bundle {
    println!("Parsing bundle...");
    Bundle::from_json_str(json).unwrap()
}

fn generate_dir(dir: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(dir);

    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }
    dir
}

fn clear_generated() {
    fs::remove_dir_all(generate_dir(MODEL_OUTPUT_DIRECTORY)).unwrap();
    fs::remove_dir_all(generate_dir(SERDE_OUTPUT_DIRECTORY)).unwrap();
    fs::remove_dir_all(generate_dir(TYPE_HINTS_OUTPUT_DIRECTORY)).unwrap();
}

fn output_release_dir(target: &str, fhir_release: &str) -> PathBuf {
    let dir = generate_dir(target).join(fhir_release.to_lowercase());
    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    dir
}

fn output_source_dir(target: &str, fhir_release: &str, kind: &str) -> PathBuf {
    let dir = output_release_dir(target, fhir_release).join(kind);
    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    dir
}

fn output_source_file(target: &str, fhir_release: &str, kind: &str, name: &str) -> PathBuf {
    output_source_dir(target, fhir_release, kind).join(format!("{}.rs", name))
}

fn write_source_files(target: &str, fhir_release: &str, kind: &str, source_files: &[SourceFile]) {
    println!("Writing {}...", kind);

    let mut mod_names = vec![];
    for SourceFile { name, source } in source_files {
        write_source_file(target, fhir_release, kind, &name, source);
        mod_names.push(name.as_str());
    }
    write_source_mod_file(target, fhir_release, kind, &mod_names);
}

fn write_source_file(
    target: &str,
    fhir_release: &str,
    kind: &str,
    name: &str,
    tokens: &TokenStream,
) {
    let file_path = output_source_file(target, fhir_release, kind, name);
    let mut file =
        File::create(&file_path).expect(&format!("Error creating output file '{:?}'", file_path));

    write!(
        file,
        "// Generated on {} by fhirbolt-codegen v{}\n",
        Utc::today().naive_utc(),
        env!("CARGO_PKG_VERSION")
    )
    .unwrap();
    write!(file, "{}", tokens).unwrap();
}

fn write_root_mod_file(path: &str) {
    let mut file = File::create(&generate_dir(path).join("mod.rs")).unwrap();

    for fhir_release in BUILD_FHIR_RELEASES {
        writeln!(
            file,
            "#[cfg(feature = \"{}\")]",
            fhir_release.to_lowercase()
        )
        .unwrap();
        writeln!(file, "pub mod {};", fhir_release.to_lowercase()).unwrap();
    }
}

fn write_type_hints_root_mod_file(path: &str) {
    let mut file = File::create(&generate_dir(path).join("mod.rs")).unwrap();

    for fhir_release in BUILD_FHIR_RELEASES {
        writeln!(file, "pub mod {};", fhir_release.to_lowercase()).unwrap();
    }
}

fn write_model_release_mod_file(fhir_release: &str) {
    let mut file =
        File::create(&output_release_dir(MODEL_OUTPUT_DIRECTORY, fhir_release).join("mod.rs"))
            .unwrap();

    writeln!(file, "pub mod types;").unwrap();
    writeln!(file, "pub mod resources;").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "mod resource;").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "pub use resource::*;").unwrap();
    // TODO: remove after migrating serialization to fhirbolt-serde
    writeln!(file, "mod serde_helpers;").unwrap();
}

fn write_serde_release_mod_file(fhir_release: &str) {
    let mut file =
        File::create(&output_release_dir(SERDE_OUTPUT_DIRECTORY, fhir_release).join("mod.rs"))
            .unwrap();

    writeln!(file, "mod types;").unwrap();
    writeln!(file, "mod resources;").unwrap();
    writeln!(file, "mod resource;").unwrap();
    writeln!(file, "mod serde_helpers;").unwrap();
}

fn write_source_mod_file(target: &str, fhir_release: &str, kind: &str, types: &[&str]) {
    let mut file =
        File::create(&output_source_dir(target, fhir_release, kind).join("mod.rs")).unwrap();

    for r#type in types {
        let mod_name = r#type;
        write!(file, "mod {};\n", mod_name).unwrap();
    }

    for r#type in types {
        let mod_name = r#type;
        write!(file, "pub use {}::*;\n", mod_name).unwrap();
    }
}

fn type_hints_output_source_file(fhir_release: &str) -> PathBuf {
    generate_dir(TYPE_HINTS_OUTPUT_DIRECTORY).join(format!("{}.rs", fhir_release.to_lowercase()))
}

fn write_type_hints_source_file(fhir_release: &str, tokens: &TokenStream) {
    let file_path = type_hints_output_source_file(fhir_release);
    let mut file =
        File::create(&file_path).expect(&format!("Error creating output file '{:?}'", file_path));

    write!(
        file,
        "// Generated on {} by fhirbolt-codegen v{}\n",
        Utc::today().naive_utc(),
        env!("CARGO_PKG_VERSION")
    )
    .unwrap();
    write!(file, "{}", tokens).unwrap();
}

fn generate_and_write(fhir_release: &str, types_bundle: &Bundle, resources_bundle: &Bundle) {
    let generated = generate_all(types_bundle, resources_bundle, fhir_release);

    write_source_files(
        MODEL_OUTPUT_DIRECTORY,
        fhir_release,
        "types",
        &generated.types_struct_source_files,
    );
    write_source_files(
        MODEL_OUTPUT_DIRECTORY,
        fhir_release,
        "resources",
        &generated.resources_struct_source_files,
    );
    write_source_file(
        MODEL_OUTPUT_DIRECTORY,
        fhir_release,
        ".",
        &generated.resource_enum_source_file.name,
        &generated.resource_enum_source_file.source,
    );

    write_source_files(
        SERDE_OUTPUT_DIRECTORY,
        fhir_release,
        "types",
        &generated.types_serde_source_files,
    );
    write_source_files(
        SERDE_OUTPUT_DIRECTORY,
        fhir_release,
        "resources",
        &generated.resources_serde_source_files,
    );

    write_source_file(
        SERDE_OUTPUT_DIRECTORY,
        fhir_release,
        ".",
        &generated.resource_enum_serde_source_file.name,
        &generated.resource_enum_serde_source_file.source,
    );

    write_source_file(
        SERDE_OUTPUT_DIRECTORY,
        fhir_release,
        ".",
        &generated.serde_helpers.name,
        &generated.serde_helpers.source,
    );

    write_type_hints_source_file(fhir_release, &generated.type_hints);

    // TODO: remove after migrating serialization to fhirbolt-serde
    write_source_file(
        MODEL_OUTPUT_DIRECTORY,
        fhir_release,
        ".",
        &generated.serde_helpers.name,
        &generated.serde_helpers.source,
    );
}

fn rustfmt() {
    println!("Running 'cargo fmt'...");
    let _ = process::Command::new("cargo")
        .args(["fmt"])
        .output()
        .unwrap();
}

fn main() {
    clear_generated();

    write_root_mod_file(MODEL_OUTPUT_DIRECTORY);
    write_root_mod_file(SERDE_OUTPUT_DIRECTORY);
    write_type_hints_root_mod_file(TYPE_HINTS_OUTPUT_DIRECTORY);

    for fhir_release in BUILD_FHIR_RELEASES {
        println!("Generating FHIR {}...", fhir_release);
        write_model_release_mod_file(*fhir_release);
        write_serde_release_mod_file(*fhir_release);

        let zip_file = download_fhir_definitions(*fhir_release);
        let mut zip_archive = open_zip_archive(&zip_file);

        let types_json = read_types_from_zip_archive(&mut zip_archive);
        let types_bundle = parse_bundle(&types_json);

        let resources_json = read_resources_from_zip_archive(&mut zip_archive);
        let resources_bundle = parse_bundle(&resources_json);

        generate_and_write(*fhir_release, &types_bundle, &resources_bundle);

        println!("FHIR {} generated succesfully!", fhir_release);
    }

    rustfmt();

    // TODO: remove after migrating Serialize impls to fhirbolt-serde
    {
        for fhir_release in BUILD_FHIR_RELEASES {
            use std::io::{BufRead, BufReader};

            // model
            let path = PathBuf::from(MODEL_OUTPUT_DIRECTORY)
                .join(fhir_release)
                .join("serde_helpers.rs");

            let lines = {
                let file = File::open(&path).unwrap();
                BufReader::new(file)
                    .lines()
                    .take(8)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            fs::remove_file(&path).unwrap();

            let mut file = File::create(path).unwrap();
            file.write_all(lines.as_bytes()).unwrap();

            // serde
            let path = PathBuf::from(SERDE_OUTPUT_DIRECTORY)
                .join(fhir_release)
                .join("serde_helpers.rs");

            let lines = {
                let file = File::open(&path).unwrap();
                BufReader::new(file)
                    .lines()
                    .skip(8)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            fs::remove_file(&path).unwrap();

            let mut file = File::create(path).unwrap();
            file.write_all(lines.as_bytes()).unwrap();
        }
    }
}
