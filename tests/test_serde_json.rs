use assert_json_diff::assert_json_eq;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use fhirbolt::{
    model::{self, AnyResource, FhirRelease},
    DeserializationConfig, DeserializationMode,
};

use test_utils::examples::{examples, JsonOrXml};

fn test_serde_json<R: Serialize + DeserializeOwned + AnyResource>(mode: DeserializationMode) {
    let mut examples_iter = examples(R::fhir_release(), JsonOrXml::Json);

    while let Some(file) = examples_iter.next() {
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

        let resource: R = fhirbolt::json::from_json_value(
            json_value.clone(),
            Some(DeserializationConfig { mode }),
        )
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

        assert_json_eq!(fhirbolt::json::to_json_value(resource).unwrap(), json_value);
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
