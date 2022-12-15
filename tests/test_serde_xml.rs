use std::io::Read;

use serde::de::DeserializeOwned;
use serde::Serialize;

use fhirbolt::{
    model::{self, AnyResource, FhirRelease},
    serde::{DeserializationConfig, DeserializationMode},
};

use test_utils::{
    assert_xml_eq,
    examples::{examples, JsonOrXml},
};

fn test_serde_xml<R: Serialize + DeserializeOwned + AnyResource>(mode: DeserializationMode) {
    let mut examples_iter = examples(R::fhir_release(), JsonOrXml::Xml);

    let mut buffer = Vec::new();

    while let Some(mut file) = examples_iter.next() {
        match R::fhir_release() {
            FhirRelease::R4 => (),
            FhirRelease::R4B => {
                if mode != DeserializationMode::Lax {
                    // missing status
                    if file.name() == "valuesets.xml" {
                        continue;
                    }
                }
            }
        };

        println!("{}", file.name());

        buffer.clear();
        file.read_to_end(&mut buffer).unwrap();

        let resource: R =
            fhirbolt::xml::from_slice(&buffer[..], Some(DeserializationConfig { mode })).unwrap();

        assert_xml_eq(
            &fhirbolt::xml::to_vec(&resource).unwrap(),
            &buffer,
            R::fhir_release() == FhirRelease::R4B && file.name() == "valuesets.xml",
        );
    }
}

#[test]
fn test_serde_xml_r4() {
    test_serde_xml::<model::r4::Resource>(DeserializationMode::Strict);
    test_serde_xml::<model::r4::Resource>(DeserializationMode::Compatibility);
    test_serde_xml::<model::r4::Resource>(DeserializationMode::Lax);
}

#[test]
fn test_serde_xml_r4b() {
    test_serde_xml::<model::r4b::Resource>(DeserializationMode::Strict);
    test_serde_xml::<model::r4b::Resource>(DeserializationMode::Compatibility);
    test_serde_xml::<model::r4b::Resource>(DeserializationMode::Lax);
}
