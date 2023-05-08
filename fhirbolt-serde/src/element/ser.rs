//! Serialize FHIR resources as generic element.

use std::{str::FromStr, vec};

use serde::{
    ser::{self, Error, Impossible, SerializeMap, SerializeSeq, SerializeStruct},
    Serialize,
};

use fhirbolt_element::{Element, Primitive, Value};
use fhirbolt_shared::FhirRelease;

use crate::{
    context::{ser::SerializationContext, Format},
    decimal,
    element::error,
};

impl<const R: FhirRelease> Serialize for SerializationContext<&Value<R>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self.value {
            Value::Element(element) => self.with_context(element, |ctx| ctx.serialize(serializer)),
            Value::Sequence(sequence) => {
                let mut seq = serializer.serialize_seq(Some(sequence.len()))?;
                for e in sequence {
                    self.with_context(e, |ctx| seq.serialize_element(&ctx))?;
                }
                seq.end()
            }
            Value::Primitive(value) => match value {
                Primitive::Bool(b) => serializer.serialize_bool(*b),
                Primitive::Integer(i) => serializer.serialize_i32(*i),
                Primitive::Integer64(i) => serializer.serialize_str(&i.to_string()),
                Primitive::Decimal(s) => {
                    if self.output == Format::Json {
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

impl<const R: FhirRelease> Serialize for SerializationContext<Vec<Option<&Value<R>>>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut seq_serialzier = serializer.serialize_seq(Some(self.value.len()))?;

        for value in &self.value {
            self.with_context(value.as_deref(), |ctx| {
                seq_serialzier.serialize_element(&ctx)
            })?
        }

        seq_serialzier.end()
    }
}

impl<const R: FhirRelease> Serialize for SerializationContext<Option<&Value<R>>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if let Some(value) = self.value {
            self.with_context(value, |ctx| serializer.serialize_some(&ctx))
        } else {
            serializer.serialize_none()
        }
    }
}

impl<const R: FhirRelease> Serialize for SerializationContext<&Element<R>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.value.len()))?;

        let mut is_resource = false;
        if let Some(Value::Primitive(Primitive::String(r))) = self.value.get("resourceType") {
            self.current_path_mut().push(r);
            is_resource = true;
        }

        enum MaybeSorted<'a, const R: FhirRelease> {
            Sorted(vec::IntoIter<(&'a String, &'a Value<R>)>),
            Unsorted(indexmap::map::Iter<'a, String, Value<R>>),
        }

        impl<'a, const R: FhirRelease> Iterator for MaybeSorted<'a, R> {
            type Item = (&'a String, &'a Value<R>);

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    MaybeSorted::Sorted(i) => i.next(),
                    MaybeSorted::Unsorted(i) => i.next(),
                }
            }
        }

        let values = if self.output == Format::Xml || self.config.always_sort {
            let mut values = self.value.iter().collect::<Vec<_>>();

            values.sort_unstable_by_key(|(k, _v)| self.current_path().position_of_child(k));

            MaybeSorted::Sorted(values.into_iter())
        } else {
            MaybeSorted::Unsorted(self.value.iter())
        };

        for (key, value) in values {
            self.current_path_mut().push(key);

            if self.output == Format::Json && self.current_path().current_element_is_primitive() {
                match value {
                    Value::Element(element) => {
                        if let Some(v) = element.get("value") {
                            self.with_context(v, |ctx| map.serialize_entry(key, &ctx))?;
                        }

                        let primitive_element = PrimitiveElement::from_element(element)?;

                        if primitive_element.id.is_some() || !primitive_element.extension.is_empty()
                        {
                            self.with_context(&primitive_element, |ctx| {
                                map.serialize_entry(&format!("_{}", key), &ctx)
                            })?;
                        }
                    }
                    Value::Sequence(sequence) => {
                        let values = sequence.iter().map(|e| e.get("value")).collect::<Vec<_>>();

                        if values.iter().any(|e| e.is_some()) {
                            self.with_context(values, |ctx| map.serialize_entry(key, &ctx))?
                        }

                        let elements = sequence
                            .iter()
                            .map(|e| {
                                PrimitiveElement::from_element(e).map(|v| {
                                    if v.id.is_some() || !v.extension.is_empty() {
                                        Some(v)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;

                        if elements.iter().any(|e| e.is_some()) {
                            self.with_context(
                                elements.iter().map(|e| e.as_ref()).collect::<Vec<_>>(),
                                |ctx| map.serialize_entry(&format!("_{}", key), &ctx),
                            )?
                        }
                    }
                    _ => self.with_context(value, |ctx| map.serialize_entry(key, &ctx))?,
                }
            } else {
                self.with_context(value, |ctx| map.serialize_entry(key, &ctx))?;
            }

            self.current_path_mut().pop();
        }

        if is_resource {
            self.current_path_mut().pop();
        }

        map.end()
    }
}

impl<const R: FhirRelease> Serialize for SerializationContext<Vec<&Element<R>>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut seq_serialzier = serializer.serialize_seq(Some(self.value.len()))?;

        for value in &self.value {
            self.with_context(*value, |ctx| seq_serialzier.serialize_element(&ctx))?
        }

        seq_serialzier.end()
    }
}

#[derive(Clone)]
pub struct PrimitiveElement<'a, const R: FhirRelease> {
    pub id: Option<&'a str>,
    pub extension: &'a [Element<R>],
}

