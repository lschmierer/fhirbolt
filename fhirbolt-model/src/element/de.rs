use std::{cell::RefCell, marker::PhantomData, mem, rc::Rc};

use serde::de::{DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Unexpected, Visitor};

use fhirbolt_shared::{path::ElementPath, FhirRelease};

use super::{Element, Primitive, Value};

pub struct DeserializationContext<V> {
    _phantom: PhantomData<V>,
    from_json: bool,
    current_path: Rc<RefCell<ElementPath>>,
}

impl DeserializationContext<Value> {
    pub fn new(fhir_release: FhirRelease, from_json: bool) -> Self {
        DeserializationContext {
            _phantom: PhantomData,
            from_json,
            current_path: Rc::new(RefCell::new(ElementPath::new(fhir_release))),
        }
    }
}

impl<V> DeserializationContext<V> {
    fn clone_for<F>(&self) -> DeserializationContext<F> {
        DeserializationContext {
            _phantom: PhantomData,
            from_json: self.from_json,
            current_path: self.current_path.clone(),
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
    fn into_element<'a, D>(self) -> Result<Element, D::Error>
    where
        D: Deserializer<'a>,
    {
        Ok(self
            .0
            .into_iter()
            .map(|(s, v)| v.into_value::<D>().map(|v| (s, v)))
            .collect::<Result<_, D::Error>>()?)
    }
}

impl InternalValue {
    fn into_value<'a, D>(self) -> Result<Value, D::Error>
    where
        D: Deserializer<'a>,
    {
        match self {
            InternalValue::Element(e) => Ok(Value::Element(e.into_element::<D>()?)),
            InternalValue::Sequence(s) => Ok(Value::Sequence(
                s.into_iter()
                    .map(|e| {
                        e.ok_or(D::Error::invalid_type(Unexpected::Option, &"a value"))
                            .and_then(|e| e.into_element::<D>())
                    })
                    .collect::<Result<Vec<_>, D::Error>>()?,
            )),
            InternalValue::Primitive(p) => Ok(Value::Primitive(p)),
        }
    }
}

impl<'de> DeserializeSeed<'de> for DeserializationContext<Value> {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_any(ValueVisitor(self.clone_for()))?
            .into_value::<D>()
    }
}

impl<'de> DeserializeSeed<'de> for DeserializationContext<InternalValue> {
    type Value = InternalValue;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor(self))
    }
}

struct ValueVisitor(DeserializationContext<InternalValue>);

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = InternalValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
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

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
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

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let current_path = self.0.current_path.borrow();
        let current_element = current_path.current_element();

        if self.0.current_path.borrow().current_element_is_resource()
            && current_element == Some("resourceType")
        {
            Ok(InternalValue::Primitive(Primitive::String(v)))
        } else if self.0.from_json
            && current_element != Some("id")
            && current_element != Some("url")
        {
            let mut element = InternalElement::default();
            element.0.insert(
                "value".into(),
                InternalValue::Primitive(Primitive::String(v)),
            );
            Ok(InternalValue::Element(element))
        } else {
            if current_path.current_element_is_boolean() {
                Ok(InternalValue::Primitive(Primitive::Bool(
                    v.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Other(&v), &"a boolean"))?,
                )))
            } else if current_path.current_element_is_integer()
                || current_path.current_element_is_positive_integer()
                || current_path.current_element_is_unsigned_integer()
            {
                Ok(InternalValue::Primitive(Primitive::Integer(
                    v.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Other(&v), &"an integer"))?,
                )))
            } else {
                Ok(InternalValue::Primitive(Primitive::String(v)))
            }
        }
    }

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
        let mut is_resource = false;

        let mut element = InternalElement::default();

        while let Some(key) = map_access.next_key::<String>()? {
            let key = if key.starts_with("_") {
                key[1..].into()
            } else {
                key
            };

            if (self.0.current_path.borrow().current_element_is_resource()
                || self.0.current_path.borrow().is_empty())
                && key == "resourceType"
            {
                let value: String = map_access.next_value()?;

                self.0.current_path.borrow_mut().push(&value);
                is_resource = true;

                element
                    .0
                    .insert(key, InternalValue::Primitive(Primitive::String(value)));
            } else {
                self.0.current_path.as_ref().borrow_mut().push(&key);

                if self.0.from_json && self.0.current_path.borrow().current_element_is_decimal() {
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
                    let value = map_access.next_value_seed(self.0.clone_for::<InternalValue>())?;

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
                    } else if self.0.current_path.borrow().current_element_is_sequence() {
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

                self.0.current_path.as_ref().borrow_mut().pop();
            }
        }

        if is_resource {
            self.0.current_path.as_ref().borrow_mut().pop();
        }

        Ok(InternalValue::Element(element))
    }

    fn visit_seq<V>(self, mut seq_access: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut elements = Vec::new();

        while let Some(value) =
            seq_access.next_element_seed(self.0.clone_for::<Option<InternalValue>>())?
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

impl<'de> DeserializeSeed<'de> for DeserializationContext<Option<InternalValue>> {
    type Value = Option<InternalValue>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(ElementVisitor(self.clone_for()))
    }
}
struct ElementVisitor(DeserializationContext<Option<InternalValue>>);

impl<'de> Visitor<'de> for ElementVisitor {
    type Value = Option<InternalValue>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence element")
    }

    fn visit_map<V>(self, map_access: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        ValueVisitor(self.0.clone_for())
            .visit_map(map_access)
            .map(Some)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0
            .clone_for::<InternalValue>()
            .deserialize(deserializer)
            .map(Some)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }
}
