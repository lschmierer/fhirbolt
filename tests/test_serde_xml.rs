use std::io::Read;

use serde::de::DeserializeOwned;
use serde::Serialize;

use fhirbolt::{
    element::Element,
    model::AnyResource,
    serde::{DeserializationConfig, DeserializationMode},
    FhirRelease,
};

use test_utils::{
    assert_xml_eq,
    examples::{examples, JsonOrXml},
};

fn test_serde_xml<E: Serialize + DeserializeOwned + AnyResource, const R: FhirRelease>(
    mode: DeserializationMode,
) {
    let mut examples_iter = examples(R, JsonOrXml::Xml);

    let mut buffer = Vec::new();

    while let Some(mut file) = examples_iter.next() {
        match R {
            FhirRelease::R4B => {
                if mode != DeserializationMode::Lax {
                    // missing status
                    if file.name() == "valuesets.xml" {
                        continue;
                    }
                }
            }
            _ => (),
        };

        println!("{}", file.name());

        buffer.clear();
        file.read_to_end(&mut buffer).unwrap();

        let element: Element<R> = fhirbolt::element::xml::from_slice(&buffer).unwrap();

        let mut element_buffer = Vec::new();
        _ = fhirbolt::element::xml::to_writer(&mut element_buffer, &element).unwrap();

        assert_xml_eq(
            &element_buffer,
            &buffer,
            R == FhirRelease::R4B && file.name() == "valuesets.xml",
        );

        let resource: E =
            fhirbolt::model::xml::from_slice(&buffer[..], Some(DeserializationConfig { mode }))
                .unwrap();

        assert_xml_eq(
            &fhirbolt::model::xml::to_vec(&resource).unwrap(),
            &buffer,
            R == FhirRelease::R4B && file.name() == "valuesets.xml",
        );
    }
}

#[test]
fn test_serde_xml_r4() {
    test_serde_xml::<fhirbolt::model::r4::Resource, { FhirRelease::R4 }>(
        DeserializationMode::Strict,
    );
    test_serde_xml::<fhirbolt::model::r4::Resource, { FhirRelease::R4 }>(
        DeserializationMode::Compatibility,
    );
    test_serde_xml::<fhirbolt::model::r4::Resource, { FhirRelease::R4 }>(DeserializationMode::Lax);
}

#[test]
fn test_serde_xml_r4b() {
    test_serde_xml::<fhirbolt::model::r4b::Resource, { FhirRelease::R4B }>(
        DeserializationMode::Strict,
    );
    test_serde_xml::<fhirbolt::model::r4b::Resource, { FhirRelease::R4B }>(
        DeserializationMode::Compatibility,
    );
    test_serde_xml::<fhirbolt::model::r4b::Resource, { FhirRelease::R4B }>(
        DeserializationMode::Lax,
    );
}
