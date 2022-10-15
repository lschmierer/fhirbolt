use std::fs;
use std::path;

use assert_json_diff::assert_json_eq;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use zip::ZipArchive;

use fhirbolt::{
    model::{self, AnyResource, FhirRelease},
    serde::{DeserializationConfig, DeserializationMode},
};

const FHIR_EXAMPLES_JSON_DOWNLOAD_URL: &str = "http://hl7.org/fhir/{}/examples-json.zip";

fn fhir_examples_json_download_url(fhir_release: FhirRelease) -> String {
    str::replace(
        FHIR_EXAMPLES_JSON_DOWNLOAD_URL,
        "{}",
        &fhir_release.to_string().to_lowercase(),
    )
}

fn download_fhir_examples_json(fhir_release: FhirRelease) -> path::PathBuf {
    let examples_json_download_url = fhir_examples_json_download_url(fhir_release);
    let examples_json_folder_path = path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("fhirbolt")
        .join(fhir_release.to_string().to_lowercase());
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

fn test_serde_json<R: Serialize + DeserializeOwned + AnyResource>(mode: DeserializationMode) {
    let examples_json_zip_path = download_fhir_examples_json(R::fhir_release());

    let zip_file = fs::File::open(examples_json_zip_path).expect("Error opening zip file");
    let mut archive = ZipArchive::new(zip_file).unwrap();

    let start = if R::fhir_release() == FhirRelease::R4B {
        1
    } else {
        0
    };

    for i in start..archive.len() {
        let file = archive.by_index(i).unwrap();
        let file_name = file.name().to_owned();

        // not a FHIR resource
        if file_name.ends_with("package-min-ver.json") {
            continue;
        }

        match R::fhir_release() {
            FhirRelease::R4 => {
                if mode != DeserializationMode::Lax {
                    // all questionnaires seem to have missing linkIds
                    if file_name.ends_with("-questionnaire.json") {
                        continue;
                    }
                }
            }
            FhirRelease::R4B => {
                // R4B examples contain some rubbish
                if file_name.starts_with("__MACOSX/") {
                    continue;
                }

                if mode != DeserializationMode::Lax {
                    // missing status
                    if [
                        "examples-json/codesystem-catalogType.json",
                        "examples-json/valueset-catalogType.json",
                        "examples-json/valuesets.json",
                    ]
                    .contains(&file_name.as_str())
                    {
                        continue;
                    }
                }
            }
        };

        println!("{}", file_name);

        let mut json_value: Value = serde_json::from_reader(file).unwrap();

        let resource: R =
            fhirbolt::json::from_value(json_value.clone(), Some(DeserializationConfig { mode }))
                .unwrap();

        // contains null value in primitive array, while fhirbolt accepts it, it does not replicate this
        if R::fhir_release() == FhirRelease::R4B {
            if file_name.starts_with("examples-json/activitydefinition-") {
                if let Some(timing) = json_value
                    .as_object_mut()
                    .unwrap()
                    .get_mut("timingTiming")
                    .and_then(|t| t.as_object_mut())
                {
                    timing.remove("event");
                }
            }
            if file_name.starts_with("examples-json/plandefinition-") {
                if let Some(timing) = json_value
                    .as_object_mut()
                    .unwrap()
                    .get_mut("contained")
                    .and_then(|c| c.as_array_mut())
                    .and_then(|a| {
                        if a.get(0)
                            .and_then(|v| v.as_object())
                            .map(|m| m.contains_key("timingTiming"))
                            .unwrap_or(false)
                        {
                            a.get_mut(0)
                        } else {
                            a.get_mut(1)
                        }
                    })
                    .and_then(|v| v.as_object_mut())
                    .and_then(|m| m.get_mut("timingTiming"))
                    .and_then(|t| t.as_object_mut())
                {
                    timing.remove("event");
                }
            }
        }

        assert_json_eq!(fhirbolt::json::to_value(resource).unwrap(), json_value);
    }
}

#[test]
fn test_serde_json_r4() {
    test_serde_json::<model::r4::Resource>(DeserializationMode::Strict);
    test_serde_json::<model::r4::Resource>(DeserializationMode::Compatibility);
    test_serde_json::<model::r4::Resource>(DeserializationMode::Lax);
}

#[test]
fn test_serde_json_r4b() {
    test_serde_json::<model::r4b::Resource>(DeserializationMode::Strict);
    test_serde_json::<model::r4b::Resource>(DeserializationMode::Compatibility);
    test_serde_json::<model::r4b::Resource>(DeserializationMode::Lax);
}
