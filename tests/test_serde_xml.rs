use std::io::Read;

use fhirbolt::{
    element::Element,
    serde::{
        DeserializationConfig, DeserializationMode, DeserializeResourceOwned, SerializeResource,
    },
    FhirRelease, FhirReleases,
};

use test_utils::{
    assert_xml_eq,
    examples::{examples, JsonOrXml},
    shuffle,
};

fn test_serde_xml<'a, T, const R: FhirRelease>(mode: DeserializationMode)
where
    T: DeserializeResourceOwned + SerializeResource,
{
    let mut examples_iter = examples(R, JsonOrXml::Xml);

    let mut buffer = Vec::new();

    while let Some(mut file) = examples_iter.next() {
        match R {
            FhirReleases::R4B => {
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

        let element: Element<R> =
            shuffle::shuffle_element(fhirbolt::xml::from_slice(&buffer, None).unwrap());

        let mut element_buffer = Vec::new();
        _ = fhirbolt::xml::to_writer(&mut element_buffer, &element, None).unwrap();

        assert_xml_eq(
            &element_buffer,
            &buffer,
            R == FhirReleases::R4B && file.name() == "valuesets.xml",
        );

        let resource: T =
            fhirbolt::xml::from_slice(&buffer[..], Some(DeserializationConfig { mode })).unwrap();

        assert_xml_eq(
            &fhirbolt::xml::to_vec(&resource, None).unwrap(),
            &buffer,
            R == FhirReleases::R4B && file.name() == "valuesets.xml",
        );

        assert_eq!(
            fhirbolt::element::from_element::<R, T>(
                element.clone(),
                Some(DeserializationConfig { mode })
            )
            .unwrap(),
            resource
        );

        if !(
            // missing status
            R == FhirReleases::R4B && file.name() == "valuesets.xml"
        ) {
            assert_eq!(
                fhirbolt::element::to_element::<R, T>(resource).unwrap(),
                element
            );
        }
    }
}

#[cfg(feature = "r4")]
#[test]
fn test_serde_xml_r4() {
    test_serde_xml::<fhirbolt::model::r4::Resource, { FhirReleases::R4 }>(
        DeserializationMode::Strict,
    );
    test_serde_xml::<fhirbolt::model::r4::Resource, { FhirReleases::R4 }>(
        DeserializationMode::Compatibility,
    );
    test_serde_xml::<fhirbolt::model::r4::Resource, { FhirReleases::R4 }>(DeserializationMode::Lax);
}

#[cfg(feature = "r4b")]
#[test]
fn test_serde_xml_r4b() {
    test_serde_xml::<fhirbolt::model::r4b::Resource, { FhirReleases::R4B }>(
        DeserializationMode::Strict,
    );
    test_serde_xml::<fhirbolt::model::r4b::Resource, { FhirReleases::R4B }>(
        DeserializationMode::Compatibility,
    );
    test_serde_xml::<fhirbolt::model::r4b::Resource, { FhirReleases::R4B }>(
        DeserializationMode::Lax,
    );
}

#[cfg(feature = "r5")]
#[test]
fn test_serde_xml_r5() {
    test_serde_xml::<fhirbolt::model::r5::Resource, { FhirReleases::R5 }>(
        DeserializationMode::Strict,
    );
    test_serde_xml::<fhirbolt::model::r5::Resource, { FhirReleases::R5 }>(
        DeserializationMode::Compatibility,
    );
    test_serde_xml::<fhirbolt::model::r5::Resource, { FhirReleases::R5 }>(DeserializationMode::Lax);
}
