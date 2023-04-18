use serde::de::{DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Unexpected, Visitor};

use fhirbolt_element::{Element, Primitive, Value};
use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::{
    context::de::{CurrentElement, DeserializationContext},
    DeserializationMode,
};

pub const PRIMITIVE_CHILDREN: &[&str] = &["id", "extension", "value"];

#[derive(Default, Debug)]
pub struct InternalElement(indexmap::IndexMap<String, InternalValue>);

#[derive(Debug)]
enum InternalValue {
    Element(InternalElement),
    Sequence(Vec<Option<InternalElement>>),
    Primitive(InternalPrimitive),
}

#[derive(Debug)]
enum InternalPrimitive {
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
        D: Deserializer<'a>,
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
                        return Err(Error::invalid_type(Unexpected::Bool(b), &"string"))
                    }
                    InternalValue::Primitive(InternalPrimitive::Integer(i)) => {
                        return Err(Error::invalid_type(Unexpected::Signed(i), &"string"))
                    }
                    InternalValue::Element(_) => {
                        return Err(Error::invalid_type(Unexpected::Other("element"), &"string"))
                    }
                    InternalValue::Sequence(_) => {
                        return Err(Error::invalid_type(
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
                            return Err(Error::custom(format_args!(
                                "unknown field `{}`, expected one of {:?}",
                                key, PRIMITIVE_CHILDREN
                            )));
                        }
                    } else {
                        let fields = current_path.children();

                        if !fields.map(|s| s.contains(&key)).unwrap_or(false) {
                            if let Some(expected_fields) = fields {
                                return Err(Error::custom(format_args!(
                                    "unknown field `{}`, expected one of {:?}",
                                    key,
                                    &expected_fields.iter().collect::<Vec<_>>()
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
}

impl InternalValue {
    fn into_value<'a, D, const R: FhirRelease>(
        self,
        deserialization_mode: DeserializationMode,
        current_path: &mut ElementPath,
    ) -> Result<Value<R>, D::Error>
    where
        D: Deserializer<'a>,
    {
        match self {
            InternalValue::Element(e) => Ok(Value::Element(
                e.into_element::<D, R>(deserialization_mode, current_path)?,
            )),
            InternalValue::Sequence(s) => Ok(Value::Sequence(
                s.into_iter()
                    .map(|e| {
                        e.ok_or(Error::invalid_type(Unexpected::Option, &"a value"))
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
                            Err(Error::invalid_type(Unexpected::Signed(i), &expected))
                        }
                        InternalPrimitive::String(s) => {
                            Ok(Value::Primitive(Primitive::Bool(s.parse().map_err(
                                |_| Error::invalid_value(Unexpected::Other(&s), &expected),
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
                            Err(Error::invalid_type(Unexpected::Bool(b), &expected))
                        }
                        InternalPrimitive::Integer(i) => {
                            Ok(Value::Primitive(Primitive::Integer(i)))
                        }
                        InternalPrimitive::String(s) => {
                            Ok(Value::Primitive(Primitive::Integer(s.parse().map_err(
                                |_| Error::invalid_value(Unexpected::Other(&s), &expected),
                            )?)))
                        }
                    }
                } else if current_path.parent_element_is_decimal() {
                    let expected = "a decimal";
                    match p {
                        InternalPrimitive::Bool(b) => {
                            Err(Error::invalid_type(Unexpected::Bool(b), &expected))
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
                            Err(Error::invalid_type(Unexpected::Bool(b), &expected))
                        }
                        InternalPrimitive::Integer(i) => {
                            Err(Error::invalid_type(Unexpected::Signed(i), &expected))
                        }
                        InternalPrimitive::String(s) => Ok(Value::Primitive(Primitive::String(s))),
                    }
                }
            }
        }
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &mut DeserializationContext<InternalElement> {
    type Value = InternalElement;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match deserializer.deserialize_any(ValueVisitor(self.transmute()))? {
            InternalValue::Element(e) => Ok(e),
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

impl<'de> DeserializeSeed<'de> for &mut DeserializationContext<InternalValue> {
    type Value = InternalValue;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
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
        E: Error,
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
        E: Error,
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
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_string())
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
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
        E: Error,
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
                            .map_err(Error::custom)?,
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
        D: Deserializer<'de>,
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
        D: Deserializer<'de>,
    {
        self.0
            .transmute::<InternalValue>()
            .deserialize(deserializer)
            .map(Some)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }
}
