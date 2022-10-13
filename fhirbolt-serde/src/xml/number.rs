use serde::{
    de::{self, DeserializeSeed, IntoDeserializer, MapAccess, Visitor},
    forward_to_deserialize_any,
};

use crate::xml::error::{Error, Result};

const SERDE_NUMBER_TOKEN: &str = "$serde_json::private::Number";

/// `NumberDeserializer` was taken (and adapted) from serde_json
/// (with "arbitrary_precision" feature).
///
/// Decimals are deserialized into serde_json's intermediate arbitrary_precision
/// representation `{ "$serde_json::private::Number": "123.456" }` to not loose precision.
/// fhirbolt model types know how to deserialize from this representation.
pub struct NumberDeserializer {
    pub number: Option<String>,
}

impl NumberDeserializer {
    pub fn new(number: String) -> NumberDeserializer {
        NumberDeserializer {
            number: Some(number),
        }
    }
}

impl<'de> MapAccess<'de> for NumberDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.number.is_none() {
            return Ok(None);
        }
        seed.deserialize(NumberFieldDeserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self.number.take().unwrap().into_deserializer())
    }
}

struct NumberFieldDeserializer;

impl<'de> de::Deserializer<'de> for NumberFieldDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(SERDE_NUMBER_TOKEN)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
