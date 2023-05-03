// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::types::Signature;
impl serde::ser::Serialize for SerializationContext<&Signature> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Signature", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if self.value.r#when.id.as_deref() == Some("$invalid") {
                return missing_field_error("when");
            }
            if let Some(some) = self.value.r#when.value.as_ref().map(Ok) {
                state.serialize_entry("when", &some?)?;
            }
            if self.value.r#when.id.is_some() || !self.value.r#when.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#when.id.as_ref(),
                    extension: &self.value.r#when.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_when", ctx)
                })?;
            }
        } else if self.value.r#when.id.as_deref() == Some("$invalid") {
            return missing_field_error("when");
        }
        self.with_context(&self.value.r#when, |ctx| state.serialize_entry("when", ctx))?;
        if self.value.r#who.id.as_deref() == Some("$invalid") {
            return missing_field_error("who");
        }
        self.with_context(&self.value.r#who, |ctx| state.serialize_entry("who", ctx))?;
        if let Some(some) = self.value.r#on_behalf_of.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("onBehalfOf", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#target_format.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("targetFormat", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_targetFormat", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#target_format.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("targetFormat", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#sig_format.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("sigFormat", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sigFormat", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#sig_format.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("sigFormat", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#data.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("data", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_data", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#data.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("data", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Signature>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Signature>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Signature> {
    type Value = Signature;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Signature>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Signature;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Signature")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Signature, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "when")]
                    When,
                    #[serde(rename = "_when")]
                    WhenPrimitiveElement,
                    #[serde(rename = "who")]
                    Who,
                    #[serde(rename = "onBehalfOf")]
                    OnBehalfOf,
                    #[serde(rename = "targetFormat")]
                    TargetFormat,
                    #[serde(rename = "_targetFormat")]
                    TargetFormatPrimitiveElement,
                    #[serde(rename = "sigFormat")]
                    SigFormat,
                    #[serde(rename = "_sigFormat")]
                    SigFormatPrimitiveElement,
                    #[serde(rename = "data")]
                    Data,
                    #[serde(rename = "_data")]
                    DataPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "type",
                            "when",
                            "who",
                            "onBehalfOf",
                            "targetFormat",
                            "sigFormat",
                            "data",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#when: Option<fhirbolt_model::r4::types::Instant> = None;
                let mut r#who: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#on_behalf_of: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#target_format: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#sig_format: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#data: Option<fhirbolt_model::r4::types::Base64Binary> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Coding>,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Coding,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::When => {
                            if self.0.from_json {
                                let some = r#when.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("when"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#when.is_some() {
                                    return Err(serde::de::Error::duplicate_field("when"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Instant,
                                > = self.0.transmute();
                                r#when = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WhenPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#when.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_when"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("when");
                            }
                        }
                        Field::Who => {
                            if r#who.is_some() {
                                return Err(serde::de::Error::duplicate_field("who"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#who = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::OnBehalfOf => {
                            if r#on_behalf_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#on_behalf_of = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::TargetFormat => {
                            if self.0.from_json {
                                let some = r#target_format.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetFormat"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#target_format.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetFormat"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#target_format = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TargetFormatPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#target_format.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_targetFormat"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("targetFormat");
                            }
                        }
                        Field::SigFormat => {
                            if self.0.from_json {
                                let some = r#sig_format.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sigFormat"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#sig_format.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sigFormat"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#sig_format = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SigFormatPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#sig_format.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sigFormat"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sigFormat");
                            }
                        }
                        Field::Data => {
                            if self.0.from_json {
                                let some = r#data.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#data.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Base64Binary,
                                > = self.0.transmute();
                                r#data = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DataPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#data.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_data"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("data");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(Signature {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#when: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#when.unwrap_or(Default::default())
                    } else {
                        r#when.ok_or(serde::de::Error::missing_field("when"))?
                    },
                    r#who: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#who.unwrap_or(Default::default())
                    } else {
                        r#who.ok_or(serde::de::Error::missing_field("who"))?
                    },
                    r#on_behalf_of,
                    r#target_format,
                    r#sig_format,
                    r#data,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Signature>> {
    type Value = Box<Signature>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Signature>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Signature>> {
    type Value = Vec<Signature>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Signature>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Signature>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Signature> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
