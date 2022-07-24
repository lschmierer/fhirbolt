// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ImplementationGuideDefinitionResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
    Invalid,
}
impl Default for ImplementationGuideDefinitionResourceExample {
    fn default() -> ImplementationGuideDefinitionResourceExample {
        ImplementationGuideDefinitionResourceExample::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ImplementationGuideDefinitionPageName {
    Url(Box<super::super::types::Url>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ImplementationGuideDefinitionPageName {
    fn default() -> ImplementationGuideDefinitionPageName {
        ImplementationGuideDefinitionPageName::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ImplementationGuideManifestResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
    Invalid,
}
impl Default for ImplementationGuideManifestResourceExample {
    fn default() -> ImplementationGuideManifestResourceExample {
        ImplementationGuideManifestResourceExample::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideDependsOn {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#uri: super::super::types::Canonical,
    pub r#package_id: Option<super::super::types::Id>,
    pub r#version: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ImplementationGuideDependsOn {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#uri.value.as_ref() {
            state.serialize_entry("uri", some)?;
        }
        if self.r#uri.id.is_some() || !self.r#uri.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#uri.id,
                extension: &self.r#uri.extension,
            };
            state.serialize_entry("_uri", &primitive_element)?;
        }
        if let Some(some) = self.r#package_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("packageId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_packageId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#version.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("version", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_version", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideDependsOn {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "uri")]
            Uri,
            #[serde(rename = "_uri")]
            UriPrimitiveElement,
            #[serde(rename = "packageId")]
            PackageId,
            #[serde(rename = "_packageId")]
            PackageIdPrimitiveElement,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideDependsOn;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideDependsOn")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideDependsOn, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#uri: Option<super::super::types::Canonical> = None;
                let mut r#package_id: Option<super::super::types::Id> = None;
                let mut r#version: Option<super::super::types::String> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Uri => {
                            let some = r#uri.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UriPrimitiveElement => {
                            let some = r#uri.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_uri"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PackageId => {
                            let some = r#package_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PackageIdPrimitiveElement => {
                            let some = r#package_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_packageId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Version => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::VersionPrimitiveElement => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImplementationGuideDependsOn {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#uri: r#uri.ok_or(serde::de::Error::missing_field("uri"))?,
                    r#package_id,
                    r#version,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideGlobal {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#profile: super::super::types::Canonical,
}
impl serde::ser::Serialize for ImplementationGuideGlobal {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#profile.value.as_ref() {
            state.serialize_entry("profile", some)?;
        }
        if self.r#profile.id.is_some() || !self.r#profile.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#profile.id,
                extension: &self.r#profile.extension,
            };
            state.serialize_entry("_profile", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideGlobal {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "_profile")]
            ProfilePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideGlobal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideGlobal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImplementationGuideGlobal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#profile: Option<super::super::types::Canonical> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TypePrimitiveElement => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Profile => {
                            let some = r#profile.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ProfilePrimitiveElement => {
                            let some = r#profile.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_profile"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImplementationGuideGlobal {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#profile: r#profile.ok_or(serde::de::Error::missing_field("profile"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideDefinitionGrouping {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#description: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ImplementationGuideDefinitionGrouping {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#name.value.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#name.id,
                extension: &self.r#name.extension,
            };
            state.serialize_entry("_name", &primitive_element)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideDefinitionGrouping {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideDefinitionGrouping;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideDefinitionGrouping")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideDefinitionGrouping, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImplementationGuideDefinitionGrouping {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#description,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideDefinitionResource {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Box<super::super::types::Reference>,
    pub r#fhir_version: Vec<super::super::types::Code>,
    pub r#name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#example: Option<ImplementationGuideDefinitionResourceExample>,
    pub r#grouping_id: Option<super::super::types::Id>,
}
impl serde::ser::Serialize for ImplementationGuideDefinitionResource {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("reference", &self.r#reference)?;
        if !self.r#fhir_version.is_empty() {
            let values: Vec<_> = self.r#fhir_version.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("fhirVersion", &values)?;
            }
            let requires_elements = self
                .r#fhir_version
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#fhir_version
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
                state.serialize_entry("_fhirVersion", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#example.as_ref() {
            match some {
                ImplementationGuideDefinitionResourceExample::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("exampleBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_exampleBoolean", &primitive_element)?;
                    }
                }
                ImplementationGuideDefinitionResourceExample::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("exampleCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_exampleCanonical", &primitive_element)?;
                    }
                }
                ImplementationGuideDefinitionResourceExample::Invalid => {
                    return Err(serde::ser::Error::custom("example is invalid"))
                }
            }
        }
        if let Some(some) = self.r#grouping_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("groupingId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_groupingId", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideDefinitionResource {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "reference")]
            Reference,
            #[serde(rename = "fhirVersion")]
            FhirVersion,
            #[serde(rename = "_fhirVersion")]
            FhirVersionPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "exampleBoolean")]
            ExampleBoolean,
            #[serde(rename = "_exampleBoolean")]
            ExampleBooleanPrimitiveElement,
            #[serde(rename = "exampleCanonical")]
            ExampleCanonical,
            #[serde(rename = "_exampleCanonical")]
            ExampleCanonicalPrimitiveElement,
            #[serde(rename = "groupingId")]
            GroupingId,
            #[serde(rename = "_groupingId")]
            GroupingIdPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideDefinitionResource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideDefinitionResource")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideDefinitionResource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#fhir_version: Option<Vec<super::super::types::Code>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#example: Option<ImplementationGuideDefinitionResourceExample> = None;
                let mut r#grouping_id: Option<super::super::types::Id> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Reference => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        Field::FhirVersion => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#fhir_version.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("fhirVersion"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::FhirVersionPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#fhir_version.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_fhirVersion"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ExampleBoolean => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideDefinitionResourceExample::Boolean(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideDefinitionResourceExample::Boolean(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "exampleBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("example[x]"));
                            }
                        }
                        Field::ExampleBooleanPrimitiveElement => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideDefinitionResourceExample::Boolean(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideDefinitionResourceExample::Boolean(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_exampleBoolean",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_example[x]"));
                            }
                        }
                        Field::ExampleCanonical => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideDefinitionResourceExample::Canonical(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideDefinitionResourceExample::Canonical(
                                variant,
                            ) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "exampleCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("example[x]"));
                            }
                        }
                        Field::ExampleCanonicalPrimitiveElement => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideDefinitionResourceExample::Canonical(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideDefinitionResourceExample::Canonical(
                                variant,
                            ) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_exampleCanonical",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_example[x]"));
                            }
                        }
                        Field::GroupingId => {
                            let some = r#grouping_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupingId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::GroupingIdPrimitiveElement => {
                            let some = r#grouping_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_groupingId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImplementationGuideDefinitionResource {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference: r#reference.ok_or(serde::de::Error::missing_field("reference"))?,
                    r#fhir_version: r#fhir_version.unwrap_or(vec![]),
                    r#name,
                    r#description,
                    r#example,
                    r#grouping_id,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideDefinitionPage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: ImplementationGuideDefinitionPageName,
    pub r#title: super::super::types::String,
    pub r#generation: super::super::types::Code,
    pub r#page: Vec<ImplementationGuideDefinitionPage>,
}
impl serde::ser::Serialize for ImplementationGuideDefinitionPage {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        match self.r#name {
            ImplementationGuideDefinitionPageName::Url(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("nameUrl", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_nameUrl", &primitive_element)?;
                }
            }
            ImplementationGuideDefinitionPageName::Reference(ref value) => {
                state.serialize_entry("nameReference", value)?;
            }
            ImplementationGuideDefinitionPageName::Invalid => {
                return Err(serde::ser::Error::custom("name is a required field"))
            }
        }
        if let Some(some) = self.r#title.value.as_ref() {
            state.serialize_entry("title", some)?;
        }
        if self.r#title.id.is_some() || !self.r#title.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#title.id,
                extension: &self.r#title.extension,
            };
            state.serialize_entry("_title", &primitive_element)?;
        }
        if let Some(some) = self.r#generation.value.as_ref() {
            state.serialize_entry("generation", some)?;
        }
        if self.r#generation.id.is_some() || !self.r#generation.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#generation.id,
                extension: &self.r#generation.extension,
            };
            state.serialize_entry("_generation", &primitive_element)?;
        }
        if !self.r#page.is_empty() {
            state.serialize_entry("page", &self.r#page)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideDefinitionPage {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "nameUrl")]
            NameUrl,
            #[serde(rename = "_nameUrl")]
            NameUrlPrimitiveElement,
            #[serde(rename = "nameReference")]
            NameReference,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "generation")]
            Generation,
            #[serde(rename = "_generation")]
            GenerationPrimitiveElement,
            #[serde(rename = "page")]
            Page,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideDefinitionPage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideDefinitionPage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideDefinitionPage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<ImplementationGuideDefinitionPageName> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#generation: Option<super::super::types::Code> = None;
                let mut r#page: Option<Vec<ImplementationGuideDefinitionPage>> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::NameUrl => {
                            let r#enum = r#name.get_or_insert(
                                ImplementationGuideDefinitionPageName::Url(Default::default()),
                            );
                            if let ImplementationGuideDefinitionPageName::Url(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("nameUrl"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("name[x]"));
                            }
                        }
                        Field::NameUrlPrimitiveElement => {
                            let r#enum = r#name.get_or_insert(
                                ImplementationGuideDefinitionPageName::Url(Default::default()),
                            );
                            if let ImplementationGuideDefinitionPageName::Url(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_nameUrl"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_name[x]"));
                            }
                        }
                        Field::NameReference => {
                            if r#name.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameReference"));
                            }
                            r#name = Some(ImplementationGuideDefinitionPageName::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::Title => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TitlePrimitiveElement => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_title"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Generation => {
                            let some = r#generation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("generation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::GenerationPrimitiveElement => {
                            let some = r#generation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_generation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Page => {
                            if r#page.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            r#page = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ImplementationGuideDefinitionPage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#title: r#title.ok_or(serde::de::Error::missing_field("title"))?,
                    r#generation: r#generation
                        .ok_or(serde::de::Error::missing_field("generation"))?,
                    r#page: r#page.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideDefinitionParameter {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#value: super::super::types::String,
}
impl serde::ser::Serialize for ImplementationGuideDefinitionParameter {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#code.value.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#code.id,
                extension: &self.r#code.extension,
            };
            state.serialize_entry("_code", &primitive_element)?;
        }
        if let Some(some) = self.r#value.value.as_ref() {
            state.serialize_entry("value", some)?;
        }
        if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#value.id,
                extension: &self.r#value.extension,
            };
            state.serialize_entry("_value", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideDefinitionParameter {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideDefinitionParameter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideDefinitionParameter")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideDefinitionParameter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#value: Option<super::super::types::String> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Code => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CodePrimitiveElement => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_code"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Value => {
                            let some = r#value.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ValuePrimitiveElement => {
                            let some = r#value.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_value"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImplementationGuideDefinitionParameter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#value: r#value.ok_or(serde::de::Error::missing_field("value"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideDefinitionTemplate {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#source: super::super::types::String,
    pub r#scope: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ImplementationGuideDefinitionTemplate {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#code.value.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#code.id,
                extension: &self.r#code.extension,
            };
            state.serialize_entry("_code", &primitive_element)?;
        }
        if let Some(some) = self.r#source.value.as_ref() {
            state.serialize_entry("source", some)?;
        }
        if self.r#source.id.is_some() || !self.r#source.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#source.id,
                extension: &self.r#source.extension,
            };
            state.serialize_entry("_source", &primitive_element)?;
        }
        if let Some(some) = self.r#scope.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("scope", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_scope", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideDefinitionTemplate {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "_source")]
            SourcePrimitiveElement,
            #[serde(rename = "scope")]
            Scope,
            #[serde(rename = "_scope")]
            ScopePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideDefinitionTemplate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideDefinitionTemplate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideDefinitionTemplate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#source: Option<super::super::types::String> = None;
                let mut r#scope: Option<super::super::types::String> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Code => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CodePrimitiveElement => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_code"));
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
                        Field::Scope => {
                            let some = r#scope.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ScopePrimitiveElement => {
                            let some = r#scope.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_scope"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImplementationGuideDefinitionTemplate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#source: r#source.ok_or(serde::de::Error::missing_field("source"))?,
                    r#scope,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideDefinition {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#grouping: Vec<ImplementationGuideDefinitionGrouping>,
    pub r#resource: Vec<ImplementationGuideDefinitionResource>,
    pub r#page: Option<ImplementationGuideDefinitionPage>,
    pub r#parameter: Vec<ImplementationGuideDefinitionParameter>,
    pub r#template: Vec<ImplementationGuideDefinitionTemplate>,
}
impl serde::ser::Serialize for ImplementationGuideDefinition {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#grouping.is_empty() {
            state.serialize_entry("grouping", &self.r#grouping)?;
        }
        if !self.r#resource.is_empty() {
            state.serialize_entry("resource", &self.r#resource)?;
        }
        if let Some(some) = self.r#page.as_ref() {
            state.serialize_entry("page", some)?;
        }
        if !self.r#parameter.is_empty() {
            state.serialize_entry("parameter", &self.r#parameter)?;
        }
        if !self.r#template.is_empty() {
            state.serialize_entry("template", &self.r#template)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideDefinition {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "grouping")]
            Grouping,
            #[serde(rename = "resource")]
            Resource,
            #[serde(rename = "page")]
            Page,
            #[serde(rename = "parameter")]
            Parameter,
            #[serde(rename = "template")]
            Template,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideDefinition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#grouping: Option<Vec<ImplementationGuideDefinitionGrouping>> = None;
                let mut r#resource: Option<Vec<ImplementationGuideDefinitionResource>> = None;
                let mut r#page: Option<ImplementationGuideDefinitionPage> = None;
                let mut r#parameter: Option<Vec<ImplementationGuideDefinitionParameter>> = None;
                let mut r#template: Option<Vec<ImplementationGuideDefinitionTemplate>> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Grouping => {
                            if r#grouping.is_some() {
                                return Err(serde::de::Error::duplicate_field("grouping"));
                            }
                            r#grouping = Some(map_access.next_value()?);
                        }
                        Field::Resource => {
                            if r#resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            r#resource = Some(map_access.next_value()?);
                        }
                        Field::Page => {
                            if r#page.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            r#page = Some(map_access.next_value()?);
                        }
                        Field::Parameter => {
                            if r#parameter.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            r#parameter = Some(map_access.next_value()?);
                        }
                        Field::Template => {
                            if r#template.is_some() {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            r#template = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ImplementationGuideDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#grouping: r#grouping.unwrap_or(vec![]),
                    r#resource: r#resource.unwrap_or(vec![]),
                    r#page,
                    r#parameter: r#parameter.unwrap_or(vec![]),
                    r#template: r#template.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideManifestResource {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Box<super::super::types::Reference>,
    pub r#example: Option<ImplementationGuideManifestResourceExample>,
    pub r#relative_path: Option<super::super::types::Url>,
}
impl serde::ser::Serialize for ImplementationGuideManifestResource {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("reference", &self.r#reference)?;
        if let Some(some) = self.r#example.as_ref() {
            match some {
                ImplementationGuideManifestResourceExample::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("exampleBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_exampleBoolean", &primitive_element)?;
                    }
                }
                ImplementationGuideManifestResourceExample::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("exampleCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_exampleCanonical", &primitive_element)?;
                    }
                }
                ImplementationGuideManifestResourceExample::Invalid => {
                    return Err(serde::ser::Error::custom("example is invalid"))
                }
            }
        }
        if let Some(some) = self.r#relative_path.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("relativePath", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_relativePath", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideManifestResource {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "reference")]
            Reference,
            #[serde(rename = "exampleBoolean")]
            ExampleBoolean,
            #[serde(rename = "_exampleBoolean")]
            ExampleBooleanPrimitiveElement,
            #[serde(rename = "exampleCanonical")]
            ExampleCanonical,
            #[serde(rename = "_exampleCanonical")]
            ExampleCanonicalPrimitiveElement,
            #[serde(rename = "relativePath")]
            RelativePath,
            #[serde(rename = "_relativePath")]
            RelativePathPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideManifestResource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideManifestResource")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideManifestResource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#example: Option<ImplementationGuideManifestResourceExample> = None;
                let mut r#relative_path: Option<super::super::types::Url> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Reference => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        Field::ExampleBoolean => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideManifestResourceExample::Boolean(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideManifestResourceExample::Boolean(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "exampleBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("example[x]"));
                            }
                        }
                        Field::ExampleBooleanPrimitiveElement => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideManifestResourceExample::Boolean(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideManifestResourceExample::Boolean(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_exampleBoolean",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_example[x]"));
                            }
                        }
                        Field::ExampleCanonical => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideManifestResourceExample::Canonical(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideManifestResourceExample::Canonical(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "exampleCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("example[x]"));
                            }
                        }
                        Field::ExampleCanonicalPrimitiveElement => {
                            let r#enum = r#example.get_or_insert(
                                ImplementationGuideManifestResourceExample::Canonical(
                                    Default::default(),
                                ),
                            );
                            if let ImplementationGuideManifestResourceExample::Canonical(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_exampleCanonical",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_example[x]"));
                            }
                        }
                        Field::RelativePath => {
                            let some = r#relative_path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("relativePath"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RelativePathPrimitiveElement => {
                            let some = r#relative_path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_relativePath"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImplementationGuideManifestResource {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference: r#reference.ok_or(serde::de::Error::missing_field("reference"))?,
                    r#example,
                    r#relative_path,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideManifestPage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#title: Option<super::super::types::String>,
    pub r#anchor: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for ImplementationGuideManifestPage {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#name.value.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#name.id,
                extension: &self.r#name.extension,
            };
            state.serialize_entry("_name", &primitive_element)?;
        }
        if let Some(some) = self.r#title.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("title", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_title", &primitive_element)?;
            }
        }
        if !self.r#anchor.is_empty() {
            let values: Vec<_> = self.r#anchor.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("anchor", &values)?;
            }
            let requires_elements = self
                .r#anchor
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#anchor
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
                state.serialize_entry("_anchor", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideManifestPage {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "anchor")]
            Anchor,
            #[serde(rename = "_anchor")]
            AnchorPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideManifestPage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideManifestPage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideManifestPage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#anchor: Option<Vec<super::super::types::String>> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Title => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TitlePrimitiveElement => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_title"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Anchor => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#anchor.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("anchor"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::AnchorPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#anchor.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_anchor"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                    }
                }
                Ok(ImplementationGuideManifestPage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#title,
                    r#anchor: r#anchor.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuideManifest {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#rendering: Option<super::super::types::Url>,
    pub r#resource: Vec<ImplementationGuideManifestResource>,
    pub r#page: Vec<ImplementationGuideManifestPage>,
    pub r#image: Vec<super::super::types::String>,
    pub r#other: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for ImplementationGuideManifest {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#rendering.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("rendering", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_rendering", &primitive_element)?;
            }
        }
        if !self.r#resource.is_empty() {
            state.serialize_entry("resource", &self.r#resource)?;
        }
        if !self.r#page.is_empty() {
            state.serialize_entry("page", &self.r#page)?;
        }
        if !self.r#image.is_empty() {
            let values: Vec<_> = self.r#image.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("image", &values)?;
            }
            let requires_elements = self
                .r#image
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#image
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
                state.serialize_entry("_image", &primitive_elements)?;
            }
        }
        if !self.r#other.is_empty() {
            let values: Vec<_> = self.r#other.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("other", &values)?;
            }
            let requires_elements = self
                .r#other
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#other
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
                state.serialize_entry("_other", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuideManifest {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "rendering")]
            Rendering,
            #[serde(rename = "_rendering")]
            RenderingPrimitiveElement,
            #[serde(rename = "resource")]
            Resource,
            #[serde(rename = "page")]
            Page,
            #[serde(rename = "image")]
            Image,
            #[serde(rename = "_image")]
            ImagePrimitiveElement,
            #[serde(rename = "other")]
            Other,
            #[serde(rename = "_other")]
            OtherPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuideManifest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuideManifest")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImplementationGuideManifest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#rendering: Option<super::super::types::Url> = None;
                let mut r#resource: Option<Vec<ImplementationGuideManifestResource>> = None;
                let mut r#page: Option<Vec<ImplementationGuideManifestPage>> = None;
                let mut r#image: Option<Vec<super::super::types::String>> = None;
                let mut r#other: Option<Vec<super::super::types::String>> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Rendering => {
                            let some = r#rendering.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendering"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RenderingPrimitiveElement => {
                            let some = r#rendering.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_rendering"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Resource => {
                            if r#resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            r#resource = Some(map_access.next_value()?);
                        }
                        Field::Page => {
                            if r#page.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            r#page = Some(map_access.next_value()?);
                        }
                        Field::Image => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#image.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ImagePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#image.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_image"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Other => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#other.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("other"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::OtherPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#other.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_other"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                    }
                }
                Ok(ImplementationGuideManifest {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#rendering,
                    r#resource: r#resource.unwrap_or(vec![]),
                    r#page: r#page.unwrap_or(vec![]),
                    r#image: r#image.unwrap_or(vec![]),
                    r#other: r#other.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImplementationGuide {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: super::super::types::Uri,
    pub r#version: Option<super::super::types::String>,
    pub r#name: super::super::types::String,
    pub r#title: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#package_id: super::super::types::Id,
    pub r#license: Option<super::super::types::Code>,
    pub r#fhir_version: Vec<super::super::types::Code>,
    pub r#depends_on: Vec<ImplementationGuideDependsOn>,
    pub r#global: Vec<ImplementationGuideGlobal>,
    pub r#definition: Option<ImplementationGuideDefinition>,
    pub r#manifest: Option<ImplementationGuideManifest>,
}
impl serde::ser::Serialize for ImplementationGuide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ImplementationGuide")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#url.value.as_ref() {
            state.serialize_entry("url", some)?;
        }
        if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#url.id,
                extension: &self.r#url.extension,
            };
            state.serialize_entry("_url", &primitive_element)?;
        }
        if let Some(some) = self.r#version.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("version", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_version", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#name.value.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#name.id,
                extension: &self.r#name.extension,
            };
            state.serialize_entry("_name", &primitive_element)?;
        }
        if let Some(some) = self.r#title.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("title", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_title", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#experimental.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("experimental", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_experimental", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#publisher.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("publisher", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_publisher", &primitive_element)?;
            }
        }
        if !self.r#contact.is_empty() {
            state.serialize_entry("contact", &self.r#contact)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if !self.r#use_context.is_empty() {
            state.serialize_entry("useContext", &self.r#use_context)?;
        }
        if !self.r#jurisdiction.is_empty() {
            state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
        }
        if let Some(some) = self.r#copyright.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("copyright", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_copyright", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#package_id.value.as_ref() {
            state.serialize_entry("packageId", some)?;
        }
        if self.r#package_id.id.is_some() || !self.r#package_id.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#package_id.id,
                extension: &self.r#package_id.extension,
            };
            state.serialize_entry("_packageId", &primitive_element)?;
        }
        if let Some(some) = self.r#license.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("license", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_license", &primitive_element)?;
            }
        }
        if !self.r#fhir_version.is_empty() {
            let values: Vec<_> = self.r#fhir_version.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("fhirVersion", &values)?;
            }
            let requires_elements = self
                .r#fhir_version
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#fhir_version
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
                state.serialize_entry("_fhirVersion", &primitive_elements)?;
            }
        }
        if !self.r#depends_on.is_empty() {
            state.serialize_entry("dependsOn", &self.r#depends_on)?;
        }
        if !self.r#global.is_empty() {
            state.serialize_entry("global", &self.r#global)?;
        }
        if let Some(some) = self.r#definition.as_ref() {
            state.serialize_entry("definition", some)?;
        }
        if let Some(some) = self.r#manifest.as_ref() {
            state.serialize_entry("manifest", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImplementationGuide {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "experimental")]
            Experimental,
            #[serde(rename = "_experimental")]
            ExperimentalPrimitiveElement,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "_publisher")]
            PublisherPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            #[serde(rename = "packageId")]
            PackageId,
            #[serde(rename = "_packageId")]
            PackageIdPrimitiveElement,
            #[serde(rename = "license")]
            License,
            #[serde(rename = "_license")]
            LicensePrimitiveElement,
            #[serde(rename = "fhirVersion")]
            FhirVersion,
            #[serde(rename = "_fhirVersion")]
            FhirVersionPrimitiveElement,
            #[serde(rename = "dependsOn")]
            DependsOn,
            #[serde(rename = "global")]
            Global,
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "manifest")]
            Manifest,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImplementationGuide;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImplementationGuide")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImplementationGuide, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
                let mut r#package_id: Option<super::super::types::Id> = None;
                let mut r#license: Option<super::super::types::Code> = None;
                let mut r#fhir_version: Option<Vec<super::super::types::Code>> = None;
                let mut r#depends_on: Option<Vec<ImplementationGuideDependsOn>> = None;
                let mut r#global: Option<Vec<ImplementationGuideGlobal>> = None;
                let mut r#definition: Option<ImplementationGuideDefinition> = None;
                let mut r#manifest: Option<ImplementationGuideManifest> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRules => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Language => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LanguagePrimitiveElement => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        Field::Contained => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Url => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UrlPrimitiveElement => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_url"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Version => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::VersionPrimitiveElement => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Title => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TitlePrimitiveElement => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_title"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Status => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusPrimitiveElement => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Experimental => {
                            let some = r#experimental.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimental"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ExperimentalPrimitiveElement => {
                            let some = r#experimental.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_experimental"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Date => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DatePrimitiveElement => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_date"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Publisher => {
                            let some = r#publisher.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PublisherPrimitiveElement => {
                            let some = r#publisher.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_publisher"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Contact => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            r#contact = Some(map_access.next_value()?);
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::UseContext => {
                            if r#use_context.is_some() {
                                return Err(serde::de::Error::duplicate_field("useContext"));
                            }
                            r#use_context = Some(map_access.next_value()?);
                        }
                        Field::Jurisdiction => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            r#jurisdiction = Some(map_access.next_value()?);
                        }
                        Field::Copyright => {
                            let some = r#copyright.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("copyright"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CopyrightPrimitiveElement => {
                            let some = r#copyright.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_copyright"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PackageId => {
                            let some = r#package_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PackageIdPrimitiveElement => {
                            let some = r#package_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_packageId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::License => {
                            let some = r#license.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("license"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LicensePrimitiveElement => {
                            let some = r#license.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_license"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::FhirVersion => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#fhir_version.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("fhirVersion"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::FhirVersionPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#fhir_version.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_fhirVersion"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::DependsOn => {
                            if r#depends_on.is_some() {
                                return Err(serde::de::Error::duplicate_field("dependsOn"));
                            }
                            r#depends_on = Some(map_access.next_value()?);
                        }
                        Field::Global => {
                            if r#global.is_some() {
                                return Err(serde::de::Error::duplicate_field("global"));
                            }
                            r#global = Some(map_access.next_value()?);
                        }
                        Field::Definition => {
                            if r#definition.is_some() {
                                return Err(serde::de::Error::duplicate_field("definition"));
                            }
                            r#definition = Some(map_access.next_value()?);
                        }
                        Field::Manifest => {
                            if r#manifest.is_some() {
                                return Err(serde::de::Error::duplicate_field("manifest"));
                            }
                            r#manifest = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ImplementationGuide {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url: r#url.ok_or(serde::de::Error::missing_field("url"))?,
                    r#version,
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#title,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#experimental,
                    r#date,
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#description,
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                    r#copyright,
                    r#package_id: r#package_id
                        .ok_or(serde::de::Error::missing_field("package_id"))?,
                    r#license,
                    r#fhir_version: r#fhir_version.unwrap_or(vec![]),
                    r#depends_on: r#depends_on.unwrap_or(vec![]),
                    r#global: r#global.unwrap_or(vec![]),
                    r#definition,
                    r#manifest,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
