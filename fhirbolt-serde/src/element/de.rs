//! Deserialize FHIR resources as generic element.

use std::{mem, vec};

use serde::{
    de::{
        self,
        value::{StrDeserializer, StringDeserializer},
        DeserializeSeed, Error, MapAccess, SeqAccess, Unexpected, Visitor,
    },
    forward_to_deserialize_any,
};

use fhirbolt_element::{Element, Primitive, Value};
use fhirbolt_shared::FhirRelease;

use crate::{context::de::DeserializationContext, element::error, DeserializationMode};

pub const PRIMITIVE_CHILDREN: &[&str] = &["id", "extension", "value"];

impl<'a, 'de, const R: FhirRelease> DeserializeSeed<'de> for DeserializationContext<Element<R>> {
    type Value = Element<R>;

    #[inline]
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        match deserializer.deserialize_any(ValueVisitor(self.transmute()))? {
            InternalValue::Element(e) => e.into_element::<D, R>(),
            InternalValue::Sequence(_) => {
                Err(D::Error::invalid_type(Unexpected::Seq, &"an element"))
            }
            InternalValue::Primitive(_) => Err(D::Error::invalid_type(
                Unexpected::Other("primitive"),
                &"an element",
            )),
        }
    }
}

#[derive(Default, Debug)]
pub struct InternalElement(indexmap::IndexMap<String, InternalValue>);

#[derive(Debug)]
enum InternalValue {
    Element(InternalElement),
    Sequence(Vec<Option<InternalElement>>),
    Primitive(Primitive),
}

impl InternalElement {
    fn into_element<'a, D, const R: FhirRelease>(self) -> Result<Element<R>, D::Error>
    where
        D: de::Deserializer<'a>,
    {
        Ok(Element::from_iter(
            self.0
                .into_iter()
                .map(|(s, v)| v.into_value::<D, R>().map(|v| (s, v)))
                .collect::<Result<Vec<_>, D::Error>>()?,
        ))
    }
}

impl InternalValue {
    fn into_value<'a, D, const R: FhirRelease>(self) -> Result<Value<R>, D::Error>
    where
        D: de::Deserializer<'a>,
    {
        match self {
            InternalValue::Element(e) => Ok(Value::Element(e.into_element::<D, R>()?)),
            InternalValue::Sequence(s) => Ok(Value::Sequence(
                s.into_iter()
                    .map(|e| {
                        e.ok_or(D::Error::invalid_type(Unexpected::Option, &"a value"))
                            .and_then(|e| e.into_element::<D, R>())
                    })
                    .collect::<Result<Vec<_>, D::Error>>()?,
            )),
            InternalValue::Primitive(p) => Ok(Value::Primitive(p)),
        }
    }
}

impl<'de> DeserializeSeed<'de> for &mut DeserializationContext<InternalValue> {
    type Value = InternalValue;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor(self))
    }
}

struct ValueVisitor<'a>(&'a mut DeserializationContext<InternalValue>);

impl<'a, 'de> Visitor<'de> for ValueVisitor<'a> {
    type Value = InternalValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.from_json {
            let mut element = InternalElement::default();
            element
                .0
                .insert("value".into(), InternalValue::Primitive(Primitive::Bool(v)));
            Ok(InternalValue::Element(element))
        } else {
            Ok(InternalValue::Primitive(Primitive::Bool(v)))
        }
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.from_json {
            let mut element = InternalElement::default();
            element.0.insert(
                "value".into(),
                InternalValue::Primitive(Primitive::Integer(v)),
            );
            Ok(InternalValue::Element(element))
        } else {
            Ok(InternalValue::Primitive(Primitive::Integer(v)))
        }
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(v.to_string())
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let current_path = &self.0.current_path;
        let current_element = current_path.current_element();

