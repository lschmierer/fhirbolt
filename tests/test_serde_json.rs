use std::io::Read;

use assert_json_diff::assert_json_eq;
use serde_json::Value;

use fhirbolt::{
    element::Element,
    serde::{
        DeserializationConfig, DeserializationMode, DeserializeResourceOwned, SerializeResource,
    },
    FhirRelease, FhirReleases,
};

use test_utils::examples::{examples, JsonOrXml};

const MISSING_STATUS_FILES: &[&str] = &[
    &"examples-json/codesystem-catalogType.json",
    &"examples-json/valueset-catalogType.json",
    &"examples-json/valuesets.json",
];

fn test_serde_json<'a, T, const R: FhirRelease>(mode: DeserializationMode)
where
    T: DeserializeResourceOwned + SerializeResource,
{
    let mut examples_iter = examples(R, JsonOrXml::Json);

    let mut buffer = Vec::new();

    while let Some(mut file) = examples_iter.next() {
        // not a FHIR resource
        if file.name().ends_with("package-min-ver.json") {
            continue;
        }

        match R {
            FhirReleases::R4 => {
                if mode != DeserializationMode::Lax {
                    // all questionnaires seem to have missing linkIds
                    if file.name().ends_with("-questionnaire.json") {
                        continue;
                    }
                }
            }
            FhirReleases::R4B => {
                // R4B examples contain some rubbish
                if file.name().starts_with("__MACOSX/") {
                    continue;
                }

                if mode != DeserializationMode::Lax {
                    // missing status
                    if MISSING_STATUS_FILES.contains(&file.name()) {
                        continue;
                    }
                }
            }
            _ => (),
        };

        println!("{}", file.name());

        buffer.clear();
        file.read_to_end(&mut buffer).unwrap();

        let mut json_value: Value = serde_json::from_slice(&buffer).unwrap();

        // contains null value in primitive array, while fhirbolt accepts it, it does not replicate this
        if R == FhirReleases::R4B {
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

        let element_from_slice: Element<R> = fhirbolt::json::from_slice(&buffer, None).unwrap();

        assert_json_eq!(
            fhirbolt::json::to_json_value(element_from_slice.clone(), None).unwrap(),
            json_value
        );

        let element_from_value: Element<R> =
            fhirbolt::json::from_json_value(json_value.clone(), None).unwrap();

        assert_json_eq!(
            fhirbolt::json::to_json_value(element_from_value, None).unwrap(),
            json_value
        );

        let resource: T =
            fhirbolt::json::from_slice(&buffer, Some(DeserializationConfig { mode })).unwrap();
        assert_json_eq!(
            fhirbolt::json::to_json_value(resource.clone(), None).unwrap(),
            json_value
        );

        assert_eq!(
            fhirbolt::element::from_element::<R, T>(
                element_from_slice.clone(),
                Some(DeserializationConfig { mode })
            )
            .unwrap(),
            resource
        );

        if !(
            // all questionnaires seem to have missing linkIds
            R == FhirReleases::R4 && file.name().ends_with("-questionnaire.json")
            ||
            // missing status
            R == FhirReleases::R4B && MISSING_STATUS_FILES.contains(&file.name())
        ) {
            assert_eq!(
                fhirbolt::element::to_element::<R, T>(resource).unwrap(),
                element_from_slice
            );
        }
    }
}

#[cfg(feature = "r4")]
#[test]
#[cfg_attr(not(feature = "r4"), ignore)]
fn test_serde_json_r4() {
    test_serde_json::<fhirbolt::model::r4::Resource, { FhirReleases::R4 }>(
        DeserializationMode::Strict,
    );
    test_serde_json::<fhirbolt::model::r4::Resource, { FhirReleases::R4 }>(
        DeserializationMode::Compatibility,
    );
    test_serde_json::<fhirbolt::model::r4::Resource, { FhirReleases::R4 }>(
        DeserializationMode::Lax,
    );
}

#[cfg(feature = "r4b")]
#[test]
fn test_serde_json_r4b() {
    test_serde_json::<fhirbolt::model::r4b::Resource, { FhirReleases::R4B }>(
        DeserializationMode::Strict,
    );
    test_serde_json::<fhirbolt::model::r4b::Resource, { FhirReleases::R4B }>(
        DeserializationMode::Compatibility,
    );
    test_serde_json::<fhirbolt::model::r4b::Resource, { FhirReleases::R4B }>(
        DeserializationMode::Lax,
    );
}

#[cfg(feature = "r5")]
#[test]
fn test_serde_json_r5() {
    test_serde_json::<fhirbolt::model::r5::Resource, { FhirReleases::R5 }>(
        DeserializationMode::Strict,
    );
    test_serde_json::<fhirbolt::model::r5::Resource, { FhirReleases::R5 }>(
        DeserializationMode::Compatibility,
    );
    test_serde_json::<fhirbolt::model::r5::Resource, { FhirReleases::R5 }>(
        DeserializationMode::Lax,
    );
}
