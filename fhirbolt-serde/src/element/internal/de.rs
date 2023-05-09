use std::{iter, mem, vec};

use serde::{
    de::{self, DeserializeSeed, MapAccess, SeqAccess, Unexpected, Visitor},
    forward_to_deserialize_any,
};

use fhirbolt_element::{Element, Primitive, Value};
use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::{
    context::{
        de::{CurrentElement, DeserializationContext},
        Format,
    },
    element::{self, Deserializer},
    DeserializationMode,
};

const SERDE_JSON_NUMBER_TOKEN: &str = "$serde_json::private::Number";
const PRIMITIVE_CHILDREN: &[&str] = &["id", "extension", "value"];

#[derive(Default, Debug)]
pub struct InternalElement<const R: FhirRelease>(pub Element<R>);

impl<const R: FhirRelease> InternalElement<R> {
    pub fn into_element<'a, D>(
        self,
        deserialization_mode: DeserializationMode,
        current_path: &mut ElementPath,
    ) -> Result<Element<R>, D::Error>
    where
        D: de::Deserializer<'a>,
    {
        let mut element = self.0;

        resolve_element_types::<D, R>(&mut element, deserialization_mode, current_path)?;

        Ok(element)
    }
}

pub fn resolve_element_types<'a, D, const R: FhirRelease>(
    element: &mut Element<R>,
    deserialization_mode: DeserializationMode,
    current_path: &mut ElementPath,
) -> Result<(), D::Error>
where
    D: de::Deserializer<'a>,
{
    let mut is_resource = false;

    for (key, value) in element.iter_mut() {
        // in case of top-level resource: current_path is empty
        if (current_path.current_element_is_resource() || current_path.is_empty())
            && key == "resourceType"
        {
            if let Value::Primitive(Primitive::String(s)) = value {
                current_path.push(s);
                is_resource = true;
            }
        } else {
            // check if field is valid at current path
            if deserialization_mode == DeserializationMode::Strict {
                validate_field_is_valid::<D>(current_path, key)?;
            }

            current_path.push(key);

            resolve_value_types::<D, R>(value, deserialization_mode, current_path)?;

            if current_path.current_element_is_sequence() {
                if let Value::Element(e) = value {
                    *value = Value::Sequence(vec![mem::take(e)]);
                }
            }

            current_path.pop();
        }
    }

    if is_resource {
        current_path.pop();
    }

    Ok(())
}

fn validate_field_is_valid<'a, D>(current_path: &ElementPath, field: &str) -> Result<(), D::Error>
where
    D: de::Deserializer<'a>,
{
    if current_path.current_element_is_primitive() {
        if !PRIMITIVE_CHILDREN.contains(&field) {
            return Err(de::Error::custom(format_args!(
                "unknown field `{}`, expected one of {:?}",
                field, PRIMITIVE_CHILDREN
            )));
        }
    } else {
        let fields = current_path.children();

        if !fields.map(|s| s.contains(field)).unwrap_or(false) {
            if let Some(expected_fields) = fields {
                return Err(de::Error::custom(format_args!(
                    "unknown field `{}`, expected one of {:?}",
                    field,
                    &expected_fields.iter().collect::<Vec<_>>()
                )));
            } else {
                return Err(de::Error::custom(format_args!(
                    "unknown field `{}`, there are no fields",
                    field
                )));
            }
        }
    }

    Ok(())
}

