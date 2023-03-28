//! Deserialize FHIR resources from XML.

use std::{
    borrow::Cow,
    io,
    mem::{self},
};

use serde::{
    de::{self, DeserializeSeed, Visitor},
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
    next_path: ElementPath,
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
            next_path: path,
        })
    }

    fn peek(&mut self) -> &mut Event {
        &mut self.next_event
    }

    fn next_event(&mut self) -> Result<Event> {
        match &self.next_event {
            Event::EmptyElement(_) => self.next_path.pop(),
            _ => (),
        }

        let event = mem::replace(&mut self.next_event, self.reader.next_event()?);

        match &self.next_event {
            Event::ElementStart(e) | Event::EmptyElement(e) | Event::Div(e) => {
                self.next_path.push(&e.name)
            }
            Event::ElementEnd => self.next_path.pop(),
            _ => (),
        }

        Ok(event)
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        ElementDeserializer::new(self, false).deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum seq identifier ignored_any
    }
}

struct ElementDeserializer<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    expect_map: bool,
}

impl<'a, R: Read> ElementDeserializer<'a, R> {
    fn new(de: &'a mut Deserializer<R>, expect_map: bool) -> Self {
        ElementDeserializer { de, expect_map }
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut ElementDeserializer<'a, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if !self.expect_map && self.de.next_path.current_element_is_sequence() {
            self.deserialize_seq(visitor)
        } else {
            self.deserialize_map(visitor)
        }
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SequenceAccess::new(self.de)?)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct enum identifier ignored_any
    }
}

struct ElementAccess<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    element: Element,
    is_empty: bool,
    write_resource_type: bool,
    value_type_hint: TypeHint,
}

impl<'a, R: Read> ElementAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Result<Self> {
        let value_type_hint = if de.next_path.current_element_is_boolean() {
            TypeHint::Bool
        } else if de.next_path.current_element_is_integer() {
            TypeHint::Integer
        } else if de.next_path.current_element_is_positive_integer()
            || de.next_path.current_element_is_unsigned_integer()
        {
            TypeHint::PositiveInt
        } else {
            TypeHint::String
        };

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
            value_type_hint,
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
            seed.deserialize(&mut ValueDeserializer(
                "resourceType".into(),
                TypeHint::String,
            ))
            .map(Some)
        } else if self.element.id.is_some() {
            seed.deserialize(&mut ValueDeserializer("id".into(), TypeHint::String))
                .map(Some)
        } else if self.element.url.is_some() {
            seed.deserialize(&mut ValueDeserializer("url".into(), TypeHint::String))
                .map(Some)
        } else if self.element.value.is_some() {
            seed.deserialize(&mut ValueDeserializer("value".into(), TypeHint::String))
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

                        seed.deserialize(&mut ValueDeserializer("id".into(), TypeHint::String))
                            .map(Some)
                    } else {
                        seed.deserialize(&mut ValueDeserializer(e.name.clone(), TypeHint::String))
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
            seed.deserialize(&mut ValueDeserializer(
                self.element.name.clone(),
                TypeHint::String,
            ))
        } else if let Some(id) = self.element.id.take() {
            seed.deserialize(&mut ValueDeserializer(id.into(), TypeHint::String))
        } else if let Some(url) = self.element.url.take() {
            seed.deserialize(&mut ValueDeserializer(url.into(), TypeHint::String))
        } else if let Some(value) = self.element.value.take() {
            seed.deserialize(&mut ValueDeserializer(value.into(), self.value_type_hint))
        } else {
            seed.deserialize(&mut ElementDeserializer::new(self.de, false))
        }
    }
}

struct SequenceAccess<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    current_element_name: Cow<'static, str>,
}

impl<'a, R: Read> SequenceAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Result<Self> {
        let current_element_name = match de.peek() {
            Event::ElementStart(e) | Event::EmptyElement(e) | Event::Div(e) => e.name.clone(),
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

        Ok(SequenceAccess {
            de,
            current_element_name,
        })
    }
}

impl<'de, 'a, R: Read> de::SeqAccess<'de> for SequenceAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.de.peek() {
            Event::ElementStart(e) | Event::EmptyElement(e) | Event::Div(e) => {
                if e.name == self.current_element_name {
                    seed.deserialize(&mut ElementDeserializer::new(self.de, true))
                        .map(Some)
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum TypeHint {
    Bool,
    Integer,
    PositiveInt,
    String,
}

struct ValueDeserializer(Cow<'static, str>, TypeHint);

impl<'de> de::Deserializer<'de> for &mut ValueDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.1 {
            TypeHint::Bool => visitor.visit_bool(self.0.parse()?),
            TypeHint::Integer => visitor.visit_i32(self.0.parse()?),
            TypeHint::PositiveInt => visitor.visit_u32(self.0.parse()?),
            TypeHint::String => match mem::take(&mut self.0) {
                Cow::Owned(s) => visitor.visit_string(s),
                Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
            },
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
