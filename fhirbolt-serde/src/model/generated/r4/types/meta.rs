// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::types::Meta;
impl serde::ser::Serialize for SerializationContext<&Meta> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Meta", field
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#version_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("versionId", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_versionId", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#version_id.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("versionId", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#last_updated.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("lastUpdated", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_lastUpdated", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#last_updated.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("lastUpdated", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#source.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("source", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_source", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#source.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("source", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#profile.is_empty() {
                let values = tri!(self
                    .value
                    .r#profile
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("profile", &values));
                }
                let requires_elements = self
                    .value
                    .r#profile
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#profile
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_profile", ctx)));
                }
            }
        } else if !self.value.r#profile.is_empty() {
            tri!(self.with_context(&self.value.r#profile, |ctx| state
                .serialize_entry("profile", ctx)));
        }
        if !self.value.r#security.is_empty() {
            tri!(self.with_context(&self.value.r#security, |ctx| state
                .serialize_entry("security", ctx)));
        }
        if !self.value.r#tag.is_empty() {
            tri!(self.with_context(&self.value.r#tag, |ctx| state.serialize_entry("tag", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Meta>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Meta>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Meta> {
    type Value = Meta;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Meta>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Meta;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Meta")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Meta, V::Error>
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
                    #[serde(rename = "versionId")]
                    VersionId,
                    #[serde(rename = "_versionId")]
                    VersionIdPrimitiveElement,
                    #[serde(rename = "lastUpdated")]
                    LastUpdated,
                    #[serde(rename = "_lastUpdated")]
                    LastUpdatedPrimitiveElement,
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "_source")]
                    SourcePrimitiveElement,
                    #[serde(rename = "profile")]
                    Profile,
                    #[serde(rename = "_profile")]
                    ProfilePrimitiveElement,
                    #[serde(rename = "security")]
                    Security,
                    #[serde(rename = "tag")]
                    Tag,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "versionId",
                            "lastUpdated",
                            "source",
                            "profile",
                            "security",
                            "tag",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#version_id: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#last_updated: Option<fhirbolt_model::r4::types::Instant> = None;
                let mut r#source: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#profile: Option<Vec<fhirbolt_model::r4::types::Canonical>> = None;
                let mut r#security: Option<Vec<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#tag: Option<Vec<fhirbolt_model::r4::types::Coding>> = None;
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
                        Field::VersionId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("versionId"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#version_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("versionId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Id,
                                > = self.0.transmute();
                                r#version_id =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::VersionIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_versionId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("versionId");
                            }
                        }
                        Field::LastUpdated => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#last_updated.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lastUpdated"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#last_updated.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lastUpdated"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Instant,
                                > = self.0.transmute();
                                r#last_updated =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::LastUpdatedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#last_updated.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lastUpdated"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lastUpdated");
                            }
                        }
                        Field::Source => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Uri,
                                > = self.0.transmute();
                                r#source = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SourcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_source"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("source");
                            }
                        }
                        Field::Profile => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#profile.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(values.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != values.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        values.len(),
                                        &"primitive elements length",
                                    ));
                                }
                                if vec.iter().any(|v| v.value.is_some()) {
                                    return Err(serde::de::Error::duplicate_field("profile"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#profile.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Canonical,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ProfilePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#profile.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(elements.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != elements.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        elements.len(),
                                        &"primitive values length",
                                    ));
                                }
                                if vec
                                    .iter()
                                    .any(|e| e.id.is_some() || !e.extension.is_empty())
                                {
                                    return Err(serde::de::Error::duplicate_field("_profile"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("profile");
                            }
                        }
                        Field::Security => {
                            if self.0.from == crate::context::Format::Json {
                                if r#security.is_some() {
                                    return Err(serde::de::Error::duplicate_field("security"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Coding>,
                                > = self.0.transmute();
                                r#security = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#security.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Coding,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Tag => {
                            if self.0.from == crate::context::Format::Json {
                                if r#tag.is_some() {
                                    return Err(serde::de::Error::duplicate_field("tag"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Coding>,
                                > = self.0.transmute();
                                r#tag = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#tag.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Coding,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                Ok(Meta {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#version_id,
                    r#last_updated,
                    r#source,
                    r#profile: r#profile.unwrap_or(vec![]),
                    r#security: r#security.unwrap_or(vec![]),
                    r#tag: r#tag.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Meta>> {
    type Value = Box<Meta>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Meta>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Meta>> {
    type Value = Vec<Meta>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Meta>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Meta>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Meta> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