fn resolve_value_types<'a, D, const R: FhirRelease>(
    value: &mut Value<R>,
    deserialization_mode: DeserializationMode,
    current_path: &mut ElementPath,
) -> Result<(), D::Error>
where
    D: de::Deserializer<'a>,
{
    match value {
        Value::Element(e) => {
            resolve_element_types::<D, R>(e, deserialization_mode, current_path)?;
        }
        Value::Sequence(s) => {
            for element in s {
                resolve_element_types::<D, R>(element, deserialization_mode, current_path)?;
            }
        }
        Value::Primitive(p) => {
            if current_path.parent_element_is_boolean() {
                *p = map_bool::<D>(p)?
            } else if current_path.parent_element_is_integer()
                || current_path.parent_element_is_positive_integer()
                || current_path.parent_element_is_unsigned_integer()
            {
                *p = map_integer::<D>(p)?
            } else if current_path.parent_element_is_integer64() {
                *p = map_integer64::<D>(p)?
            } else if current_path.parent_element_is_decimal() {
                *p = map_decimal::<D>(p)?
            } else {
                *p = map_string::<D>(p)?
            }
        }
    }

    Ok(())
}

fn map_bool<'a, D>(primitive: &Primitive) -> Result<Primitive, D::Error>
where
    D: de::Deserializer<'a>,
{
    let expected = "a boolean";
    match primitive {
        Primitive::Bool(b) => Ok(Primitive::Bool(*b)),
        Primitive::Integer(i) => Err(de::Error::invalid_type(
            Unexpected::Signed((*i).into()),
            &expected,
        )),
        Primitive::Integer64(i) => Err(de::Error::invalid_type(
            Unexpected::Other(&format!("integer `{}`", i)),
            &expected,
        )),
        Primitive::Decimal(s) => {
            return Err(de::Error::invalid_type(
                Unexpected::Other(&format!("decimal `{}`", s)),
                &expected,
            ))
        }
        Primitive::String(s) => {
            Ok(Primitive::Bool(s.parse().map_err(|_| {
                de::Error::invalid_value(Unexpected::Other(s), &expected)
            })?))
        }
    }
}

fn map_integer<'a, D>(primitive: &Primitive) -> Result<Primitive, D::Error>
where
    D: de::Deserializer<'a>,
{
    let expected = "an integer";
    match primitive {
        Primitive::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(*b), &expected)),
        Primitive::Integer(i) => Ok(Primitive::Integer(*i)),
        Primitive::Integer64(i) => Ok(Primitive::Integer(*i as i32)),
        Primitive::Decimal(s) => {
            return Err(de::Error::invalid_type(
                Unexpected::Other(&format!("decimal `{}`", s)),
                &expected,
            ))
        }
        Primitive::String(s) => {
            Ok(Primitive::Integer(s.parse().map_err(|_| {
                de::Error::invalid_value(Unexpected::Other(s), &expected)
            })?))
        }
    }
}

fn map_integer64<'a, D>(primitive: &Primitive) -> Result<Primitive, D::Error>
where
    D: de::Deserializer<'a>,
{
    let expected = "an integer64";
    match primitive {
        Primitive::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(*b), &expected)),
        Primitive::Integer(i) => Ok(Primitive::Integer64((*i).into())),
        Primitive::Integer64(i) => Ok(Primitive::Integer64(*i)),
        Primitive::Decimal(s) => {
            return Err(de::Error::invalid_type(
                Unexpected::Other(&format!("decimal `{}`", s)),
                &expected,
            ))
        }
        Primitive::String(s) => {
            Ok(Primitive::Integer64(s.parse().map_err(|_| {
                de::Error::invalid_value(Unexpected::Other(s), &expected)
            })?))
        }
    }
}

fn map_decimal<'a, D>(primitive: &mut Primitive) -> Result<Primitive, D::Error>
where
    D: de::Deserializer<'a>,
{
    let expected = "a decimal";
    match primitive {
        Primitive::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(*b), &expected)),
        Primitive::Integer(i) => Ok(Primitive::Decimal(i.to_string())),
        Primitive::Integer64(i) => Ok(Primitive::Decimal(i.to_string())),
        Primitive::Decimal(s) | Primitive::String(s) => Ok(Primitive::Decimal(mem::take(s))),
    }
}

