use std::fs;
use std::io::Read;
use std::path;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use zip::ZipArchive;

use fhirbolt::{
    model::{self, AnyResource},
    serde::{DeserializationConfig, DeserializationMode},
    FhirRelease,
};

const FHIR_EXAMPLES_XML_DOWNLOAD_URL: &str = "http://hl7.org/fhir/{}/examples.zip";

fn fhir_examples_xml_download_url(fhir_release: FhirRelease) -> String {
    str::replace(
        FHIR_EXAMPLES_XML_DOWNLOAD_URL,
        "{}",
        &fhir_release.to_string().to_lowercase(),
    )
}

fn download_fhir_examples_xml(fhir_release: FhirRelease) -> path::PathBuf {
    let examples_xml_download_url = fhir_examples_xml_download_url(fhir_release);
    let examples_xml_folder_path = path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("fhirbolt")
        .join(fhir_release.to_string().to_lowercase());
    let examples_xml_zip_path = examples_xml_folder_path.join("examples.zip");

    if !examples_xml_zip_path.exists() {
        fs::create_dir_all(examples_xml_folder_path).unwrap();

        let bytes = reqwest::blocking::get(examples_xml_download_url)
            .and_then(|r| r.bytes())
            .expect(&format!("Error downloading {} examples", fhir_release));
        fs::write(&examples_xml_zip_path, bytes)
            .expect(&format!("Error writing \"{:?}\"", examples_xml_zip_path));

        println!(" Download complete!");
    }

    examples_xml_zip_path
}

fn test_serde_xml<R: Serialize + DeserializeOwned + AnyResource>(mode: DeserializationMode) {
    let examples_xml_zip_path = download_fhir_examples_xml(R::fhir_release());

    let zip_file = fs::File::open(examples_xml_zip_path).expect("Error opening zip file");
    let mut archive = ZipArchive::new(zip_file).unwrap();

    let mut buffer = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        println!("{}", file.name());

        buffer.clear();
        file.read_to_end(&mut buffer).unwrap();

        let mut deserializer = fhirbolt::xml::Deserializer::from_slice::<R>(&buffer[..]);
        let json_value = Value::deserialize(&mut deserializer).unwrap();

        let resource: R =
            fhirbolt::xml::from_slice(&buffer[..], Some(DeserializationConfig { mode })).unwrap();

        assert_eq!(fhirbolt::json::to_value(resource).unwrap(), json_value);
    }
}

#[test]
fn test_serde_xml_r4() {
    test_serde_xml::<model::r4::Resource>(DeserializationMode::Strict);
    test_serde_xml::<model::r4::Resource>(DeserializationMode::Compatibility);
    test_serde_xml::<model::r4::Resource>(DeserializationMode::Lax);
}
