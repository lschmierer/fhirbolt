use std::{cell::RefCell, rc::Rc, str::FromStr};

use serde::{
    ser::{Error, SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

use fhirbolt_shared::{path::ElementPath, FhirRelease};

use super::{Element, Primitive, Value};

pub struct SerializationContext<V> {
    value: V,
    output_json: bool,
    current_path: Rc<RefCell<ElementPath>>,
}

impl<'a> SerializationContext<&'a Value> {
    pub fn new(value: &'a Value, fhir_release: FhirRelease, output_json: bool) -> Self {
        SerializationContext {
            value,
            output_json,
            current_path: Rc::new(RefCell::new(ElementPath::new(fhir_release))),
        }
    }
}

impl<V: Clone> SerializationContext<V> {
    fn clone_with<W>(&self, value: W) -> SerializationContext<W> {
        SerializationContext {
            value,
            output_json: self.output_json,
            current_path: self.current_path.clone(),
        }
    }
}

impl Serialize for SerializationContext<&Value> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.value {
            Value::Element(element) => self.clone_with(element).serialize(serializer),
            Value::Sequence(sequence) => {
                let mut seq = serializer.serialize_seq(Some(sequence.len()))?;
                for e in sequence {
                    seq.serialize_element(&self.clone_with(e))?;
                }
                seq.end()
            }
            Value::Primitive(value) => match value {
                Primitive::Bool(b) => serializer.serialize_bool(*b),
                Primitive::Integer(i) => serializer.serialize_i64(*i),
                Primitive::Decimal(s) => {
                    if self.output_json {
                        serde_json::Number::from_str(s)
                            .map_err(|_| Error::custom(format!("invalid decimal: \"{}\"", s)))?
                            .serialize(serializer)
                    } else {
                        serializer.serialize_str(s)
                    }
                }
                Primitive::String(s) => serializer.serialize_str(s),
            },
        }
    }
}

impl Serialize for SerializationContext<&Element> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.value.len()))?;

        let mut is_resource = false;
        if let Some(Value::Primitive(Primitive::String(r))) = self.value.get("resourceType") {
            self.current_path.borrow_mut().push(r);
            is_resource = true;
        }

        for (key, value) in self.value {
            self.current_path.borrow_mut().push(&key);

            if self.output_json && self.current_path.borrow().current_element_is_primitive() {
                match value {
                    Value::Element(element) => {
                        if let Some(v) = element.get("value") {
                            map.serialize_entry(key, &self.clone_with(v))?;
                        }

                        let primitive_element = PrimitiveElement::from_element(element)?;

                        if primitive_element.id.is_some() || !primitive_element.extension.is_empty()
                        {
                            map.serialize_entry(
                                &format!("_{}", key),
                                &self.clone_with(primitive_element),
                            )?;
                        }
                    }
                    Value::Sequence(sequence) => {
                        let values = sequence
                            .iter()
                            .map(|e| e.get("value").map(|v| self.clone_with(v)))
                            .collect::<Vec<_>>();

                        if values.iter().any(|e| e.is_some()) {
                            map.serialize_entry(key, &values)?
                        }

                        let elements = sequence
                            .iter()
                            .map(|e| {
                                PrimitiveElement::from_element(e)
                                    .map(|e| self.clone_with(e))
                                    .map(|c| {
                                        if c.value.id.is_some() || !c.value.extension.is_empty() {
                                            Some(c)
                                        } else {
                                            None
                                        }
                                    })
                            })
                            .collect::<Result<Vec<_>, _>>()?;

                        if elements.iter().any(|e| e.is_some()) {
                            map.serialize_entry(&format!("_{}", key), &elements)?
                        }
                    }
                    _ => map.serialize_entry(key, &self.clone_with(value))?,
                }
            } else {
                map.serialize_entry(key, &self.clone_with(value))?;
            }

            self.current_path.borrow_mut().pop();
        }

        if is_resource {
            self.current_path.borrow_mut().pop();
        }

        map.end()
    }
}

#[derive(Clone)]
pub struct PrimitiveElement<'a> {
    pub id: Option<&'a str>,
    pub extension: &'a [Element],
}

impl<'a> PrimitiveElement<'a> {
    fn from_element<E>(value: &'a Element) -> Result<PrimitiveElement<'a>, E>
    where
        E: Error,
    {
        Ok(PrimitiveElement {
            id: value
                .get("id")
                .map(|v| {
                    if let Value::Primitive(Primitive::String(s)) = v {
                        Ok(s.as_str())
                    } else {
                        Err(E::custom(format!("invalid primitive value: {:?}", v)))
                    }
                })
                .transpose()?,
            extension: value
                .get("extension")
                .map(|v| {
                    if let Value::Sequence(s) = v {
                        Ok(s.as_slice())
                    } else {
                        Err(E::custom(format!("invalid extension: {:?}", v)))
                    }
                })
                .transpose()?
                .unwrap_or(&[]),
        })
    }
}

impl Serialize for SerializationContext<PrimitiveElement<'_>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(if self.value.id.is_some() {
            self.value.extension.len() + 1
        } else {
            self.value.extension.len()
        }))?;

        if let Some(id) = self.value.id {
            map.serialize_entry("id", id)?
        }

        if !self.value.extension.is_empty() {
            self.current_path.borrow_mut().push("extension");

            let elements = self
                .value
                .extension
                .iter()
                .map(|e| self.clone_with(e))
                .collect::<Vec<_>>();

            map.serialize_entry("extension", &elements)?;

            self.current_path.borrow_mut().pop();
        }

        map.end()
    }
}
