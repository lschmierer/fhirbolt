//! Deserialize FHIR resources as generic element.

use serde::{
    de::{self, DeserializeSeed, Visitor},
    forward_to_deserialize_any,
};

use fhirbolt_element::Element;
use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::{
    context::de::DeserializationContext, element::error, element::internal::de::InternalElement,
};

impl<'a, 'de, const R: FhirRelease> DeserializeSeed<'de> for DeserializationContext<Element<R>> {
    type Value = Element<R>;

    #[inline]
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.transmute::<InternalElement>()
            .deserialize(deserializer)?
            .into_element::<D, { R }>(self.config.mode, &mut ElementPath::new(R))
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
        Deserializer(InternalElement::from_element(self.0)).deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum seq identifier ignored_any
    }
}
