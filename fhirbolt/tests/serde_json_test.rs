use std::fs;
use std::path;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use zip::ZipArchive;

const FHIR_EXAMPLES_JSON_DOWNLOAD_URL: &str = "http://hl7.org/fhir/{}/examples-json.zip";

fn fhir_examples_json_download_url(fhir_release: &str) -> String {
    str::replace(
        FHIR_EXAMPLES_JSON_DOWNLOAD_URL,
        "{}",
        &fhir_release.to_lowercase(),
    )
}

fn download_fhir_examples_json(fhir_release: &str) -> path::PathBuf {
    let examples_json_download_url = fhir_examples_json_download_url(fhir_release);
    let examples_json_folder_path =
        path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(fhir_release.to_lowercase());
    let examples_json_zip_path = examples_json_folder_path.join("examples-json.zip");

    if !examples_json_zip_path.exists() {
        fs::create_dir_all(examples_json_folder_path).unwrap();

        let bytes = reqwest::blocking::get(examples_json_download_url)
            .and_then(|r| r.bytes())
            .expect(&format!("Error downloading {} examples", fhir_release));
        fs::write(&examples_json_zip_path, bytes)
            .expect(&format!("Error writing \"{:?}\"", examples_json_zip_path));

        println!(" Download complete!");
    }

    examples_json_zip_path
}

fn test_serde_json<R: Serialize + DeserializeOwned>(fhir_release: &str) {
    let examples_json_zip_path = download_fhir_examples_json(fhir_release);

    let zip_file = fs::File::open(examples_json_zip_path).expect("Error opening zip file");
    let mut archive = ZipArchive::new(zip_file).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();

        // all questionnaires seem to have missing linkIds
        if file.name().ends_with("-questionnaire.json") {
            continue;
        }

        // not a FHIR resource
        if file.name() == "package-min-ver.json" {
            continue;
        }

        println!("{}", file.name());

        let json_value: Value = serde_json::from_reader(file).unwrap();
        let resource: R = serde_json::from_value(json_value.clone()).unwrap();
        assert_eq!(serde_json::to_value(resource).unwrap(), json_value);

        //let mut s = String::new();

        //archive.by_index(i).unwrap().read_to_string(&mut s).unwrap();

        //let bundle: R = serde_json::from_str(&s).unwrap();
    }
}

#[test]
fn test_serde_json_r4() {
    test_serde_json::<fhirbolt::r4::resources::Resource>("R4")
}
