//! Deserialize FHIR resources from XML.

use std::{
    io,
    mem::{self},
};

use serde::{
    de::{
        self,
        value::{StrDeserializer, StringDeserializer},
        DeserializeSeed, Visitor,
    },
    forward_to_deserialize_any,
};

use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::{
    context::Format,
    xml::{
        error::{Error, Result},
        event::{Element, Event},
        read::{IoRead, Read, SliceRead, StrRead},
    },
    DeserializationConfig, DeserializeResource, DeserializeResourceOwned,
};

fn from_deserializer<'de, R, T>(
    de: &mut Deserializer<R>,
    config: Option<DeserializationConfig>,
) -> Result<T>
where
    R: Read,
    T: DeserializeResource<'de>,
{
    T::deserialization_context(config.unwrap_or(Default::default()), Format::Xml).deserialize(de)
}

/// Deserialize an instance of resource type `T` directly from an IO stream of XML (e.g. coming from network).
///
/// # Example
/// ```
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource;
///
/// // The type of `s` is `&str`
/// let s = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
/// <Observation xmlns=\"http://hl7.org/fhir\">
///     <status value=\"final\"/>
///     <code>
///         <text value=\"some code\"/>
///     </code>
///     <valueString value=\"some value\"/>
/// </Observation>";
///
/// // `s.as_bytes()` returns `&[u8]` which implements `std::io::Read`
/// let r: Resource = fhirbolt::xml::from_reader(s.as_bytes(), None).unwrap();
/// println!("{:?}", r);
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_reader<R: io::Read, T>(rdr: R, config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeResourceOwned,
{
    from_deserializer(
        &mut Deserializer::from_reader(rdr, T::FHIR_RELEASE)?,
        config,
    )
}

/// Deserialize an instance of resource type `T` from a bytes of XML.
///
/// # Example
/// ```
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource;
///
/// // The type of `s` is `&[u8]`
/// let b = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
/// <Observation xmlns=\"http://hl7.org/fhir\">
///     <status value=\"final\"/>
///     <code>
///         <text value=\"some code\"/>
///     </code>
///     <valueString value=\"some value\"/>
/// </Observation>";
///
/// let r: Resource = fhirbolt::xml::from_slice(b, None).unwrap();
/// println!("{:?}", r);
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_slice<'a, T>(v: &'a [u8], config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeResource<'a>,
{
    from_deserializer(&mut Deserializer::from_slice(v, T::FHIR_RELEASE)?, config)
}

/// Deserialize an instance of resource type `T` from a string of XML.
///
/// # Example
/// ```
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource;
///
/// // The type of `s` is `&str`
/// let s = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
/// <Observation xmlns=\"http://hl7.org/fhir\">
///     <status value=\"final\"/>
///     <code>
///         <text value=\"some code\"/>
///     </code>
///     <valueString value=\"some value\"/>
/// </Observation>";
///
/// let r: Resource = fhirbolt::xml::from_str(s, None).unwrap();
/// println!("{:?}", r);
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_str<'a, T>(s: &'a str, config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeResource<'a>,
{
    from_deserializer(&mut Deserializer::from_str(s, T::FHIR_RELEASE)?, config)
}

pub struct Deserializer<R: Read> {
    reader: R,
    next_event: Event,
}

impl<R: io::Read> Deserializer<IoRead<R>> {
    pub fn from_reader(reader: R, fhir_release: FhirRelease) -> Result<Self> {
        Deserializer::new(IoRead::new(reader), fhir_release)
    }
}

impl<'a> Deserializer<SliceRead<'a>> {
    pub fn from_slice(bytes: &'a [u8], fhir_release: FhirRelease) -> Result<Self> {
        Deserializer::new(SliceRead::new(bytes), fhir_release)
    }
}

impl<'a> Deserializer<StrRead<'a>> {
    pub fn from_str(s: &'a str, fhir_release: FhirRelease) -> Result<Self> {
        Deserializer::new(StrRead::new(s), fhir_release)
    }
}

impl<R: Read> Deserializer<R> {
    fn new(mut reader: R, fhir_release: FhirRelease) -> Result<Self> {
        let first_event = reader.next_event()?;

        let mut path = ElementPath::new(fhir_release);
        match &first_event {
            Event::ElementStart(e) | Event::EmptyElement(e) => path.push(&e.name),
            _ => (),
        }

        Ok(Deserializer {
            reader,
            next_event: first_event,
        })
    }

    fn peek(&mut self) -> &mut Event {
        &mut self.next_event
    }

    fn next_event(&mut self) -> Result<Event> {
        Ok(mem::replace(
            &mut self.next_event,
            self.reader.next_event()?,
        ))
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        ElementDeserializer::new(self).deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum seq identifier ignored_any
    }
}

struct ElementDeserializer<'a, R: Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: Read> ElementDeserializer<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        ElementDeserializer { de }
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut ElementDeserializer<'a, R> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(ElementAccess::new(self.de)?)
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct enum seq identifier ignored_any
    }
}