        if self.0.from_json {
            if current_element == Some("id")
                || current_path.currently_in_extension() && current_element == Some("url")
            {
                Ok(InternalValue::Primitive(Primitive::String(v)))
            } else {
                let mut element = InternalElement::default();
                element.0.insert(
                    "value".into(),
                    InternalValue::Primitive(Primitive::String(v)),
                );
                Ok(InternalValue::Element(element))
            }
        } else if current_element == Some("value") {
            if current_path.parent_element_is_boolean() {
                Ok(InternalValue::Primitive(Primitive::Bool(
                    v.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Other(&v), &"a boolean"))?,
                )))
            } else if current_path.parent_element_is_integer()
                || current_path.parent_element_is_positive_integer()
                || current_path.parent_element_is_unsigned_integer()
            {
                Ok(InternalValue::Primitive(Primitive::Integer(
                    v.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Other(&v), &"an integer"))?,
                )))
            } else if current_path.parent_element_is_decimal() {
                Ok(InternalValue::Primitive(Primitive::Decimal(v)))
            } else {
                Ok(InternalValue::Primitive(Primitive::String(v)))
            }
        } else {
            Ok(InternalValue::Primitive(Primitive::String(v)))
        }
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut is_resource = false;

        let mut element = InternalElement::default();

        while let Some(key) = map_access.next_key::<String>()? {
            let key = if key.starts_with("_") {
                key[1..].into()
            } else {
                key
            };

            // in case of top-level resource: current_path is empty
            if (self.0.current_path.current_element_is_resource() || self.0.current_path.is_empty())
                && key == "resourceType"
            {
                let value: String = map_access.next_value()?;

                self.0.current_path.push(&value);
                is_resource = true;

                element
                    .0
                    .insert(key, InternalValue::Primitive(Primitive::String(value)));
            } else {
                // check if field is valid at current path
                if self.0.config.mode == DeserializationMode::Strict {
                    if self.0.current_path.current_element_is_primitive() {
                        if !PRIMITIVE_CHILDREN.contains(&key.as_str()) {
                            return Err(Error::custom(format_args!(
                                "unknown field `{}`, expected one of {:?}",
                                key, PRIMITIVE_CHILDREN
                            )));
                        }
                    } else {
                        let fields = self.0.current_path.children();

                        if !fields.map(|s| s.contains(&key)).unwrap_or(false) {
                            if let Some(expected_fields) = fields {
                                return Err(Error::custom(format_args!(
                                    "unknown field `{}`, expected one of {:?}",
                                    key, expected_fields
                                )));
                            } else {
                                return Err(Error::custom(format_args!(
                                    "unknown field `{}`, there are no fields",
                                    key
                                )));
                            }
                        }
                    }
                }

                self.0.current_path.push(&key);

                if self.0.from_json && self.0.current_path.current_element_is_decimal() {
                    let value: serde_json::Number = map_access.next_value()?;

                    let mut decimal_element = InternalElement::default();
                    decimal_element.0.insert(
                        "value".into(),
                        InternalValue::Primitive(Primitive::Decimal(value.to_string())),
                    );

                    element
                        .0
                        .insert(key, InternalValue::Element(decimal_element));
                } else {
                    let value = map_access.next_value_seed(self.0.transmute::<InternalValue>())?;

                    if let Some(existing) = element.0.get_mut(&key) {
                        match (existing, value) {
                            (InternalValue::Element(e), InternalValue::Element(n)) => {
                                e.0.extend(n.0)
                            }
                            (InternalValue::Sequence(ev), InternalValue::Sequence(nv)) => {
                                *ev = mem::take(ev)
                                    .into_iter()
                                    .zip(nv)
                                    .take_while(|(e, n)| e.is_some() || n.is_some())
                                    .map(|(e, n)| match (e, n) {
                                        (Some(mut e), Some(n)) => {
                                            e.0.extend(n.0);
                                            Some(e)
                                        }
                                        (Some(e), None) => Some(e),
                                        (None, Some(n)) => Some(n),
                                        _ => None,
                                    })
                                    .collect()
                            }
                            (InternalValue::Sequence(es), InternalValue::Element(n)) => {
                                es.push(Some(n))
                            }
                            (e, v) => *e = v,
                        }
                    } else if self.0.current_path.current_element_is_sequence() {
                        match value {
                            InternalValue::Element(e) => element
                                .0
                                .insert(key, InternalValue::Sequence(vec![Some(e)])),
                            InternalValue::Sequence(s) => {
                                element.0.insert(key, InternalValue::Sequence(s))
                            }
                            InternalValue::Primitive(v) => {
                                return Err(V::Error::invalid_type(
                                    Unexpected::Other(&format!("primitive value {:?}", v)),
                                    &"a sequence element",
                                ))
                            }
                        };
                    } else {
                        element.0.insert(key, value);
                    }
                }

                self.0.current_path.pop();
            }
        }

        if is_resource {
            self.0.current_path.pop();
        }

        Ok(InternalValue::Element(element))
    }

    fn visit_seq<V>(self, mut seq_access: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut elements = Vec::new();

        while let Some(value) =
            seq_access.next_element_seed(self.0.transmute::<Option<InternalValue>>())?
        {
            match value {
                Some(InternalValue::Element(e)) => elements.push(Some(e)),
                Some(InternalValue::Sequence(_)) => {
                    return Err(V::Error::invalid_type(
                        Unexpected::Seq,
                        &"a sequence element",
                    ))
                }
                Some(InternalValue::Primitive(_)) => {
                    return Err(V::Error::invalid_type(
                        Unexpected::Seq,
                        &"a sequence element",
                    ))
                }
                None => elements.push(None),
            }
        }

        Ok(InternalValue::Sequence(elements))
    }
}

impl<'de> DeserializeSeed<'de> for &mut DeserializationContext<Option<InternalValue>> {
    type Value = Option<InternalValue>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(SeqElementVisitor(self))
    }
}
struct SeqElementVisitor<'a>(&'a mut DeserializationContext<Option<InternalValue>>);

impl<'a, 'de> Visitor<'de> for SeqElementVisitor<'a> {
    type Value = Option<InternalValue>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence element")
    }

    #[inline]
    fn visit_map<V>(self, map_access: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        ValueVisitor(self.0.transmute())
            .visit_map(map_access)
            .map(Some)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.0
            .transmute::<InternalValue>()
            .deserialize(deserializer)
            .map(Some)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
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
                Primitive::Integer(i) => visitor.visit_i64(i),
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
    resource_type: Option<Value<R>>,
    next_key: Option<String>,
    next_value: Option<Value<R>>,
    next_seq_iter: Option<vec::IntoIter<Element<R>>>,
}

impl<const R: FhirRelease> ElementAccess<R> {
    fn new(mut element: Element<R>) -> ElementAccess<R> {
        let resource_type = element.remove("resourceType");

        ElementAccess {
            iter: element.into_iter(),
            resource_type,
            next_key: None,
            next_value: None,
            next_seq_iter: None,
        }
    }
}

impl<'de, const R: FhirRelease> MapAccess<'de> for ElementAccess<R> {
    type Error = error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> error::Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(resource_type) = self.resource_type.take() {
            self.next_value = Some(resource_type);

            seed.deserialize(StrDeserializer::new("resourceType"))
                .map(Some)
        } else if let Some(key) = &self.next_key {
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
