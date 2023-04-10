#![feature(adt_const_params)]

use std::{fmt, io::Read};

use serde::{de::DeserializeSeed, Serialize};

use fhirbolt::{
    element::Element,
    model::AnyResource,
    serde::{DeserializationConfig, DeserializationContext, DeserializationMode},
    FhirRelease,
};

use test_utils::{
    assert_xml_eq,
    examples::{examples, JsonOrXml},
};

fn test_serde_xml<'a, T, const R: FhirRelease>(mode: DeserializationMode)
where
    T: AnyResource + Serialize + PartialEq + fmt::Debug + Clone,
    for<'c, 'de> &'c mut DeserializationContext<T>: DeserializeSeed<'de, Value = T>,
{
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

        let element = fhirbolt::serde::xml::from_slice::<Element<R>>(&buffer, None).unwrap();

        let mut element_buffer = Vec::new();
        _ = fhirbolt::serde::element::xml::to_writer(&mut element_buffer, &element).unwrap();

        assert_xml_eq(
            &element_buffer,
            &buffer,
            R == FhirRelease::R4B && file.name() == "valuesets.xml",
        );

        let resource = fhirbolt::serde::xml::from_slice::<T>(
            &buffer[..],
            Some(DeserializationConfig { mode }),
        )
        .unwrap();

        assert_xml_eq(
            &fhirbolt::serde::model::xml::to_vec(&resource).unwrap(),
            &buffer,
            R == FhirRelease::R4B && file.name() == "valuesets.xml",
        );

        assert_eq!(
            fhirbolt::serde::element::from_element::<R, T>(
                element.clone(),
                Some(DeserializationConfig { mode })
            )
            .unwrap(),
            resource
        );

        if !(
            // missing status
            R == FhirRelease::R4B && file.name() == "valuesets.xml"
        ) {
            assert_eq!(
                fhirbolt::serde::element::to_element::<R, T>(resource).unwrap(),
                element
            );
        }
    }
}

#[cfg(feature = "r4")]
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

#[cfg(feature = "r4b")]
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