impl<'a, const R: FhirRelease> PrimitiveElement<'a, R> {
    fn from_element<E>(value: &'a Element<R>) -> Result<PrimitiveElement<'a, R>, E>
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

impl<const R: FhirRelease> Serialize for SerializationContext<&PrimitiveElement<'_, R>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
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
            self.current_path_mut().push("extension");

            let elements = self.value.extension.iter().collect::<Vec<_>>();

            self.with_context(elements, |ctx| map.serialize_entry("extension", &ctx))?;

            self.current_path_mut().pop();
        }

        map.end()
    }
}

impl<const R: FhirRelease> Serialize
    for SerializationContext<Vec<Option<&PrimitiveElement<'_, R>>>>
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut seq_serialzier = serializer.serialize_seq(Some(self.value.len()))?;

        for value in &self.value {
            self.with_context(value.as_deref(), |ctx| {
                seq_serialzier.serialize_element(&ctx)
            })?
        }

        seq_serialzier.end()
    }
}

impl<const R: FhirRelease> Serialize for SerializationContext<Option<&PrimitiveElement<'_, R>>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if let Some(value) = self.value {
            self.with_context(value, |ctx| serializer.serialize_some(&ctx))
        } else {
            serializer.serialize_none()
        }
    }
}

pub struct Serializer<const R: FhirRelease>;

impl<const R: FhirRelease> ser::Serializer for Serializer<R> {
    type Ok = Value<R>;
    type Error = error::Error;

    type SerializeSeq = SerializeSequence<R>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = SerializeElement<R>;
    type SerializeStruct = SerializeDecimal<R>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Bool(v)))
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer(v.into())))
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer(v.into())))
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer(v)))
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer64(v)))
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer(v.into())))
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer(v.into())))
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer(v as i32)))
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Integer64(v as i64)))
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Decimal(v.to_string())))
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Decimal(v.to_string())))
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut s = String::new();
        s.push(v);
        Ok(Value::Primitive(Primitive::String(s)))
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::String(v.into())))
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("bytes not supported"))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("options not supported"))
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::custom("options not supported"))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unit not supported"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("unit structs not supported"))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("units not supported"))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::custom("newtype structs not supported"))
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::custom("newtype variants not supported"))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSequence { vec: Vec::new() })
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::custom("tuples not supported"))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::custom("tuple structs not supported"))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::custom("tuple variants not supported"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeElement {
            map: indexmap::IndexMap::new(),
            next_key: None,
        })
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        // Decimal is the only struct that cann occur
        Ok(SerializeDecimal(None))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::custom("struct variants not supported"))
    }
}

pub struct SerializeSequence<const R: FhirRelease> {
    vec: Vec<Element<R>>,
}

impl<const R: FhirRelease> ser::SerializeSeq for SerializeSequence<R> {
    type Ok = Value<R>;
    type Error = error::Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> error::Result<()>
    where
        T: ?Sized + Serialize,
    {
        match value.serialize(Serializer)? {
            Value::Element(e) => self.vec.push(e),
            _ => return Err(Error::custom("sequence can only contain elements")),
        }
        Ok(())
    }

    #[inline]
    fn end(self) -> error::Result<Value<R>> {
        Ok(Value::Sequence(self.vec))
    }
}

pub struct SerializeElement<const R: FhirRelease> {
    map: indexmap::IndexMap<String, Value<R>>,
    next_key: Option<String>,
}

impl<const R: FhirRelease> ser::SerializeMap for SerializeElement<R> {
    type Ok = Value<R>;
    type Error = error::Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> error::Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.next_key = Some(key.serialize(MapKeySerializer)?);
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> error::Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.map
            .insert(self.next_key.take().unwrap(), value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> error::Result<Value<R>> {
        Ok(Value::Element(Element::from_iter(self.map.into_iter())))
    }
}

fn key_must_be_a_string() -> error::Error {
    error::Error::custom("key must be a string")
}

struct MapKeySerializer;

impl serde::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = error::Error;

    type SerializeSeq = Impossible<String, Self::Error>;
    type SerializeTuple = Impossible<String, Self::Error>;
    type SerializeTupleStruct = Impossible<String, Self::Error>;
    type SerializeTupleVariant = Impossible<String, Self::Error>;
    type SerializeMap = Impossible<String, Self::Error>;
    type SerializeStruct = Impossible<String, Self::Error>;
    type SerializeStructVariant = Impossible<String, Self::Error>;

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> error::Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_bool(self, _value: bool) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_f32(self, _value: f32) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_f64(self, _value: f64) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_char(self, value: char) -> error::Result<String> {
        let mut s = String::new();
        s.push(value);
        Ok(s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> error::Result<String> {
        Ok(value.to_string())
    }

    fn serialize_bytes(self, _value: &[u8]) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> error::Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> error::Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T>(self, _value: &T) -> error::Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> error::Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> error::Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> error::Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> error::Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> error::Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> error::Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> error::Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }
}
pub struct SerializeDecimal<const R: FhirRelease>(Option<Value<R>>);

impl<const R: FhirRelease> SerializeStruct for SerializeDecimal<R> {
    type Ok = Value<R>;
    type Error = error::Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if key == decimal::TOKEN {
            self.0 = Some(Value::Primitive(Primitive::Decimal(
                value
                    .serialize(decimal::DecimalStrEmitter::<Self::Error>::new())
                    .map_err(Error::custom)?,
            )));
            Ok(())
        } else {
            Err(Error::custom("expected decimal"))
        }
    }

    fn end(self) -> Result<Value<R>, Self::Error> {
        if let Some(value) = self.0 {
            Ok(value)
        } else {
            Err(Error::custom("expected decimal"))
        }
    }
}
