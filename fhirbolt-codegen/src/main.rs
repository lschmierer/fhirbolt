use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::str;

use chrono::Utc;
use proc_macro2::TokenStream;
use zip::read::{ZipArchive, ZipFile};

use fhirbolt_codegen::{generate, model::Bundle, SourceFile};

enum FhirRelease {
    R4,
}

impl fmt::Display for FhirRelease {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FhirRelease::R4 => write!(f, "R4"),
        }
    }
}

const BUILD_FHIR_RELEASES: &[FhirRelease] = &[FhirRelease::R4];

const FHIR_DEFINITIONS_JSON_DOWNLOAD_URL: &str = "http://hl7.org/fhir/{}/definitions.json.zip";

fn tmp_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("target")
        .join("fhirbolt-codegen");

    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    dir
}

fn fhir_definitions_json_zip(fhir_release: &FhirRelease) -> PathBuf {
    tmp_dir().join(format!("{}.zip", fhir_release.to_string().to_lowercase()))
}

fn fhir_definitions_json_download_url(fhir_release: &FhirRelease) -> String {
    str::replace(
        FHIR_DEFINITIONS_JSON_DOWNLOAD_URL,
        "{}",
        &fhir_release.to_string().to_lowercase(),
    )
}

fn download_fhir_definitions(fhir_release: &FhirRelease) -> PathBuf {
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
    let zip_file = fs::File::open(zip_path).expect("Error opening zip file");
    ZipArchive::new(zip_file).unwrap()
}

fn read_file_from_zip_archive(zip_archive: &mut ZipArchive<File>, name: &str) -> String {
    println!("Reading '{}' from zip archive...", name);
    let mut file_content = String::new();
    <ZipFile as Read>::read_to_string(&mut zip_archive.by_name(name).unwrap(), &mut file_content)
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

fn generate_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("fhirbolt")
        .join("src")
        .join("generated");

    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    dir
}

fn clear_generated() {
    fs::remove_dir_all(generate_dir()).unwrap();
}

fn output_release_dir(fhir_release: &FhirRelease) -> PathBuf {
    let dir = generate_dir().join(fhir_release.to_string().to_lowercase());
    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    dir
}

fn output_source_dir(fhir_release: &FhirRelease, kind: &str) -> PathBuf {
    let dir = output_release_dir(fhir_release).join(kind);
    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    dir
}

fn output_source_file(fhir_release: &FhirRelease, kind: &str, name: &str) -> PathBuf {
    output_source_dir(fhir_release, kind).join(format!("{}.rs", name))
}

fn write_source_file(fhir_release: &FhirRelease, kind: &str, name: &str, tokens: TokenStream) {
    let file_path = output_source_file(fhir_release, kind, name);
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

fn write_root_mod_file() {
    let mut file = File::create(&generate_dir().join("mod.rs")).unwrap();

    for fhir_release in BUILD_FHIR_RELEASES {
        write!(file, "pub mod {};", fhir_release.to_string().to_lowercase()).unwrap();
    }
}

fn write_release_mod_file(fhir_release: &FhirRelease) {
    let mut file = File::create(&output_release_dir(fhir_release).join("mod.rs")).unwrap();

    write!(file, "pub mod types;\n").unwrap();
    write!(file, "pub mod resources;\n").unwrap();
}

fn write_source_mod_file(fhir_release: &FhirRelease, kind: &str, types: &Vec<String>) {
    let mut file = File::create(&output_source_dir(fhir_release, kind).join("mod.rs")).unwrap();

    for r#type in types {
        let mod_name = r#type;
        write!(file, "mod {};\n", mod_name).unwrap();
    }

    for r#type in types {
        let mod_name = r#type;
        write!(file, "pub use {}::*;\n", mod_name).unwrap();
    }
}

fn generate_and_write_bundle(fhir_release: &FhirRelease, bundle: &Bundle, kind: &str) {
    println!("Writing {}...", kind);
    let generated = generate(&bundle);
    let mut mod_names = vec![];

    for SourceFile { name, source } in generated {
        write_source_file(fhir_release, kind, &name, source);
        mod_names.push(name);
    }
    write_source_mod_file(&fhir_release, kind, &mod_names);
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

    write_root_mod_file();

    for fhir_release in BUILD_FHIR_RELEASES {
        println!("Generating FHIR {}...", fhir_release);
        write_release_mod_file(&fhir_release);

        let zip_file = download_fhir_definitions(fhir_release);
        let mut zip_archive = open_zip_archive(&zip_file);

        let types_json = read_types_from_zip_archive(&mut zip_archive);
        let types_bundle = parse_bundle(&types_json);
        generate_and_write_bundle(fhir_release, &types_bundle, "types");

        let resources_json = read_resources_from_zip_archive(&mut zip_archive);
        let resources_bundle = parse_bundle(&resources_json);
        generate_and_write_bundle(fhir_release, &resources_bundle, "resources");
        println!("FHIR {} generated succesfull!", fhir_release);
    }

    rustfmt()
}
