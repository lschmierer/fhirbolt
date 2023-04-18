use std::{collections::VecDeque, vec};

use serde::{
    de::{
        self,
        value::{StrDeserializer, StringDeserializer},
        DeserializeSeed, MapAccess, SeqAccess, Unexpected, Visitor,
    },
    forward_to_deserialize_any,
};

use fhirbolt_element::{Element, Primitive, Value};
use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::{
    context::de::{CurrentElement, DeserializationContext},
    element::{self, Deserializer},
    DeserializationMode,
};

pub const PRIMITIVE_CHILDREN: &[&str] = &["id", "extension", "value"];

#[derive(Default, Debug)]
pub struct InternalElement(pub indexmap::IndexMap<String, InternalValue>);

#[derive(Debug)]
pub enum InternalValue {
    Element(InternalElement),
    Sequence(Vec<Option<InternalElement>>),
    Primitive(InternalPrimitive),
}

#[derive(Debug)]
pub enum InternalPrimitive {
    Bool(bool),
    Integer(i64),
    String(String),
}

impl InternalElement {
    pub fn into_element<'a, D, const R: FhirRelease>(
        self,
        deserialization_mode: DeserializationMode,
        current_path: &mut ElementPath,
    ) -> Result<Element<R>, D::Error>
    where
        D: de::Deserializer<'a>,
    {
        let mut is_resource = false;
        let mut element = Element::new();

        for (key, value) in self.0 {
            // in case of top-level resource: current_path is empty
            if (current_path.current_element_is_resource() || current_path.is_empty())
                && key == "resourceType"
            {
                match value {
                    InternalValue::Primitive(InternalPrimitive::String(s)) => {
                        current_path.push(&s);
                        is_resource = true;

                        element.insert(key, Value::Primitive(Primitive::String(s)));
                    }
                    InternalValue::Primitive(InternalPrimitive::Bool(b)) => {
                        return Err(de::Error::invalid_type(Unexpected::Bool(b), &"string"))
                    }
                    InternalValue::Primitive(InternalPrimitive::Integer(i)) => {
                        return Err(de::Error::invalid_type(Unexpected::Signed(i), &"string"))
                    }
                    InternalValue::Element(_) => {
                        return Err(de::Error::invalid_type(
                            Unexpected::Other("element"),
                            &"string",
                        ))
                    }
                    InternalValue::Sequence(_) => {
                        return Err(de::Error::invalid_type(
                            Unexpected::Other("sequence"),
                            &"string",
                        ))
                    }
                }
            } else {
                // check if field is valid at current path
                if deserialization_mode == DeserializationMode::Strict {
                    if current_path.current_element_is_primitive() {
                        if !PRIMITIVE_CHILDREN.contains(&key.as_str()) {
                            return Err(de::Error::custom(format_args!(
                                "unknown field `{}`, expected one of {:?}",
                                key, PRIMITIVE_CHILDREN
                            )));
                        }
                    } else {
                        let fields = current_path.children();

                        if !fields.map(|s| s.contains(&key)).unwrap_or(false) {
                            if let Some(expected_fields) = fields {
                                return Err(de::Error::custom(format_args!(
                                    "unknown field `{}`, expected one of {:?}",
                                    key,
                                    &expected_fields.iter().collect::<Vec<_>>()
                                )));
                            } else {
                                return Err(de::Error::custom(format_args!(
                                    "unknown field `{}`, there are no fields",
                                    key
                                )));
                            }
                        }
                    }
                }

                current_path.push(&key);

                let value = value.into_value::<D, { R }>(deserialization_mode, current_path)?;
                element.insert(
                    key,
                    match value {
                        Value::Element(e) if current_path.current_element_is_sequence() => {
                            Value::Sequence(vec![e])
                        }
                        value => value,
                    },
                );

                current_path.pop();
            }
        }

        if is_resource {
            current_path.pop();
        }

        Ok(element)
    }

    pub fn from_element<const R: FhirRelease>(element: Element<R>) -> Self {
        InternalElement(indexmap::IndexMap::from_iter(
            element
                .into_iter()
                .map(|(key, value)| (key, InternalValue::from_value(value))),
        ))
    }
}