struct ElementAccess<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    element: Element,
    is_empty: bool,
    is_resource: bool,
    write_resource_type: Option<String>,
}

impl<'a, R: Read> ElementAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Result<Self> {
        let (mut element, is_empty) = match de.next_event()? {
            Event::EmptyElement(e) | Event::Div(e) => (e, true),
            Event::ElementStart(e) => (e, false),
            Event::ElementEnd => {
                return Err(Error::InvalidFhirEvent {
                    found: "end tag",
                    expected: "element",
                })
            }
            Event::Eof => {
                return Err(Error::InvalidFhirEvent {
                    found: "eof",
                    expected: "element",
                })
            }
        };

        let is_resource = element.is_resource();
        let write_resource_type = if is_resource {
            Some(mem::take(&mut element.name))
        } else {
            None
        };

        Ok(ElementAccess {
            de,
            element,
            is_empty,
            is_resource,
            write_resource_type,
        })
    }
}

impl<'de, 'a, R: Read> de::MapAccess<'de> for ElementAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.write_resource_type.is_some() {
            seed.deserialize(StrDeserializer::new("resourceType"))
                .map(Some)
        } else if self.element.id.is_some() {
            seed.deserialize(StrDeserializer::new("id")).map(Some)
        } else if self.element.url.is_some() {
            seed.deserialize(StrDeserializer::new("url")).map(Some)
        } else if self.element.value.is_some() {
            seed.deserialize(StrDeserializer::new("value")).map(Some)
        } else if !self.is_empty {
            match self.de.peek() {
                Event::ElementStart(e) | Event::EmptyElement(e) | Event::Div(e) => {
                    if e.is_resource() {
                        self.element = mem::take(e);

                        self.is_resource = true;
                        self.write_resource_type = Some(mem::take(&mut self.element.name));

                        _ = self.de.next_event()?;

                        self.next_key_seed(seed)
                    } else {
                        seed.deserialize(StringDeserializer::new(mem::take(&mut e.name)))
                            .map(Some)
                    }
                }
                _ => {
                    _ = self.de.next_event()?;

                    if self.is_resource {
                        _ = self.de.next_event()?;
                    }

                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(resource_type) = self.write_resource_type.take() {
            seed.deserialize(StringDeserializer::new(resource_type))
        } else if let Some(id) = self.element.id.take() {
            seed.deserialize(StringDeserializer::new(id))
        } else if let Some(url) = self.element.url.take() {
            seed.deserialize(StringDeserializer::new(url))
        } else if let Some(value) = self.element.value.take() {
            seed.deserialize(StringDeserializer::new(value))
        } else {
            seed.deserialize(&mut ElementDeserializer::new(self.de))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use serde::de::DeserializeSeed;

    use fhirbolt_element::{Element, FhirReleases, Primitive, Value};

    use crate::{
        context::Format,
        xml::{
            error::Result,
            event::{self, Event},
            read::Read,
        },
        DeserializeResource,
    };

    use super::*;

    impl Read for VecDeque<Event> {
        fn next_event(&mut self) -> Result<Event> {
            if let Some(event) = self.pop_front() {
                Ok(event)
            } else {
                Ok(Event::Eof)
            }
        }
    }

    fn mock_deserializer(events: Vec<Event>) -> Deserializer<VecDeque<Event>> {
        Deserializer::new(VecDeque::from(events), FhirReleases::R4).unwrap()
    }

    #[test]
    fn test_resource_id() {
        let mut de = mock_deserializer(vec![
            Event::ElementStart(event::Element {
                name: "Observation".into(),
                ..Default::default()
            }),
            Event::EmptyElement(event::Element {
                name: "id".into(),
                value: Some("test_id".into()),
                ..Default::default()
            }),
            Event::ElementEnd,
        ]);

        assert_eq!(
            Element::<{ FhirReleases::R4 }>::deserialization_context(
                Default::default(),
                Format::Xml
            )
            .deserialize(&mut de)
            .unwrap(),
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "id" => Value::Element(Element! {
                    "value" => Value::Primitive(Primitive::String("test_id".into())),
                })
            }
        );
    }

    #[test]
    fn test_resource_contained() {
        let mut de = mock_deserializer(vec![
            Event::ElementStart(event::Element {
                name: "Observation".into(),
                ..Default::default()
            }),
            Event::ElementStart(event::Element {
                name: "contained".into(),
                ..Default::default()
            }),
            Event::ElementStart(event::Element {
                name: "Observation".into(),
                ..Default::default()
            }),
            Event::EmptyElement(event::Element {
                name: "identifier".into(),
                value: Some("123".into()),
                ..Default::default()
            }),
            Event::ElementEnd,
            Event::ElementEnd,
            Event::EmptyElement(event::Element {
                name: "identifier".into(),
                value: Some("123".into()),
                ..Default::default()
            }),
            Event::ElementEnd,
        ]);

        assert_eq!(
            Element::<{ FhirReleases::R4 }>::deserialization_context(
                Default::default(),
                Format::Xml
            )
            .deserialize(&mut de)
            .unwrap(),
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "contained" =>  Value::Sequence(vec![Element! {
                    "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                    "identifier" => Value::Sequence(vec![Element! {
                        "value" => Value::Primitive(Primitive::String("123".into())),
                    }])
                }]),
                "identifier" => Value::Sequence(vec![Element! {
                    "value" => Value::Primitive(Primitive::String("123".into())),
                }])
            }
        );
    }

    #[test]
    fn test_element_id() {
        let mut de = mock_deserializer(vec![
            Event::ElementStart(event::Element {
                name: "Observation".into(),
                ..Default::default()
            }),
            Event::EmptyElement(event::Element {
                name: "valueString".into(),
                id: Some("test_id".into()),
                ..Default::default()
            }),
            Event::ElementEnd,
        ]);

        assert_eq!(
            Element::<{ FhirReleases::R4 }>::deserialization_context(
                Default::default(),
                Format::Xml
            )
            .deserialize(&mut de)
            .unwrap(),
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "valueString" => Value::Element(Element! {
                    "id" => Value::Primitive(Primitive::String("test_id".into())),
                })
            }
        );
    }

    #[test]
    fn test_element_value() {
        let mut de = mock_deserializer(vec![
            Event::ElementStart(event::Element {
                name: "Observation".into(),
                ..Default::default()
            }),
            Event::EmptyElement(event::Element {
                name: "valueString".into(),
                value: Some("test_value".into()),
                ..Default::default()
            }),
            Event::ElementEnd,
        ]);

        assert_eq!(
            Element::<{ FhirReleases::R4 }>::deserialization_context(
                Default::default(),
                Format::Xml
            )
            .deserialize(&mut de)
            .unwrap(),
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "valueString" => Value::Element(Element! {
                    "value" => Value::Primitive(Primitive::String("test_value".into())),
                })
            }
        );
    }

    #[test]
    fn test_extension_url() {
        let mut de = mock_deserializer(vec![
            Event::ElementStart(event::Element {
                name: "Observation".into(),
                ..Default::default()
            }),
            Event::EmptyElement(event::Element {
                name: "extension".into(),
                url: Some("test_url".into()),
                ..Default::default()
            }),
            Event::ElementEnd,
        ]);

        assert_eq!(
            Element::<{ FhirReleases::R4 }>::deserialization_context(
                Default::default(),
                Format::Xml
            )
            .deserialize(&mut de)
            .unwrap(),
            Element! {
                "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
                "extension" => Value::Sequence(vec![Element! {
                    "url" => Value::Primitive(Primitive::String("test_url".into())),
                }])
            }
        );
    }
}
