#![feature(adt_const_params)]

use std::{fmt, io::Read};

use assert_json_diff::assert_json_eq;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use fhirbolt::{
    element::Element,
    model::AnyResource,
    serde::{DeserializationConfig, DeserializationMode},
    FhirRelease,
};

use test_utils::examples::{examples, JsonOrXml};

fn test_serde_json<
    E: AnyResource + Serialize + DeserializeOwned + PartialEq + fmt::Debug + Clone,
    const R: FhirRelease,
>(
    mode: DeserializationMode,
) {
    let mut examples_iter = examples(R, JsonOrXml::Json);

    let mut buffer = Vec::new();

    while let Some(mut file) = examples_iter.next() {
        // not a FHIR resource
        if file.name().ends_with("package-min-ver.json") {
            continue;
        }

        match R {
            FhirRelease::R4 => {
                if mode != DeserializationMode::Lax {
                    // all questionnaires seem to have missing linkIds
                    if file.name().ends_with("-questionnaire.json") {
                        continue;
                    }
                }
            }
            FhirRelease::R4B => {
                // R4B examples contain some rubbish
                if file.name().starts_with("__MACOSX/") {
                    continue;
                }

                if mode != DeserializationMode::Lax {
                    // missing status
                    if [
                        "examples-json/codesystem-catalogType.json",
                        "examples-json/valueset-catalogType.json",
                        "examples-json/valuesets.json",
                    ]
                    .contains(&file.name())
                    {
                        continue;
                    }
                }
            }
        };

        println!("{}", file.name());

        buffer.clear();
        file.read_to_end(&mut buffer).unwrap();

        let mut json_value: Value = serde_json::from_slice(&buffer).unwrap();

        // contains null value in primitive array, while fhirbolt accepts it, it does not replicate this
        if R == FhirRelease::R4B {
            if file.name().starts_with("examples-json/activitydefinition-") {
                if let Some(timing) = json_value
                    .as_object_mut()
                    .unwrap()
                    .get_mut("timingTiming")
                    .and_then(|t| t.as_object_mut())
                {
                    timing.remove("event");
                }
            }
            if file.name().starts_with("examples-json/plandefinition-") {
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

        let element_from_slice: Element<R> = fhirbolt::element::json::from_slice(&buffer).unwrap();

        assert_json_eq!(
            fhirbolt::element::json::to_json_value(element_from_slice.clone()).unwrap(),
            json_value
        );

        let element_from_value: Element<R> =
            fhirbolt::element::json::from_json_value(json_value.clone()).unwrap();

        assert_json_eq!(
            fhirbolt::element::json::to_json_value(element_from_value).unwrap(),
            json_value
        );

        let resource: E =
            fhirbolt::model::json::from_slice(&buffer, Some(DeserializationConfig { mode }))
                .unwrap();
        assert_json_eq!(
            fhirbolt::model::json::to_json_value(resource.clone()).unwrap(),
            json_value
        );

        assert_eq!(
            fhirbolt::model::from_element::<R, E>(
                element_from_slice,
                Some(DeserializationConfig { mode })
            )
            .unwrap(),
            resource
        );
    }
}

#[test]
fn test_serde_json_r4() {
    test_serde_json::<fhirbolt::model::r4::Resource, { FhirRelease::R4 }>(
        DeserializationMode::Strict,
    );
    test_serde_json::<fhirbolt::model::r4::Resource, { FhirRelease::R4 }>(
        DeserializationMode::Compatibility,
    );
    test_serde_json::<fhirbolt::model::r4::Resource, { FhirRelease::R4 }>(DeserializationMode::Lax);
}

#[test]
fn test_serde_json_r4b() {
    test_serde_json::<fhirbolt::model::r4b::Resource, { FhirRelease::R4B }>(
        DeserializationMode::Strict,
    );
    test_serde_json::<fhirbolt::model::r4b::Resource, { FhirRelease::R4B }>(
        DeserializationMode::Compatibility,
    );
    test_serde_json::<fhirbolt::model::r4b::Resource, { FhirRelease::R4B }>(
        DeserializationMode::Lax,
    );
}