impl InternalValue {
    fn into_value<'a, D, const R: FhirRelease>(
        self,
        deserialization_mode: DeserializationMode,
        current_path: &mut ElementPath,
    ) -> Result<Value<R>, D::Error>
    where
        D: de::Deserializer<'a>,
    {
        match self {
            InternalValue::Element(e) => Ok(Value::Element(
                e.into_element::<D, R>(deserialization_mode, current_path)?,
            )),
            InternalValue::Sequence(s) => Ok(Value::Sequence(
                s.into_iter()
                    .map(|e| {
                        e.ok_or(de::Error::invalid_type(Unexpected::Option, &"a value"))
                            .and_then(|e| {
                                e.into_element::<D, R>(deserialization_mode, current_path)
                            })
                    })
                    .collect::<Result<Vec<_>, D::Error>>()?,
            )),
            InternalValue::Primitive(p) => {
                if current_path.parent_element_is_boolean() {
                    let expected = "a boolean";
                    match p {
                        InternalPrimitive::Bool(b) => Ok(Value::Primitive(Primitive::Bool(b))),
                        InternalPrimitive::Integer(i) => {
                            Err(de::Error::invalid_type(Unexpected::Signed(i), &expected))
                        }
                        InternalPrimitive::String(s) => {
                            Ok(Value::Primitive(Primitive::Bool(s.parse().map_err(
                                |_| de::Error::invalid_value(Unexpected::Other(&s), &expected),
                            )?)))
                        }
                    }
                } else if current_path.parent_element_is_integer()
                    || current_path.parent_element_is_positive_integer()
                    || current_path.parent_element_is_unsigned_integer()
                {
                    let expected = "an integer";
                    match p {
                        InternalPrimitive::Bool(b) => {
                            Err(de::Error::invalid_type(Unexpected::Bool(b), &expected))
                        }
                        InternalPrimitive::Integer(i) => {
                            Ok(Value::Primitive(Primitive::Integer(i)))
                        }
                        InternalPrimitive::String(s) => {
                            Ok(Value::Primitive(Primitive::Integer(s.parse().map_err(
                                |_| de::Error::invalid_value(Unexpected::Other(&s), &expected),
                            )?)))
                        }
                    }
                } else if current_path.parent_element_is_decimal() {
                    let expected = "a decimal";
                    match p {
                        InternalPrimitive::Bool(b) => {
                            Err(de::Error::invalid_type(Unexpected::Bool(b), &expected))
                        }
                        InternalPrimitive::Integer(i) => {
                            Ok(Value::Primitive(Primitive::Decimal(i.to_string())))
                        }
                        InternalPrimitive::String(s) => Ok(Value::Primitive(Primitive::Decimal(s))),
                    }
                } else {
                    let expected = "a string";
                    match p {
                        InternalPrimitive::Bool(b) => {
                            Err(de::Error::invalid_type(Unexpected::Bool(b), &expected))
                        }
                        InternalPrimitive::Integer(i) => {
                            Err(de::Error::invalid_type(Unexpected::Signed(i), &expected))
                        }
                        InternalPrimitive::String(s) => Ok(Value::Primitive(Primitive::String(s))),
                    }
                }
            }
        }
    }

    pub fn from_value<const R: FhirRelease>(value: Value<R>) -> Self {
        match value {
            Value::Element(e) => InternalValue::Element(InternalElement::from_element(e)),
            Value::Sequence(s) => InternalValue::Sequence(
                s.into_iter()
                    .map(|e| Some(InternalElement::from_element(e)))
                    .collect(),
            ),
            Value::Primitive(p) => match p {
                Primitive::Bool(b) => InternalValue::Primitive(InternalPrimitive::Bool(b)),
                Primitive::Integer(b) => InternalValue::Primitive(InternalPrimitive::Integer(b)),
                Primitive::Decimal(s) | Primitive::String(s) => {
                    InternalValue::Primitive(InternalPrimitive::String(s))
                }
            },
        }
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &mut DeserializationContext<InternalElement> {
    type Value = InternalElement;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        match deserializer.deserialize_any(ValueVisitor(self.transmute()))? {
            InternalValue::Element(e) => Ok(e),
            InternalValue::Sequence(_) => {
                Err(de::Error::invalid_type(Unexpected::Seq, &"an element"))
            }
            InternalValue::Primitive(_) => Err(de::Error::invalid_type(
                Unexpected::Other("primitive"),
                &"an element",
            )),
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

fn merge_sequences(
    left: Vec<Option<InternalElement>>,
    right: Vec<Option<InternalElement>>,
) -> Vec<Option<InternalElement>> {
    left.into_iter()
        .zip(right)
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
            element.0.insert(
                "value".into(),
                InternalValue::Primitive(InternalPrimitive::Bool(v)),
            );
            Ok(InternalValue::Element(element))
        } else {
            Ok(InternalValue::Primitive(InternalPrimitive::Bool(v)))
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
                InternalValue::Primitive(InternalPrimitive::Integer(v)),
            );
            Ok(InternalValue::Element(element))
        } else {
            Ok(InternalValue::Primitive(InternalPrimitive::Integer(v)))
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
        if self.0.from_json {
            if self.0.current_element() == CurrentElement::Id
                || self.0.current_element() == CurrentElement::ExtensionUrl
            {
                Ok(InternalValue::Primitive(InternalPrimitive::String(v)))
            } else {
                let mut element = InternalElement::default();
                element.0.insert(
                    "value".into(),
                    InternalValue::Primitive(InternalPrimitive::String(v)),
                );
                Ok(InternalValue::Element(element))
            }
        } else {
            Ok(InternalValue::Primitive(InternalPrimitive::String(v)))
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
        let mut element = InternalElement::default();

        while let Some(key) = map_access.next_key::<String>()? {
            let key = if key.starts_with("_") {
                key[1..].into()
            } else {
                key
            };

            if key == "resourceType"
                && self.0.current_element() != CurrentElement::ExampleScenarioInstance
            {
                let value: String = map_access.next_value()?;

                element.0.insert(
                    key,
                    InternalValue::Primitive(InternalPrimitive::String(value)),
                );
            } else {
                self.0.push_current_element(match key.as_str() {
                    "id" => CurrentElement::Id,
                    "instance" => CurrentElement::ExampleScenarioInstance,
                    "extension" | "modifierExtension" => CurrentElement::Extension,
                    "url" => {
                        if self.0.current_element() == CurrentElement::Extension {
                            CurrentElement::ExtensionUrl
                        } else {
                            CurrentElement::Other
                        }
                    }
                    _ => CurrentElement::Other,
                });

                let value = if self.0.from_json {
                    let serde_json_value: serde_json::Value = map_access.next_value()?;
                    match serde_json_value {
                        serde_json::Value::Number(n) => {
                            let mut decimal_element = InternalElement::default();
                            decimal_element.0.insert(
                                "value".into(),
                                InternalValue::Primitive(InternalPrimitive::String(n.to_string())),
                            );
                            InternalValue::Element(decimal_element)
                        }
                        serde_json_value => self
                            .0
                            .transmute::<InternalValue>()
                            .deserialize(serde_json_value)
                            .map_err(de::Error::custom)?,
                    }
                } else {
                    map_access.next_value_seed(self.0.transmute::<InternalValue>())?
                };

                let existing = element.0.remove(&key);
                element.0.insert(
                    key,
                    match (existing, value) {
                        (Some(InternalValue::Element(mut e)), InternalValue::Element(n)) => {
                            if self.0.from_json {
                                e.0.extend(n.0);
                                InternalValue::Element(e)
                            } else {
                                InternalValue::Sequence(vec![Some(e), Some(n)])
                            }
                        }
                        (Some(InternalValue::Sequence(ev)), InternalValue::Sequence(nv)) => {
                            InternalValue::Sequence(merge_sequences(ev, nv))
                        }
                        (Some(InternalValue::Sequence(mut es)), InternalValue::Element(n)) => {
                            es.push(Some(n));
                            InternalValue::Sequence(es)
                        }
                        (_e, v) => v,
                    },
                );

                self.0.pop_current_element();
            }
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
                    return Err(de::Error::invalid_type(
                        Unexpected::Seq,
                        &"a sequence element",
                    ))
                }
                Some(InternalValue::Primitive(_)) => {
                    return Err(de::Error::invalid_type(
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

impl<'de> de::Deserializer<'de> for Deserializer<InternalElement> {
    type Error = element::error::Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> element::error::Result<V::Value>
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

impl<'de> de::Deserializer<'de> for Deserializer<InternalValue> {
    type Error = element::error::Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> element::error::Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            InternalValue::Element(e) => visitor.visit_map(ElementAccess::new(e)),
            InternalValue::Sequence(_) => Err(de::Error::invalid_type(
                Unexpected::Seq,
                &"an element or primitive",
            )),
            InternalValue::Primitive(p) => match p {
                InternalPrimitive::Bool(b) => visitor.visit_bool(b),
                InternalPrimitive::Integer(i) => visitor.visit_i64(i),
                InternalPrimitive::String(s) => visitor.visit_string(s),
            },
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct enum map struct seq identifier ignored_any
    }
}

struct ElementAccess {
    iter: indexmap::map::IntoIter<String, InternalValue>,
    resource_type: Option<InternalValue>,
    next_key: Option<String>,
    next_value: Option<InternalValue>,
    next_seq: Option<VecDeque<InternalElement>>,
}

impl ElementAccess {
    fn new(mut element: InternalElement) -> ElementAccess {
        let resource_type = element.0.remove("resourceType");

        ElementAccess {
            iter: element.0.into_iter(),
            resource_type,
            next_key: None,
            next_value: None,
            next_seq: None,
        }
    }
}

impl<'de> MapAccess<'de> for ElementAccess {
    type Error = element::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> element::error::Result<Option<K::Value>>
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
                InternalValue::Sequence(s) => {
                    self.next_key = Some(key);
                    self.next_seq = Some(
                        s.into_iter()
                            .map(|e| {
                                e.ok_or(element::error::Error(
                                    "invalid None, expected a valu".to_string(),
                                ))
                            })
                            .collect::<element::error::Result<_>>()?,
                    );
                    self.next_value = self
                        .next_seq
                        .as_mut()
                        .unwrap()
                        .pop_front()
                        .map(InternalValue::Element);

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

    fn next_value_seed<V>(&mut self, seed: V) -> element::error::Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let next_value = self.next_value.take().unwrap();

        if let Some(s) = &mut self.next_seq {
            self.next_value = s.pop_front().map(InternalValue::Element);
        }

        if self.next_value.is_none() {
            self.next_key = None;
            self.next_seq = None;
        }

        seed.deserialize(Deserializer(next_value))
    }
}
