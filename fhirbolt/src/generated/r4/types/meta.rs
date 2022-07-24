// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Meta {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#version_id: Option<super::super::types::Id>,
    pub r#last_updated: Option<super::super::types::Instant>,
    pub r#source: Option<super::super::types::Uri>,
    pub r#profile: Vec<super::super::types::Canonical>,
    pub r#security: Vec<Box<super::super::types::Coding>>,
    pub r#tag: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for Meta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#version_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("versionId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_versionId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#last_updated.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lastUpdated", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastUpdated", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#source.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_source", &primitive_element)?;
            }
        }
        if !self.r#profile.is_empty() {
            let values: Vec<_> = self.r#profile.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("profile", &values)?;
            }
            let requires_elements = self
                .r#profile
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#profile
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_profile", &primitive_elements)?;
            }
        }
        if !self.r#security.is_empty() {
            state.serialize_entry("security", &self.r#security)?;
        }
        if !self.r#tag.is_empty() {
            state.serialize_entry("tag", &self.r#tag)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Meta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
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
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Meta;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Meta")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Meta, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#version_id: Option<super::super::types::Id> = None;
                let mut r#last_updated: Option<super::super::types::Instant> = None;
                let mut r#source: Option<super::super::types::Uri> = None;
                let mut r#profile: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#security: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#tag: Option<Vec<Box<super::super::types::Coding>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        Field::VersionId => {
                            let some = r#version_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::VersionIdPrimitiveElement => {
                            let some = r#version_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_versionId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::LastUpdated => {
                            let some = r#last_updated.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LastUpdatedPrimitiveElement => {
                            let some = r#last_updated.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lastUpdated"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Source => {
                            let some = r#source.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SourcePrimitiveElement => {
                            let some = r#source.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Profile => {
                            let values: Vec<_> = map_access.next_value()?;
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
                                vec[i].value = value;
                            }
                        }
                        Field::ProfilePrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
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
                        }
                        Field::Security => {
                            if r#security.is_some() {
                                return Err(serde::de::Error::duplicate_field("security"));
                            }
                            r#security = Some(map_access.next_value()?);
                        }
                        Field::Tag => {
                            if r#tag.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            r#tag = Some(map_access.next_value()?);
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
        deserializer.deserialize_map(Visitor)
    }
}