fn map_string<'a, D>(primitive: &mut Primitive) -> Result<Primitive, D::Error>
where
    D: de::Deserializer<'a>,
{
    let expected = "a string";
    match primitive {
        Primitive::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(*b), &expected)),
        Primitive::Integer(i) => Err(de::Error::invalid_type(
            Unexpected::Signed((*i).into()),
            &expected,
        )),
        Primitive::Integer64(i) => Err(de::Error::invalid_type(
            Unexpected::Other(&format!("integer `{}`", i)),
            &expected,
        )),
        Primitive::Decimal(s) => {
            return Err(de::Error::invalid_type(
                Unexpected::Other(&format!("decimal `{}`", s)),
                &expected,
            ))
        }
        Primitive::String(s) => Ok(Primitive::String(mem::take(s))),
    }
}

impl<'de, const R: FhirRelease> DeserializeSeed<'de>
    for &mut DeserializationContext<InternalElement<R>>
{
    type Value = InternalElement<R>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        match deserializer.deserialize_any(ValueVisitor(self.transmute()))? {
            Value::Element(e) => Ok(InternalElement(e)),
            Value::Sequence(_) => Err(de::Error::invalid_type(Unexpected::Seq, &"an element")),
            Value::Primitive(_) => Err(de::Error::invalid_type(
                Unexpected::Other("primitive"),
                &"an element",
            )),
        }
    }
}

impl<'de, const R: FhirRelease> DeserializeSeed<'de> for &mut DeserializationContext<Value<R>> {
    type Value = Value<R>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor(self))
    }
}

fn merge_sequences<const R: FhirRelease>(
    left: Vec<Element<R>>,
    right: Vec<Element<R>>,
) -> Vec<Element<R>> {
    let left_iter = left.into_iter().map(Some).chain(iter::repeat(None));
    let right_iter = right.into_iter().map(Some).chain(iter::repeat(None));

    left_iter
        .zip(right_iter)
        .take_while(|(e, n)| e.is_some() || n.is_some())
        .flat_map(|(e, n)| match (e, n) {
            (Some(mut e), Some(n)) => {
                e.extend(n);
                Some(e)
            }
            (Some(e), None) => Some(e),
            (None, Some(n)) => Some(n),
            _ => None,
        })
        .collect()
}

struct ValueVisitor<'a, const R: FhirRelease>(&'a mut DeserializationContext<Value<R>>);

