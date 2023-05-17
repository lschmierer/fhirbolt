// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::types::UsageContext;
impl serde::ser::Serialize for SerializationContext<&UsageContext> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "UsageContext", field
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
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        } else {
            tri!(self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx)));
        }
        {
            use fhirbolt_model::r4::types::UsageContextValue as _Enum;
            match self.value.r#value {
                _Enum::CodeableConcept(ref value) => {
                    tri!(self.with_context(value, |ctx| state
                        .serialize_entry("valueCodeableConcept", ctx)));
                }
                _Enum::Quantity(ref value) => {
                    tri!(
                        self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))
                    );
                }
                _Enum::Range(ref value) => {
                    tri!(self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx)));
                }
                _Enum::Reference(ref value) => {
                    tri!(self
                        .with_context(value, |ctx| state.serialize_entry("valueReference", ctx)));
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<UsageContext>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<UsageContext>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<UsageContext> {
    type Value = UsageContext;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<UsageContext>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = UsageContext;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("UsageContext")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<UsageContext, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueRange")]
                    ValueRange,
                    #[serde(rename = "valueReference")]
                    ValueReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "code",
                            "valueCodeableConcept",
                            "valueQuantity",
                            "valueRange",
                            "valueReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#value: Option<fhirbolt_model::r4::types::UsageContextValue> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Coding>,
                            > = self.0.transmute();
                            r#code = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ValueCodeableConcept => {
                            use fhirbolt_model::r4::types::UsageContextValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::CodeableConcept(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::ValueQuantity => {
                            use fhirbolt_model::r4::types::UsageContextValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Quantity(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::ValueRange => {
                            use fhirbolt_model::r4::types::UsageContextValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::ValueReference => {
                            use fhirbolt_model::r4::types::UsageContextValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Reference(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(UsageContext {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        tri!(r#code.ok_or(serde::de::Error::missing_field("code")))
                    },
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        tri!(r#value.ok_or(serde::de::Error::missing_field("value[x]")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<UsageContext>> {
    type Value = Box<UsageContext>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<UsageContext>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<UsageContext>> {
    type Value = Vec<UsageContext>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<UsageContext>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<UsageContext>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<UsageContext> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
