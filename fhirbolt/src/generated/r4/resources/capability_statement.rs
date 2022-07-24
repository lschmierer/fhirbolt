// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementSoftware {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#version: Option<super::super::types::String>,
    pub r#release_date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for CapabilityStatementSoftware {
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
        if let Some(some) = self.r#release_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("releaseDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_releaseDate", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementSoftware {
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
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "releaseDate")]
            ReleaseDate,
            #[serde(rename = "_releaseDate")]
            ReleaseDatePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementSoftware;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementSoftware")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementSoftware, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#release_date: Option<super::super::types::DateTime> = None;
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
                        Field::ReleaseDate => {
                            let some = r#release_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("releaseDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ReleaseDatePrimitiveElement => {
                            let some = r#release_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_releaseDate"));
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
                Ok(CapabilityStatementSoftware {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#version,
                    r#release_date,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementImplementation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: super::super::types::String,
    pub r#url: Option<super::super::types::Url>,
    pub r#custodian: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CapabilityStatementImplementation {
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
        if let Some(some) = self.r#description.value.as_ref() {
            state.serialize_entry("description", some)?;
        }
        if self.r#description.id.is_some() || !self.r#description.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#description.id,
                extension: &self.r#description.extension,
            };
            state.serialize_entry("_description", &primitive_element)?;
        }
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("url", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_url", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#custodian.as_ref() {
            state.serialize_entry("custodian", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementImplementation {
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
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "custodian")]
            Custodian,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementImplementation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementImplementation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementImplementation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#url: Option<super::super::types::Url> = None;
                let mut r#custodian: Option<Box<super::super::types::Reference>> = None;
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
                        Field::Custodian => {
                            if r#custodian.is_some() {
                                return Err(serde::de::Error::duplicate_field("custodian"));
                            }
                            r#custodian = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(CapabilityStatementImplementation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#description: r#description
                        .ok_or(serde::de::Error::missing_field("description"))?,
                    r#url,
                    r#custodian,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestSecurity {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#cors: Option<super::super::types::Boolean>,
    pub r#service: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestSecurity {
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
        if let Some(some) = self.r#cors.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("cors", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_cors", &primitive_element)?;
            }
        }
        if !self.r#service.is_empty() {
            state.serialize_entry("service", &self.r#service)?;
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
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementRestSecurity {
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
            #[serde(rename = "cors")]
            Cors,
            #[serde(rename = "_cors")]
            CorsPrimitiveElement,
            #[serde(rename = "service")]
            Service,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementRestSecurity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementRestSecurity")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementRestSecurity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#cors: Option<super::super::types::Boolean> = None;
                let mut r#service: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
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
                        Field::Cors => {
                            let some = r#cors.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("cors"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CorsPrimitiveElement => {
                            let some = r#cors.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_cors"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Service => {
                            if r#service.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            r#service = Some(map_access.next_value()?);
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
                Ok(CapabilityStatementRestSecurity {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#cors,
                    r#service: r#service.unwrap_or(vec![]),
                    r#description,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResourceInteraction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestResourceInteraction {
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
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementRestResourceInteraction {
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
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementRestResourceInteraction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementRestResourceInteraction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementRestResourceInteraction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
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
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
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
                Ok(CapabilityStatementRestResourceInteraction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#documentation,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResourceSearchParam {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#definition: Option<super::super::types::Canonical>,
    pub r#type: super::super::types::Code,
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestResourceSearchParam {
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
        if let Some(some) = self.r#definition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("definition", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_definition", &primitive_element)?;
            }
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
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementRestResourceSearchParam {
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
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "_definition")]
            DefinitionPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementRestResourceSearchParam;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementRestResourceSearchParam")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementRestResourceSearchParam, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#definition: Option<super::super::types::Canonical> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
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
                        Field::Definition => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("definition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DefinitionPrimitiveElement => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_definition"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
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
                Ok(CapabilityStatementRestResourceSearchParam {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#definition,
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#documentation,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResourceOperation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#definition: super::super::types::Canonical,
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestResourceOperation {
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
        if let Some(some) = self.r#definition.value.as_ref() {
            state.serialize_entry("definition", some)?;
        }
        if self.r#definition.id.is_some() || !self.r#definition.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#definition.id,
                extension: &self.r#definition.extension,
            };
            state.serialize_entry("_definition", &primitive_element)?;
        }
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementRestResourceOperation {
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
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "_definition")]
            DefinitionPrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementRestResourceOperation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementRestResourceOperation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementRestResourceOperation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#definition: Option<super::super::types::Canonical> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
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
                        Field::Definition => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("definition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DefinitionPrimitiveElement => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_definition"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
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
                Ok(CapabilityStatementRestResourceOperation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#definition: r#definition
                        .ok_or(serde::de::Error::missing_field("definition"))?,
                    r#documentation,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResource {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#supported_profile: Vec<super::super::types::Canonical>,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#interaction: Vec<CapabilityStatementRestResourceInteraction>,
    pub r#versioning: Option<super::super::types::Code>,
    pub r#read_history: Option<super::super::types::Boolean>,
    pub r#update_create: Option<super::super::types::Boolean>,
    pub r#conditional_create: Option<super::super::types::Boolean>,
    pub r#conditional_read: Option<super::super::types::Code>,
    pub r#conditional_update: Option<super::super::types::Boolean>,
    pub r#conditional_delete: Option<super::super::types::Code>,
    pub r#reference_policy: Vec<super::super::types::Code>,
    pub r#search_include: Vec<super::super::types::String>,
    pub r#search_rev_include: Vec<super::super::types::String>,
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
}
impl serde::ser::Serialize for CapabilityStatementRestResource {
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
        if let Some(some) = self.r#profile.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("profile", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_profile", &primitive_element)?;
            }
        }
        if !self.r#supported_profile.is_empty() {
            let values: Vec<_> = self.r#supported_profile.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("supportedProfile", &values)?;
            }
            let requires_elements = self
                .r#supported_profile
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#supported_profile
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
                state.serialize_entry("_supportedProfile", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
        }
        if !self.r#interaction.is_empty() {
            state.serialize_entry("interaction", &self.r#interaction)?;
        }
        if let Some(some) = self.r#versioning.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("versioning", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_versioning", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#read_history.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("readHistory", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_readHistory", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#update_create.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("updateCreate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_updateCreate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#conditional_create.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("conditionalCreate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_conditionalCreate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#conditional_read.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("conditionalRead", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_conditionalRead", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#conditional_update.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("conditionalUpdate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_conditionalUpdate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#conditional_delete.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("conditionalDelete", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_conditionalDelete", &primitive_element)?;
            }
        }
        if !self.r#reference_policy.is_empty() {
            let values: Vec<_> = self.r#reference_policy.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("referencePolicy", &values)?;
            }
            let requires_elements = self
                .r#reference_policy
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#reference_policy
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
                state.serialize_entry("_referencePolicy", &primitive_elements)?;
            }
        }
        if !self.r#search_include.is_empty() {
            let values: Vec<_> = self.r#search_include.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("searchInclude", &values)?;
            }
            let requires_elements = self
                .r#search_include
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#search_include
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
                state.serialize_entry("_searchInclude", &primitive_elements)?;
            }
        }
        if !self.r#search_rev_include.is_empty() {
            let values: Vec<_> = self.r#search_rev_include.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("searchRevInclude", &values)?;
            }
            let requires_elements = self
                .r#search_rev_include
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#search_rev_include
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
                state.serialize_entry("_searchRevInclude", &primitive_elements)?;
            }
        }
        if !self.r#search_param.is_empty() {
            state.serialize_entry("searchParam", &self.r#search_param)?;
        }
        if !self.r#operation.is_empty() {
            state.serialize_entry("operation", &self.r#operation)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementRestResource {
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
            #[serde(rename = "supportedProfile")]
            SupportedProfile,
            #[serde(rename = "_supportedProfile")]
            SupportedProfilePrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            #[serde(rename = "interaction")]
            Interaction,
            #[serde(rename = "versioning")]
            Versioning,
            #[serde(rename = "_versioning")]
            VersioningPrimitiveElement,
            #[serde(rename = "readHistory")]
            ReadHistory,
            #[serde(rename = "_readHistory")]
            ReadHistoryPrimitiveElement,
            #[serde(rename = "updateCreate")]
            UpdateCreate,
            #[serde(rename = "_updateCreate")]
            UpdateCreatePrimitiveElement,
            #[serde(rename = "conditionalCreate")]
            ConditionalCreate,
            #[serde(rename = "_conditionalCreate")]
            ConditionalCreatePrimitiveElement,
            #[serde(rename = "conditionalRead")]
            ConditionalRead,
            #[serde(rename = "_conditionalRead")]
            ConditionalReadPrimitiveElement,
            #[serde(rename = "conditionalUpdate")]
            ConditionalUpdate,
            #[serde(rename = "_conditionalUpdate")]
            ConditionalUpdatePrimitiveElement,
            #[serde(rename = "conditionalDelete")]
            ConditionalDelete,
            #[serde(rename = "_conditionalDelete")]
            ConditionalDeletePrimitiveElement,
            #[serde(rename = "referencePolicy")]
            ReferencePolicy,
            #[serde(rename = "_referencePolicy")]
            ReferencePolicyPrimitiveElement,
            #[serde(rename = "searchInclude")]
            SearchInclude,
            #[serde(rename = "_searchInclude")]
            SearchIncludePrimitiveElement,
            #[serde(rename = "searchRevInclude")]
            SearchRevInclude,
            #[serde(rename = "_searchRevInclude")]
            SearchRevIncludePrimitiveElement,
            #[serde(rename = "searchParam")]
            SearchParam,
            #[serde(rename = "operation")]
            Operation,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementRestResource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementRestResource")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementRestResource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#profile: Option<super::super::types::Canonical> = None;
                let mut r#supported_profile: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
                let mut r#interaction: Option<Vec<CapabilityStatementRestResourceInteraction>> =
                    None;
                let mut r#versioning: Option<super::super::types::Code> = None;
                let mut r#read_history: Option<super::super::types::Boolean> = None;
                let mut r#update_create: Option<super::super::types::Boolean> = None;
                let mut r#conditional_create: Option<super::super::types::Boolean> = None;
                let mut r#conditional_read: Option<super::super::types::Code> = None;
                let mut r#conditional_update: Option<super::super::types::Boolean> = None;
                let mut r#conditional_delete: Option<super::super::types::Code> = None;
                let mut r#reference_policy: Option<Vec<super::super::types::Code>> = None;
                let mut r#search_include: Option<Vec<super::super::types::String>> = None;
                let mut r#search_rev_include: Option<Vec<super::super::types::String>> = None;
                let mut r#search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>> =
                    None;
                let mut r#operation: Option<Vec<CapabilityStatementRestResourceOperation>> = None;
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
                        Field::SupportedProfile => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#supported_profile.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("supportedProfile"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::SupportedProfilePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#supported_profile
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_supportedProfile"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Interaction => {
                            if r#interaction.is_some() {
                                return Err(serde::de::Error::duplicate_field("interaction"));
                            }
                            r#interaction = Some(map_access.next_value()?);
                        }
                        Field::Versioning => {
                            let some = r#versioning.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("versioning"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::VersioningPrimitiveElement => {
                            let some = r#versioning.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_versioning"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ReadHistory => {
                            let some = r#read_history.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("readHistory"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ReadHistoryPrimitiveElement => {
                            let some = r#read_history.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_readHistory"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::UpdateCreate => {
                            let some = r#update_create.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateCreate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UpdateCreatePrimitiveElement => {
                            let some = r#update_create.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_updateCreate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ConditionalCreate => {
                            let some = r#conditional_create.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionalCreate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ConditionalCreatePrimitiveElement => {
                            let some = r#conditional_create.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_conditionalCreate",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ConditionalRead => {
                            let some = r#conditional_read.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionalRead"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ConditionalReadPrimitiveElement => {
                            let some = r#conditional_read.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_conditionalRead"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ConditionalUpdate => {
                            let some = r#conditional_update.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionalUpdate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ConditionalUpdatePrimitiveElement => {
                            let some = r#conditional_update.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_conditionalUpdate",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ConditionalDelete => {
                            let some = r#conditional_delete.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionalDelete"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ConditionalDeletePrimitiveElement => {
                            let some = r#conditional_delete.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_conditionalDelete",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ReferencePolicy => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#reference_policy.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("referencePolicy"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ReferencePolicyPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#reference_policy
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_referencePolicy"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::SearchInclude => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#search_include.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("searchInclude"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::SearchIncludePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#search_include.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_searchInclude"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::SearchRevInclude => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#search_rev_include
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("searchRevInclude"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::SearchRevIncludePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#search_rev_include
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_searchRevInclude"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::SearchParam => {
                            if r#search_param.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchParam"));
                            }
                            r#search_param = Some(map_access.next_value()?);
                        }
                        Field::Operation => {
                            if r#operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            r#operation = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(CapabilityStatementRestResource {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#profile,
                    r#supported_profile: r#supported_profile.unwrap_or(vec![]),
                    r#documentation,
                    r#interaction: r#interaction.unwrap_or(vec![]),
                    r#versioning,
                    r#read_history,
                    r#update_create,
                    r#conditional_create,
                    r#conditional_read,
                    r#conditional_update,
                    r#conditional_delete,
                    r#reference_policy: r#reference_policy.unwrap_or(vec![]),
                    r#search_include: r#search_include.unwrap_or(vec![]),
                    r#search_rev_include: r#search_rev_include.unwrap_or(vec![]),
                    r#search_param: r#search_param.unwrap_or(vec![]),
                    r#operation: r#operation.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestInteraction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestInteraction {
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
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementRestInteraction {
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
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementRestInteraction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementRestInteraction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementRestInteraction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
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
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
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
                Ok(CapabilityStatementRestInteraction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#documentation,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRest {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: super::super::types::Code,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#security: Option<CapabilityStatementRestSecurity>,
    pub r#resource: Vec<CapabilityStatementRestResource>,
    pub r#interaction: Vec<CapabilityStatementRestInteraction>,
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
    pub r#compartment: Vec<super::super::types::Canonical>,
}
impl serde::ser::Serialize for CapabilityStatementRest {
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
        if let Some(some) = self.r#mode.value.as_ref() {
            state.serialize_entry("mode", some)?;
        }
        if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#mode.id,
                extension: &self.r#mode.extension,
            };
            state.serialize_entry("_mode", &primitive_element)?;
        }
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#security.as_ref() {
            state.serialize_entry("security", some)?;
        }
        if !self.r#resource.is_empty() {
            state.serialize_entry("resource", &self.r#resource)?;
        }
        if !self.r#interaction.is_empty() {
            state.serialize_entry("interaction", &self.r#interaction)?;
        }
        if !self.r#search_param.is_empty() {
            state.serialize_entry("searchParam", &self.r#search_param)?;
        }
        if !self.r#operation.is_empty() {
            state.serialize_entry("operation", &self.r#operation)?;
        }
        if !self.r#compartment.is_empty() {
            let values: Vec<_> = self.r#compartment.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("compartment", &values)?;
            }
            let requires_elements = self
                .r#compartment
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#compartment
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
                state.serialize_entry("_compartment", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementRest {
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
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            #[serde(rename = "security")]
            Security,
            #[serde(rename = "resource")]
            Resource,
            #[serde(rename = "interaction")]
            Interaction,
            #[serde(rename = "searchParam")]
            SearchParam,
            #[serde(rename = "operation")]
            Operation,
            #[serde(rename = "compartment")]
            Compartment,
            #[serde(rename = "_compartment")]
            CompartmentPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementRest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementRest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CapabilityStatementRest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
                let mut r#security: Option<CapabilityStatementRestSecurity> = None;
                let mut r#resource: Option<Vec<CapabilityStatementRestResource>> = None;
                let mut r#interaction: Option<Vec<CapabilityStatementRestInteraction>> = None;
                let mut r#search_param: Option<Vec<CapabilityStatementRestResourceSearchParam>> =
                    None;
                let mut r#operation: Option<Vec<CapabilityStatementRestResourceOperation>> = None;
                let mut r#compartment: Option<Vec<super::super::types::Canonical>> = None;
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
                        Field::Mode => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ModePrimitiveElement => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_mode"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Security => {
                            if r#security.is_some() {
                                return Err(serde::de::Error::duplicate_field("security"));
                            }
                            r#security = Some(map_access.next_value()?);
                        }
                        Field::Resource => {
                            if r#resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            r#resource = Some(map_access.next_value()?);
                        }
                        Field::Interaction => {
                            if r#interaction.is_some() {
                                return Err(serde::de::Error::duplicate_field("interaction"));
                            }
                            r#interaction = Some(map_access.next_value()?);
                        }
                        Field::SearchParam => {
                            if r#search_param.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchParam"));
                            }
                            r#search_param = Some(map_access.next_value()?);
                        }
                        Field::Operation => {
                            if r#operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            r#operation = Some(map_access.next_value()?);
                        }
                        Field::Compartment => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#compartment.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("compartment"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::CompartmentPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#compartment.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_compartment"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                    }
                }
                Ok(CapabilityStatementRest {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#mode: r#mode.ok_or(serde::de::Error::missing_field("mode"))?,
                    r#documentation,
                    r#security,
                    r#resource: r#resource.unwrap_or(vec![]),
                    r#interaction: r#interaction.unwrap_or(vec![]),
                    r#search_param: r#search_param.unwrap_or(vec![]),
                    r#operation: r#operation.unwrap_or(vec![]),
                    r#compartment: r#compartment.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementMessagingEndpoint {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#protocol: Box<super::super::types::Coding>,
    pub r#address: super::super::types::Url,
}
impl serde::ser::Serialize for CapabilityStatementMessagingEndpoint {
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
        state.serialize_entry("protocol", &self.r#protocol)?;
        if let Some(some) = self.r#address.value.as_ref() {
            state.serialize_entry("address", some)?;
        }
        if self.r#address.id.is_some() || !self.r#address.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#address.id,
                extension: &self.r#address.extension,
            };
            state.serialize_entry("_address", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementMessagingEndpoint {
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
            #[serde(rename = "protocol")]
            Protocol,
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "_address")]
            AddressPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementMessagingEndpoint;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementMessagingEndpoint")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementMessagingEndpoint, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#protocol: Option<Box<super::super::types::Coding>> = None;
                let mut r#address: Option<super::super::types::Url> = None;
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
                        Field::Protocol => {
                            if r#protocol.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            r#protocol = Some(map_access.next_value()?);
                        }
                        Field::Address => {
                            let some = r#address.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AddressPrimitiveElement => {
                            let some = r#address.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_address"));
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
                Ok(CapabilityStatementMessagingEndpoint {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#protocol: r#protocol.ok_or(serde::de::Error::missing_field("protocol"))?,
                    r#address: r#address.ok_or(serde::de::Error::missing_field("address"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementMessagingSupportedMessage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: super::super::types::Code,
    pub r#definition: super::super::types::Canonical,
}
impl serde::ser::Serialize for CapabilityStatementMessagingSupportedMessage {
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
        if let Some(some) = self.r#mode.value.as_ref() {
            state.serialize_entry("mode", some)?;
        }
        if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#mode.id,
                extension: &self.r#mode.extension,
            };
            state.serialize_entry("_mode", &primitive_element)?;
        }
        if let Some(some) = self.r#definition.value.as_ref() {
            state.serialize_entry("definition", some)?;
        }
        if self.r#definition.id.is_some() || !self.r#definition.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#definition.id,
                extension: &self.r#definition.extension,
            };
            state.serialize_entry("_definition", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementMessagingSupportedMessage {
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
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "_definition")]
            DefinitionPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementMessagingSupportedMessage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementMessagingSupportedMessage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementMessagingSupportedMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#definition: Option<super::super::types::Canonical> = None;
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
                        Field::Mode => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ModePrimitiveElement => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_mode"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Definition => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("definition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DefinitionPrimitiveElement => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_definition"));
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
                Ok(CapabilityStatementMessagingSupportedMessage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#mode: r#mode.ok_or(serde::de::Error::missing_field("mode"))?,
                    r#definition: r#definition
                        .ok_or(serde::de::Error::missing_field("definition"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementMessaging {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#endpoint: Vec<CapabilityStatementMessagingEndpoint>,
    pub r#reliable_cache: Option<super::super::types::UnsignedInt>,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#supported_message: Vec<CapabilityStatementMessagingSupportedMessage>,
}
impl serde::ser::Serialize for CapabilityStatementMessaging {
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
        if !self.r#endpoint.is_empty() {
            state.serialize_entry("endpoint", &self.r#endpoint)?;
        }
        if let Some(some) = self.r#reliable_cache.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("reliableCache", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_reliableCache", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
        }
        if !self.r#supported_message.is_empty() {
            state.serialize_entry("supportedMessage", &self.r#supported_message)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementMessaging {
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
            #[serde(rename = "endpoint")]
            Endpoint,
            #[serde(rename = "reliableCache")]
            ReliableCache,
            #[serde(rename = "_reliableCache")]
            ReliableCachePrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            #[serde(rename = "supportedMessage")]
            SupportedMessage,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementMessaging;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementMessaging")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementMessaging, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#endpoint: Option<Vec<CapabilityStatementMessagingEndpoint>> = None;
                let mut r#reliable_cache: Option<super::super::types::UnsignedInt> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
                let mut r#supported_message: Option<
                    Vec<CapabilityStatementMessagingSupportedMessage>,
                > = None;
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
                        Field::Endpoint => {
                            if r#endpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            r#endpoint = Some(map_access.next_value()?);
                        }
                        Field::ReliableCache => {
                            let some = r#reliable_cache.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("reliableCache"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ReliableCachePrimitiveElement => {
                            let some = r#reliable_cache.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_reliableCache"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::SupportedMessage => {
                            if r#supported_message.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportedMessage"));
                            }
                            r#supported_message = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(CapabilityStatementMessaging {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                    r#reliable_cache,
                    r#documentation,
                    r#supported_message: r#supported_message.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementDocument {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: super::super::types::Code,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#profile: super::super::types::Canonical,
}
impl serde::ser::Serialize for CapabilityStatementDocument {
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
        if let Some(some) = self.r#mode.value.as_ref() {
            state.serialize_entry("mode", some)?;
        }
        if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#mode.id,
                extension: &self.r#mode.extension,
            };
            state.serialize_entry("_mode", &primitive_element)?;
        }
        if let Some(some) = self.r#documentation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentation", &primitive_element)?;
            }
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
impl<'de> serde::de::Deserialize<'de> for CapabilityStatementDocument {
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
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "_profile")]
            ProfilePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatementDocument;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatementDocument")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CapabilityStatementDocument, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#documentation: Option<super::super::types::Markdown> = None;
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
                        Field::Mode => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ModePrimitiveElement => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_mode"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Documentation => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentationPrimitiveElement => {
                            let some = r#documentation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentation"));
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
                Ok(CapabilityStatementDocument {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#mode: r#mode.ok_or(serde::de::Error::missing_field("mode"))?,
                    r#documentation,
                    r#profile: r#profile.ok_or(serde::de::Error::missing_field("profile"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatement {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#version: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#title: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#date: super::super::types::DateTime,
    pub r#publisher: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#kind: super::super::types::Code,
    pub r#instantiates: Vec<super::super::types::Canonical>,
    pub r#imports: Vec<super::super::types::Canonical>,
    pub r#software: Option<CapabilityStatementSoftware>,
    pub r#implementation: Option<CapabilityStatementImplementation>,
    pub r#fhir_version: super::super::types::Code,
    pub r#format: Vec<super::super::types::Code>,
    pub r#patch_format: Vec<super::super::types::Code>,
    pub r#implementation_guide: Vec<super::super::types::Canonical>,
    pub r#rest: Vec<CapabilityStatementRest>,
    pub r#messaging: Vec<CapabilityStatementMessaging>,
    pub r#document: Vec<CapabilityStatementDocument>,
}
impl serde::ser::Serialize for CapabilityStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "CapabilityStatement")?;
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
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("url", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_url", &primitive_element)?;
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
        if let Some(some) = self.r#date.value.as_ref() {
            state.serialize_entry("date", some)?;
        }
        if self.r#date.id.is_some() || !self.r#date.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#date.id,
                extension: &self.r#date.extension,
            };
            state.serialize_entry("_date", &primitive_element)?;
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
        if let Some(some) = self.r#purpose.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("purpose", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_purpose", &primitive_element)?;
            }
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
        if let Some(some) = self.r#kind.value.as_ref() {
            state.serialize_entry("kind", some)?;
        }
        if self.r#kind.id.is_some() || !self.r#kind.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#kind.id,
                extension: &self.r#kind.extension,
            };
            state.serialize_entry("_kind", &primitive_element)?;
        }
        if !self.r#instantiates.is_empty() {
            let values: Vec<_> = self.r#instantiates.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiates", &values)?;
            }
            let requires_elements = self
                .r#instantiates
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates
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
                state.serialize_entry("_instantiates", &primitive_elements)?;
            }
        }
        if !self.r#imports.is_empty() {
            let values: Vec<_> = self.r#imports.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("imports", &values)?;
            }
            let requires_elements = self
                .r#imports
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#imports
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
                state.serialize_entry("_imports", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#software.as_ref() {
            state.serialize_entry("software", some)?;
        }
        if let Some(some) = self.r#implementation.as_ref() {
            state.serialize_entry("implementation", some)?;
        }
        if let Some(some) = self.r#fhir_version.value.as_ref() {
            state.serialize_entry("fhirVersion", some)?;
        }
        if self.r#fhir_version.id.is_some() || !self.r#fhir_version.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#fhir_version.id,
                extension: &self.r#fhir_version.extension,
            };
            state.serialize_entry("_fhirVersion", &primitive_element)?;
        }
        if !self.r#format.is_empty() {
            let values: Vec<_> = self.r#format.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("format", &values)?;
            }
            let requires_elements = self
                .r#format
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#format
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
                state.serialize_entry("_format", &primitive_elements)?;
            }
        }
        if !self.r#patch_format.is_empty() {
            let values: Vec<_> = self.r#patch_format.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("patchFormat", &values)?;
            }
            let requires_elements = self
                .r#patch_format
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#patch_format
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
                state.serialize_entry("_patchFormat", &primitive_elements)?;
            }
        }
        if !self.r#implementation_guide.is_empty() {
            let values: Vec<_> = self
                .r#implementation_guide
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("implementationGuide", &values)?;
            }
            let requires_elements = self
                .r#implementation_guide
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#implementation_guide
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
                state.serialize_entry("_implementationGuide", &primitive_elements)?;
            }
        }
        if !self.r#rest.is_empty() {
            state.serialize_entry("rest", &self.r#rest)?;
        }
        if !self.r#messaging.is_empty() {
            state.serialize_entry("messaging", &self.r#messaging)?;
        }
        if !self.r#document.is_empty() {
            state.serialize_entry("document", &self.r#document)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CapabilityStatement {
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
            #[serde(rename = "purpose")]
            Purpose,
            #[serde(rename = "_purpose")]
            PurposePrimitiveElement,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            #[serde(rename = "kind")]
            Kind,
            #[serde(rename = "_kind")]
            KindPrimitiveElement,
            #[serde(rename = "instantiates")]
            Instantiates,
            #[serde(rename = "_instantiates")]
            InstantiatesPrimitiveElement,
            #[serde(rename = "imports")]
            Imports,
            #[serde(rename = "_imports")]
            ImportsPrimitiveElement,
            #[serde(rename = "software")]
            Software,
            #[serde(rename = "implementation")]
            Implementation,
            #[serde(rename = "fhirVersion")]
            FhirVersion,
            #[serde(rename = "_fhirVersion")]
            FhirVersionPrimitiveElement,
            #[serde(rename = "format")]
            Format,
            #[serde(rename = "_format")]
            FormatPrimitiveElement,
            #[serde(rename = "patchFormat")]
            PatchFormat,
            #[serde(rename = "_patchFormat")]
            PatchFormatPrimitiveElement,
            #[serde(rename = "implementationGuide")]
            ImplementationGuide,
            #[serde(rename = "_implementationGuide")]
            ImplementationGuidePrimitiveElement,
            #[serde(rename = "rest")]
            Rest,
            #[serde(rename = "messaging")]
            Messaging,
            #[serde(rename = "document")]
            Document,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CapabilityStatement;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CapabilityStatement")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CapabilityStatement, V::Error>
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
                let mut r#purpose: Option<super::super::types::Markdown> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
                let mut r#kind: Option<super::super::types::Code> = None;
                let mut r#instantiates: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#imports: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#software: Option<CapabilityStatementSoftware> = None;
                let mut r#implementation: Option<CapabilityStatementImplementation> = None;
                let mut r#fhir_version: Option<super::super::types::Code> = None;
                let mut r#format: Option<Vec<super::super::types::Code>> = None;
                let mut r#patch_format: Option<Vec<super::super::types::Code>> = None;
                let mut r#implementation_guide: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#rest: Option<Vec<CapabilityStatementRest>> = None;
                let mut r#messaging: Option<Vec<CapabilityStatementMessaging>> = None;
                let mut r#document: Option<Vec<CapabilityStatementDocument>> = None;
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
                        Field::Purpose => {
                            let some = r#purpose.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PurposePrimitiveElement => {
                            let some = r#purpose.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_purpose"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                        Field::Kind => {
                            let some = r#kind.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::KindPrimitiveElement => {
                            let some = r#kind.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_kind"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Instantiates => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#instantiates.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("instantiates"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#instantiates.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_instantiates"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Imports => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#imports.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("imports"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ImportsPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#imports.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_imports"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Software => {
                            if r#software.is_some() {
                                return Err(serde::de::Error::duplicate_field("software"));
                            }
                            r#software = Some(map_access.next_value()?);
                        }
                        Field::Implementation => {
                            if r#implementation.is_some() {
                                return Err(serde::de::Error::duplicate_field("implementation"));
                            }
                            r#implementation = Some(map_access.next_value()?);
                        }
                        Field::FhirVersion => {
                            let some = r#fhir_version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("fhirVersion"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FhirVersionPrimitiveElement => {
                            let some = r#fhir_version.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_fhirVersion"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Format => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#format.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::FormatPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#format.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_format"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::PatchFormat => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#patch_format.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("patchFormat"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::PatchFormatPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#patch_format.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_patchFormat"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::ImplementationGuide => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#implementation_guide
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "implementationGuide",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ImplementationGuidePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#implementation_guide
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_implementationGuide",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Rest => {
                            if r#rest.is_some() {
                                return Err(serde::de::Error::duplicate_field("rest"));
                            }
                            r#rest = Some(map_access.next_value()?);
                        }
                        Field::Messaging => {
                            if r#messaging.is_some() {
                                return Err(serde::de::Error::duplicate_field("messaging"));
                            }
                            r#messaging = Some(map_access.next_value()?);
                        }
                        Field::Document => {
                            if r#document.is_some() {
                                return Err(serde::de::Error::duplicate_field("document"));
                            }
                            r#document = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(CapabilityStatement {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url,
                    r#version,
                    r#name,
                    r#title,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#experimental,
                    r#date: r#date.ok_or(serde::de::Error::missing_field("date"))?,
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#description,
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                    r#purpose,
                    r#copyright,
                    r#kind: r#kind.ok_or(serde::de::Error::missing_field("kind"))?,
                    r#instantiates: r#instantiates.unwrap_or(vec![]),
                    r#imports: r#imports.unwrap_or(vec![]),
                    r#software,
                    r#implementation,
                    r#fhir_version: r#fhir_version
                        .ok_or(serde::de::Error::missing_field("fhir_version"))?,
                    r#format: r#format.unwrap_or(vec![]),
                    r#patch_format: r#patch_format.unwrap_or(vec![]),
                    r#implementation_guide: r#implementation_guide.unwrap_or(vec![]),
                    r#rest: r#rest.unwrap_or(vec![]),
                    r#messaging: r#messaging.unwrap_or(vec![]),
                    r#document: r#document.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
