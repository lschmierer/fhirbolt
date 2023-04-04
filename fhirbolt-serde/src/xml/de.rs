//! Deserialize FHIR resources from XML.

use std::{
    io,
    mem::{self},
};

use serde::{
    de::{self, value::CowStrDeserializer, DeserializeSeed, Visitor},
    forward_to_deserialize_any,
};

use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::xml::{
    error::{Error, Result},
    event::{Element, Event},
    read::{self, Read},
};

pub struct Deserializer<R: Read> {
    reader: R,
    next_event: Event,
}

impl<R: io::Read> Deserializer<read::IoRead<R>> {
    pub fn from_reader(reader: R, fhir_release: FhirRelease) -> Result<Self> {
        Deserializer::new(read::IoRead::new(reader), fhir_release)
    }
}

impl<'a> Deserializer<read::SliceRead<'a>> {
    pub fn from_slice(bytes: &'a [u8], fhir_release: FhirRelease) -> Result<Self> {
        Deserializer::new(read::SliceRead::new(bytes), fhir_release)
    }
}

impl<'a> Deserializer<read::StrRead<'a>> {
    pub fn from_str(s: &'a str, fhir_release: FhirRelease) -> Result<Self> {
        Deserializer::new(read::StrRead::new(s), fhir_release)
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

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(ElementAccess::new(self.de)?)
    }

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
    write_resource_type: bool,
}

impl<'a, R: Read> ElementAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Result<Self> {
        let (element, is_empty) = match de.next_event()? {
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

        let write_resource_type = element.is_resource();

        Ok(ElementAccess {
            de,
            element,
            is_empty,
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
        if self.write_resource_type {
            seed.deserialize(CowStrDeserializer::new("resourceType".into()))
                .map(Some)
        } else if self.element.id.is_some() {
            seed.deserialize(CowStrDeserializer::new("id".into()))
                .map(Some)
        } else if self.element.url.is_some() {
            seed.deserialize(CowStrDeserializer::new("url".into()))
                .map(Some)
        } else if self.element.value.is_some() {
            seed.deserialize(CowStrDeserializer::new("value".into()))
                .map(Some)
        } else if !self.is_empty {
            match self.de.peek() {
                Event::ElementStart(e) | Event::EmptyElement(e) | Event::Div(e) => {
                    if e.is_resource() {
                        self.element = mem::take(e);
                        self.write_resource_type = true;
                        _ = self.de.next_event()?;

                        self.next_key_seed(seed)
                    } else if self.element.is_resource() && e.name == "id" {
                        self.element.id = e.value.take();
                        _ = self.de.next_event()?;

                        seed.deserialize(CowStrDeserializer::new("id".into()))
                            .map(Some)
                    } else {
                        seed.deserialize(CowStrDeserializer::new(e.name.clone()))
                            .map(Some)
                    }
                }
                _ => {
                    _ = self.de.next_event()?;

                    if self.element.is_resource() {
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
        if mem::replace(&mut self.write_resource_type, false) {
            seed.deserialize(CowStrDeserializer::new(self.element.name.clone()))
        } else if let Some(id) = self.element.id.take() {
            seed.deserialize(CowStrDeserializer::new(id.into()))
        } else if let Some(url) = self.element.url.take() {
            seed.deserialize(CowStrDeserializer::new(url.into()))
        } else if let Some(value) = self.element.value.take() {
            seed.deserialize(CowStrDeserializer::new(value.into()))
        } else {
            seed.deserialize(&mut ElementDeserializer::new(self.de))
        }
    }
}
