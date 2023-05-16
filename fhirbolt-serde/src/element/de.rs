//! Deserialize FHIR resources as generic element.

use std::{result, vec};

use serde::{
    de::{
        self,
        value::{StrDeserializer, StringDeserializer},
        DeserializeSeed, Error, MapAccess, Unexpected, Visitor,
    },
    forward_to_deserialize_any,
};

use fhirbolt_element::{Element, Primitive, Value};
use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::{
    context::de::DeserializationContext, element::error, element::internal::de::InternalElement,
};

impl<'de, const R: FhirRelease> DeserializeSeed<'de> for DeserializationContext<Element<R>> {
    type Value = Element<R>;

    #[inline]
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.transmute::<InternalElement<R>>()
            .deserialize(deserializer)?
            .into_element::<D>(self.config.mode, &mut ElementPath::new(R))
    }
}

pub struct Deserializer<T>(pub T);

impl<'de, const R: FhirRelease> de::Deserializer<'de> for Deserializer<Element<R>> {
    type Error = error::Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(ElementAccess::new(self.0))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum seq identifier ignored_any
    }
}

impl<'de, const R: FhirRelease> de::Deserializer<'de> for Deserializer<Value<R>> {
    type Error = error::Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Element(e) => visitor.visit_map(ElementAccess::new(e)),
            Value::Sequence(_) => Err(Error::invalid_type(
                Unexpected::Seq,
                &"an element or primitive",
            )),
            Value::Primitive(p) => match p {
                Primitive::Bool(b) => visitor.visit_bool(b),
                Primitive::Integer(i) => visitor.visit_i32(i),
                Primitive::Integer64(i) => visitor.visit_i64(i),
                Primitive::Decimal(s) | Primitive::String(s) => visitor.visit_string(s),
            },
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct enum map struct seq identifier ignored_any
    }
}

struct ElementAccess<const R: FhirRelease> {
    iter: indexmap::map::IntoIter<String, Value<R>>,
    next_key: Option<String>,
    next_value: Option<Value<R>>,
    next_seq_iter: Option<vec::IntoIter<Element<R>>>,
}

impl<const R: FhirRelease> ElementAccess<R> {
    fn new(mut element: Element<R>) -> ElementAccess<R> {
        // if in resource, it is important that the resourceType is deserialized as first field
        let resource_type = element.remove("resourceType");

        let next_key = if resource_type.is_some() {
            Some("resourceType".into())
        } else {
            None
        };

        let (next_seq_iter, next_value) = match resource_type {
            Some(Value::Sequence(s)) => {
                let mut iter = s.into_iter();
                let next_value = iter.next().map(Value::Element);

                (Some(iter), next_value)
            }
            Some(v) => (None, Some(v)),
            _ => (None, None),
        };

        ElementAccess {
            iter: element.into_iter(),
            next_key,
            next_value,
            next_seq_iter,
        }
    }
}

impl<'de, const R: FhirRelease> MapAccess<'de> for ElementAccess<R> {
    type Error = error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> error::Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(key) = &self.next_key {
            seed.deserialize(StrDeserializer::new(key)).map(Some)
        } else if let Some((key, value)) = self.iter.next() {
            match value {
                Value::Sequence(s) => {
                    self.next_key = Some(key);
                    self.next_seq_iter = Some(s.into_iter());
                    self.next_value = self
                        .next_seq_iter
                        .as_mut()
                        .unwrap()
                        .next()
                        .map(Value::Element);

                    seed.deserialize(StrDeserializer::new(self.next_key.as_ref().unwrap()))
                        .map(Some)
                }
                _ => {
                    self.next_value = Some(value);

                    seed.deserialize(StringDeserializer::new(key)).map(Some)
                }
            }
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> error::Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let next_value = self.next_value.take().unwrap();

        if let Some(s) = &mut self.next_seq_iter {
            self.next_value = s.next().map(Value::Element);
        }

        if self.next_value.is_none() {
            self.next_key = None;
            self.next_seq_iter = None;
        }

        seed.deserialize(Deserializer(next_value))
    }
}