impl<'a, 'de, const R: FhirRelease> Visitor<'de> for ValueVisitor<'a, R> {
    type Value = Value<R>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.from == Format::Json {
            Ok(Value::Element(Element! {
                "value" => Value::Primitive(Primitive::Bool(v)),
            }))
        } else {
            Ok(Value::Primitive(Primitive::Bool(v)))
        }
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.from == Format::Json {
            Ok(Value::Element(Element! {
                "value" =>  Value::Primitive(Primitive::Integer64(v as i64)),
            }))
        } else {
            Ok(Value::Primitive(Primitive::Integer64(v as i64)))
        }
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.from == Format::Json {
            Ok(Value::Element(Element! {
                "value" =>  Value::Primitive(Primitive::Integer64(v)),
            }))
        } else {
            Ok(Value::Primitive(Primitive::Integer64(v)))
        }
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.from == Format::Json {
            let number = serde_json::Number::from_f64(v)
                .ok_or_else(|| de::Error::custom("not a JSON number"))?;

            Ok(Value::Element(Element! {
                "value" =>  Value::Primitive(Primitive::Decimal(number.to_string())),
            }))
        } else {
            Ok(Value::Primitive(Primitive::Decimal(v.to_string())))
        }
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.from == Format::Json {
            if self.0.current_element() == CurrentElement::Id
                || self.0.current_element() == CurrentElement::ExtensionUrl
            {
                Ok(Value::Primitive(Primitive::String(v)))
            } else {
                Ok(Value::Element(Element! {
                    "value" =>  Value::Primitive(Primitive::String(v)),
                }))
            }
        } else {
            Ok(Value::Primitive(Primitive::String(v)))
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
        let mut element = Element::default();

        while let Some(key) = map_access.next_key::<String>()? {
            if key == SERDE_JSON_NUMBER_TOKEN {
                return Ok(Value::Element(Element! {
                    "value" =>  Value::Primitive(Primitive::Decimal(map_access.next_value()?)),
                }));
            }

            let key = if let Some(stripped) = key.strip_prefix('_') {
                stripped.into()
            } else {
                key
            };

            if key == "resourceType"
                && self.0.current_element() != CurrentElement::ExampleScenarioInstance
                && self.0.current_element() != CurrentElement::ConsentProvision
                && self.0.current_element() != CurrentElement::SubscriptionFilterBy
            {
                let value: String = map_access.next_value()?;

                element.insert(key, Value::Primitive(Primitive::String(value)));
            } else {
                self.0.push_current_element(match key.as_str() {
                    "id" => CurrentElement::Id,
                    "instance" => CurrentElement::ExampleScenarioInstance,
                    "provision" => CurrentElement::ConsentProvision,
                    "filterBy" => CurrentElement::SubscriptionFilterBy,
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

                let value = map_access.next_value_seed(self.0.transmute::<Value<R>>())?;
                let existing = element.remove(&key);

                element.insert(
                    key,
                    match (existing, value) {
                        (Some(Value::Element(mut e)), Value::Element(n)) => {
                            if self.0.from == Format::Json {
                                e.extend(n);
                                Value::Element(e)
                            } else {
                                Value::Sequence(vec![e, n])
                            }
                        }
                        (Some(Value::Sequence(ev)), Value::Sequence(nv)) => {
                            Value::Sequence(merge_sequences(ev, nv))
                        }
                        (Some(Value::Sequence(mut es)), Value::Element(n)) => {
                            es.push(n);
                            Value::Sequence(es)
                        }
                        (_e, v) => v,
                    },
                );

                self.0.pop_current_element();
            }
        }

        fn embed_string_in_element<const R: FhirRelease>(value: &mut Value<R>) {
            if let Value::Primitive(Primitive::String(s)) = value {
                *value = Value::Element(Element! {
                    "value" =>  Value::Primitive(Primitive::String(mem::take(s))),
                });
            }
        }

        if element.contains_key("resourceType") {
            if let Some(id) = element.get_mut("id") {
                embed_string_in_element(id)
            }
        }

        Ok(Value::Element(element))
    }

    fn visit_seq<V>(self, mut seq_access: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut elements = Vec::new();

        while let Some(value) =
            seq_access.next_element_seed(self.0.transmute::<Option<Value<R>>>())?
        {
            match value {
                Some(Value::Element(e)) => elements.push(e),
                Some(Value::Sequence(_)) => {
                    return Err(de::Error::invalid_type(
                        Unexpected::Seq,
                        &"a sequence element",
                    ))
                }
                Some(Value::Primitive(_)) => {
                    return Err(de::Error::invalid_type(
                        Unexpected::Seq,
                        &"a sequence element",
                    ))
                }
                None => elements.push(Default::default()),
            }
        }

        Ok(Value::Sequence(elements))
    }
}

impl<'de, const R: FhirRelease> DeserializeSeed<'de>
    for &mut DeserializationContext<Option<Value<R>>>
{
    type Value = Option<Value<R>>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(SeqElementVisitor(self))
    }
}

struct SeqElementVisitor<'a, const R: FhirRelease>(
    &'a mut DeserializationContext<Option<Value<R>>>,
);

impl<'a, 'de, const R: FhirRelease> Visitor<'de> for SeqElementVisitor<'a, R> {
    type Value = Option<Value<R>>;

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
            .transmute::<Value<R>>()
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

impl<'de, const R: FhirRelease> de::Deserializer<'de> for Deserializer<InternalElement<R>> {
    type Error = element::error::Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> element::error::Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Deserializer(self.0 .0).deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum seq identifier ignored_any
    }
}
