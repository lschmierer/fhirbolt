// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::types::Ratio;
impl serde::ser::Serialize for SerializationContext<&Ratio> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Ratio", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if let Some(some) = self.value.r#numerator.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("numerator", ctx)));
        }
        if let Some(some) = self.value.r#denominator.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("denominator", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Ratio>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Ratio>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Ratio> {
    type Value = Ratio;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Ratio>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Ratio;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Ratio")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Ratio, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "numerator")]
                    Numerator,
                    #[serde(rename = "denominator")]
                    Denominator,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "numerator", "denominator"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#numerator: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#denominator: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Numerator => {
                            if r#numerator.is_some() {
                                return Err(serde::de::Error::duplicate_field("numerator"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#numerator = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Denominator => {
                            if r#denominator.is_some() {
                                return Err(serde::de::Error::duplicate_field("denominator"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#denominator = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(Ratio {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#numerator,
                    r#denominator,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Ratio>> {
    type Value = Box<Ratio>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Ratio>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Ratio>> {
    type Value = Vec<Ratio>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Ratio>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Ratio>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Ratio> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
