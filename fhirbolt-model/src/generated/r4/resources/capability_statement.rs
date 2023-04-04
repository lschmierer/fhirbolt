// Generated on 2023-04-04 by fhirbolt-codegen v0.1.0
#[doc = "Software that is covered by this capability statement.  It is used when the capability statement describes the capabilities of a particular software version, independent of an installation."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementSoftware {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name the software is known by."]
    pub r#name: super::super::types::String,
    #[doc = "The version identifier for the software covered by this statement."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Date this version of the software was released."]
    pub r#release_date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for CapabilityStatementSoftware {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#release_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("releaseDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_releaseDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#release_date.as_ref() {
                    state.serialize_entry("releaseDate", some)?;
                }
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "version",
                                            "releaseDate",
                                        ],
                                    ));
                                }
                            }
                            Field::Version => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "version",
                                            "releaseDate",
                                        ],
                                    ));
                                }
                            }
                            Field::ReleaseDate => {
                                if _ctx.from_json {
                                    let some = r#release_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "releaseDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#release_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "releaseDate",
                                        ));
                                    }
                                    r#release_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::ReleaseDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#release_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_releaseDate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "releaseDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "version",
                                            "releaseDate",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "name",
                                        "version",
                                        "releaseDate",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementSoftware {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#version,
                        r#release_date,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Identifies a specific implementation instance that is described by the capability statement - i.e. a particular installation, rather than the capabilities of a software program."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementImplementation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Information about the specific installation that this capability statement relates to."]
    pub r#description: super::super::types::String,
    #[doc = "An absolute base URL for the implementation.  This forms the base for REST interfaces as well as the mailbox and document interfaces."]
    pub r#url: Option<super::super::types::Url>,
    #[doc = "The organization responsible for the management of the instance and oversight of the data on the server at the specified URL."]
    pub r#custodian: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CapabilityStatementImplementation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#description.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("description", &some)?;
                }
                if self.r#description.id.is_some() || !self.r#description.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#description.id.as_ref(),
                        extension: &self.r#description.extension,
                    };
                    state.serialize_entry("_description", &primitive_element)?;
                }
            } else {
                state.serialize_entry("description", &self.r#description)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if let Some(some) = self.r#custodian.as_ref() {
                state.serialize_entry("custodian", some)?;
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "description",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "url",
                                            "custodian",
                                        ],
                                    ));
                                }
                            }
                            Field::Url => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#url.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    r#url = Some(map_access.next_value()?);
                                }
                            }
                            Field::UrlPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "url",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "url",
                                            "custodian",
                                        ],
                                    ));
                                }
                            }
                            Field::Custodian => {
                                if r#custodian.is_some() {
                                    return Err(serde::de::Error::duplicate_field("custodian"));
                                }
                                r#custodian = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "description",
                                        "url",
                                        "custodian",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementImplementation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#description.unwrap_or(Default::default())
                        } else {
                            r#description.ok_or(serde::de::Error::missing_field("description"))?
                        },
                        r#url,
                        r#custodian,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Information about security implementation from an interface perspective - what a client needs to know."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestSecurity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Server adds CORS headers when responding to requests - this enables Javascript applications to use the server."]
    pub r#cors: Option<super::super::types::Boolean>,
    #[doc = "Types of security services that are supported/required by the system."]
    pub r#service: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "General description of how security works."]
    pub r#description: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestSecurity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#cors.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("cors", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_cors", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#cors.as_ref() {
                    state.serialize_entry("cors", some)?;
                }
            }
            if !self.r#service.is_empty() {
                state.serialize_entry("service", &self.r#service)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Cors => {
                                if _ctx.from_json {
                                    let some = r#cors.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("cors"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#cors.is_some() {
                                        return Err(serde::de::Error::duplicate_field("cors"));
                                    }
                                    r#cors = Some(map_access.next_value()?);
                                }
                            }
                            Field::CorsPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "cors",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "cors",
                                            "service",
                                            "description",
                                        ],
                                    ));
                                }
                            }
                            Field::Service => {
                                if _ctx.from_json {
                                    if r#service.is_some() {
                                        return Err(serde::de::Error::duplicate_field("service"));
                                    }
                                    r#service = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#service.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "description",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "cors",
                                            "service",
                                            "description",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "cors",
                                        "service",
                                        "description",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Identifies a restful operation supported by the solution."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResourceInteraction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded identifier of the operation, supported by the system resource."]
    pub r#code: super::super::types::Code,
    #[doc = "Guidance specific to the implementation of this operation, such as 'delete is a logical delete' or 'updates are only allowed with version id' or 'creates permitted from pre-authorized certificates only'."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestResourceInteraction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#code.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("code", &some)?;
                }
                if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#code.id.as_ref(),
                        extension: &self.r#code.extension,
                    };
                    state.serialize_entry("_code", &primitive_element)?;
                }
            } else {
                state.serialize_entry("code", &self.r#code)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Code => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#code.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    r#code = Some(map_access.next_value()?);
                                }
                            }
                            Field::CodePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "code",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "code",
                                        "documentation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementRestResourceInteraction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#documentation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Search parameters for implementations to support and/or make use of - either references to ones defined in the specification, or additional ones defined for/by the implementation."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResourceSearchParam {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of the search parameter used in the interface."]
    pub r#name: super::super::types::String,
    #[doc = "An absolute URI that is a formal reference to where this parameter was first defined, so that a client can be confident of the meaning of the search parameter (a reference to [SearchParameter.url](searchparameter-definitions.html#SearchParameter.url)). This element SHALL be populated if the search parameter refers to a SearchParameter defined by the FHIR core specification or externally defined IGs."]
    pub r#definition: Option<super::super::types::Canonical>,
    #[doc = "The type of value a search parameter refers to, and how the content is interpreted."]
    pub r#type: super::super::types::Code,
    #[doc = "This allows documentation of any distinct behaviors about how the search parameter is used.  For example, text matching algorithms."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestResourceSearchParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#definition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("definition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_definition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#definition.as_ref() {
                    state.serialize_entry("definition", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "definition",
                                            "type",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Definition => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#definition.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    r#definition = Some(map_access.next_value()?);
                                }
                            }
                            Field::DefinitionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_definition",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "definition",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "definition",
                                            "type",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "definition",
                                            "type",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "definition",
                                            "type",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "name",
                                        "definition",
                                        "type",
                                        "documentation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementRestResourceSearchParam {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#definition,
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#documentation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Definition of an operation or a named query together with its parameters and their meaning and type. Consult the definition of the operation for details about how to invoke the operation, and the parameters."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResourceOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of the operation or query. For an operation, this is the name  prefixed with $ and used in the URL. For a query, this is the name used in the _query parameter when the query is called."]
    pub r#name: super::super::types::String,
    #[doc = "Where the formal definition can be found. If a server references the base definition of an Operation (i.e. from the specification itself such as ```<http://hl7.org/fhir/OperationDefinition/ValueSet>-expand```), that means it supports the full capabilities of the operation - e.g. both GET and POST invocation.  If it only supports a subset, it must define its own custom [OperationDefinition](operationdefinition.html#) with a 'base' of the original OperationDefinition.  The custom definition would describe the specific subset of functionality supported."]
    pub r#definition: super::super::types::Canonical,
    #[doc = "Documentation that describes anything special about the operation behavior, possibly detailing different behavior for system, type and instance-level invocation of the operation."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestResourceOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#definition.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("definition", &some)?;
                }
                if self.r#definition.id.is_some() || !self.r#definition.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#definition.id.as_ref(),
                        extension: &self.r#definition.extension,
                    };
                    state.serialize_entry("_definition", &primitive_element)?;
                }
            } else {
                state.serialize_entry("definition", &self.r#definition)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "definition",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Definition => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#definition.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    r#definition = Some(map_access.next_value()?);
                                }
                            }
                            Field::DefinitionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_definition",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "definition",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "definition",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "definition",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "name",
                                        "definition",
                                        "documentation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementRestResourceOperation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#definition: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#definition.unwrap_or(Default::default())
                        } else {
                            r#definition.ok_or(serde::de::Error::missing_field("definition"))?
                        },
                        r#documentation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A specification of the restful capabilities of the solution for a specific resource type."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestResource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A type of resource exposed via the restful interface."]
    pub r#type: super::super::types::Code,
    #[doc = "A specification of the profile that describes the solution's overall support for the resource, including any constraints on cardinality, bindings, lengths or other limitations. See further discussion in [Using Profiles](profiling.html#profile-uses)."]
    pub r#profile: Option<super::super::types::Canonical>,
    #[doc = "A list of profiles that represent different use cases supported by the system. For a server, \"supported by the system\" means the system hosts/produces a set of resources that are conformant to a particular profile, and allows clients that use its services to search using this profile and to find appropriate data. For a client, it means the system will search by this profile and process data according to the guidance implicit in the profile. See further discussion in [Using Profiles](profiling.html#profile-uses)."]
    pub r#supported_profile: Vec<super::super::types::Canonical>,
    #[doc = "Additional information about the resource type used by the system."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "Identifies a restful operation supported by the solution."]
    pub r#interaction: Vec<CapabilityStatementRestResourceInteraction>,
    #[doc = "This field is set to no-version to specify that the system does not support (server) or use (client) versioning for this resource type. If this has some other value, the server must at least correctly track and populate the versionId meta-property on resources. If the value is 'versioned-update', then the server supports all the versioning features, including using e-tags for version integrity in the API."]
    pub r#versioning: Option<super::super::types::Code>,
    #[doc = "A flag for whether the server is able to return past versions as part of the vRead operation."]
    pub r#read_history: Option<super::super::types::Boolean>,
    #[doc = "A flag to indicate that the server allows or needs to allow the client to create new identities on the server (that is, the client PUTs to a location where there is no existing resource). Allowing this operation means that the server allows the client to create new identities on the server."]
    pub r#update_create: Option<super::super::types::Boolean>,
    #[doc = "A flag that indicates that the server supports conditional create."]
    pub r#conditional_create: Option<super::super::types::Boolean>,
    #[doc = "A code that indicates how the server supports conditional read."]
    pub r#conditional_read: Option<super::super::types::Code>,
    #[doc = "A flag that indicates that the server supports conditional update."]
    pub r#conditional_update: Option<super::super::types::Boolean>,
    #[doc = "A code that indicates how the server supports conditional delete."]
    pub r#conditional_delete: Option<super::super::types::Code>,
    #[doc = "A set of flags that defines how references are supported."]
    pub r#reference_policy: Vec<super::super::types::Code>,
    #[doc = "A list of _include values supported by the server."]
    pub r#search_include: Vec<super::super::types::String>,
    #[doc = "A list of _revinclude (reverse include) values supported by the server."]
    pub r#search_rev_include: Vec<super::super::types::String>,
    #[doc = "Search parameters for implementations to support and/or make use of - either references to ones defined in the specification, or additional ones defined for/by the implementation."]
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    #[doc = "Definition of an operation or a named query together with its parameters and their meaning and type. Consult the definition of the operation for details about how to invoke the operation, and the parameters."]
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
}
impl serde::ser::Serialize for CapabilityStatementRestResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#profile.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("profile", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_profile", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#profile.as_ref() {
                    state.serialize_entry("profile", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#supported_profile.is_empty() {
                    let values = self
                        .r#supported_profile
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#supported_profile.is_empty() {
                    state.serialize_entry("supportedProfile", &self.r#supported_profile)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            if !self.r#interaction.is_empty() {
                state.serialize_entry("interaction", &self.r#interaction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#versioning.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("versioning", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_versioning", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#versioning.as_ref() {
                    state.serialize_entry("versioning", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#read_history.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("readHistory", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_readHistory", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#read_history.as_ref() {
                    state.serialize_entry("readHistory", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#update_create.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("updateCreate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_updateCreate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#update_create.as_ref() {
                    state.serialize_entry("updateCreate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#conditional_create.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("conditionalCreate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_conditionalCreate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#conditional_create.as_ref() {
                    state.serialize_entry("conditionalCreate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#conditional_read.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("conditionalRead", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_conditionalRead", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#conditional_read.as_ref() {
                    state.serialize_entry("conditionalRead", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#conditional_update.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("conditionalUpdate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_conditionalUpdate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#conditional_update.as_ref() {
                    state.serialize_entry("conditionalUpdate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#conditional_delete.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("conditionalDelete", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_conditionalDelete", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#conditional_delete.as_ref() {
                    state.serialize_entry("conditionalDelete", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#reference_policy.is_empty() {
                    let values = self
                        .r#reference_policy
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#reference_policy.is_empty() {
                    state.serialize_entry("referencePolicy", &self.r#reference_policy)?;
                }
            }
            if _ctx.output_json {
                if !self.r#search_include.is_empty() {
                    let values = self
                        .r#search_include
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#search_include.is_empty() {
                    state.serialize_entry("searchInclude", &self.r#search_include)?;
                }
            }
            if _ctx.output_json {
                if !self.r#search_rev_include.is_empty() {
                    let values = self
                        .r#search_rev_include
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#search_rev_include.is_empty() {
                    state.serialize_entry("searchRevInclude", &self.r#search_rev_include)?;
                }
            }
            if !self.r#search_param.is_empty() {
                state.serialize_entry("searchParam", &self.r#search_param)?;
            }
            if !self.r#operation.is_empty() {
                state.serialize_entry("operation", &self.r#operation)?;
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::Profile => {
                                if _ctx.from_json {
                                    let some = r#profile.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("profile"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#profile.is_some() {
                                        return Err(serde::de::Error::duplicate_field("profile"));
                                    }
                                    r#profile = Some(map_access.next_value()?);
                                }
                            }
                            Field::ProfilePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "profile",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::SupportedProfile => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#supported_profile.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "supportedProfile",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#supported_profile.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SupportedProfilePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#supported_profile.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_supportedProfile",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "supportedProfile",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::Interaction => {
                                if _ctx.from_json {
                                    if r#interaction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "interaction",
                                        ));
                                    }
                                    r#interaction = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#interaction.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Versioning => {
                                if _ctx.from_json {
                                    let some = r#versioning.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "versioning",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#versioning.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "versioning",
                                        ));
                                    }
                                    r#versioning = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersioningPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#versioning.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_versioning",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "versioning",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::ReadHistory => {
                                if _ctx.from_json {
                                    let some = r#read_history.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "readHistory",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#read_history.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "readHistory",
                                        ));
                                    }
                                    r#read_history = Some(map_access.next_value()?);
                                }
                            }
                            Field::ReadHistoryPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#read_history.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_readHistory",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "readHistory",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::UpdateCreate => {
                                if _ctx.from_json {
                                    let some = r#update_create.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "updateCreate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#update_create.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "updateCreate",
                                        ));
                                    }
                                    r#update_create = Some(map_access.next_value()?);
                                }
                            }
                            Field::UpdateCreatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#update_create.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_updateCreate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "updateCreate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::ConditionalCreate => {
                                if _ctx.from_json {
                                    let some =
                                        r#conditional_create.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalCreate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#conditional_create.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalCreate",
                                        ));
                                    }
                                    r#conditional_create = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConditionalCreatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#conditional_create.get_or_insert(Default::default());
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "conditionalCreate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::ConditionalRead => {
                                if _ctx.from_json {
                                    let some = r#conditional_read.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalRead",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#conditional_read.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalRead",
                                        ));
                                    }
                                    r#conditional_read = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConditionalReadPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#conditional_read.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_conditionalRead",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "conditionalRead",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::ConditionalUpdate => {
                                if _ctx.from_json {
                                    let some =
                                        r#conditional_update.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalUpdate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#conditional_update.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalUpdate",
                                        ));
                                    }
                                    r#conditional_update = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConditionalUpdatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#conditional_update.get_or_insert(Default::default());
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "conditionalUpdate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::ConditionalDelete => {
                                if _ctx.from_json {
                                    let some =
                                        r#conditional_delete.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalDelete",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#conditional_delete.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conditionalDelete",
                                        ));
                                    }
                                    r#conditional_delete = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConditionalDeletePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#conditional_delete.get_or_insert(Default::default());
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "conditionalDelete",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::ReferencePolicy => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#reference_policy.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "referencePolicy",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#reference_policy.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ReferencePolicyPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#reference_policy.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_referencePolicy",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "referencePolicy",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::SearchInclude => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#search_include.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "searchInclude",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#search_include.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SearchIncludePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#search_include.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_searchInclude",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "searchInclude",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::SearchRevInclude => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#search_rev_include.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "searchRevInclude",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec =
                                        r#search_rev_include.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SearchRevIncludePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#search_rev_include.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_searchRevInclude",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "searchRevInclude",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "profile",
                                            "supportedProfile",
                                            "documentation",
                                            "interaction",
                                            "versioning",
                                            "readHistory",
                                            "updateCreate",
                                            "conditionalCreate",
                                            "conditionalRead",
                                            "conditionalUpdate",
                                            "conditionalDelete",
                                            "referencePolicy",
                                            "searchInclude",
                                            "searchRevInclude",
                                            "searchParam",
                                            "operation",
                                        ],
                                    ));
                                }
                            }
                            Field::SearchParam => {
                                if _ctx.from_json {
                                    if r#search_param.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "searchParam",
                                        ));
                                    }
                                    r#search_param = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#search_param.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Operation => {
                                if _ctx.from_json {
                                    if r#operation.is_some() {
                                        return Err(serde::de::Error::duplicate_field("operation"));
                                    }
                                    r#operation = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#operation.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "type",
                                        "profile",
                                        "supportedProfile",
                                        "documentation",
                                        "interaction",
                                        "versioning",
                                        "readHistory",
                                        "updateCreate",
                                        "conditionalCreate",
                                        "conditionalRead",
                                        "conditionalUpdate",
                                        "conditionalDelete",
                                        "referencePolicy",
                                        "searchInclude",
                                        "searchRevInclude",
                                        "searchParam",
                                        "operation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementRestResource {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A specification of restful operations supported by the system."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRestInteraction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A coded identifier of the operation, supported by the system."]
    pub r#code: super::super::types::Code,
    #[doc = "Guidance specific to the implementation of this operation, such as limitations on the kind of transactions allowed, or information about system wide search is implemented."]
    pub r#documentation: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CapabilityStatementRestInteraction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#code.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("code", &some)?;
                }
                if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#code.id.as_ref(),
                        extension: &self.r#code.extension,
                    };
                    state.serialize_entry("_code", &primitive_element)?;
                }
            } else {
                state.serialize_entry("code", &self.r#code)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Code => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#code.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    r#code = Some(map_access.next_value()?);
                                }
                            }
                            Field::CodePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "code",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "documentation",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "code",
                                        "documentation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementRestInteraction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#documentation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A definition of the restful capabilities of the solution, if any."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementRest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies whether this portion of the statement is describing the ability to initiate or receive restful operations."]
    pub r#mode: super::super::types::Code,
    #[doc = "Information about the system's restful capabilities that apply across all applications, such as security."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "Information about security implementation from an interface perspective - what a client needs to know."]
    pub r#security: Option<CapabilityStatementRestSecurity>,
    #[doc = "A specification of the restful capabilities of the solution for a specific resource type."]
    pub r#resource: Vec<CapabilityStatementRestResource>,
    #[doc = "A specification of restful operations supported by the system."]
    pub r#interaction: Vec<CapabilityStatementRestInteraction>,
    #[doc = "Search parameters that are supported for searching all resources for implementations to support and/or make use of - either references to ones defined in the specification, or additional ones defined for/by the implementation."]
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    #[doc = "Definition of an operation or a named query together with its parameters and their meaning and type."]
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
    #[doc = "An absolute URI which is a reference to the definition of a compartment that the system supports. The reference is to a CompartmentDefinition resource by its canonical URL ."]
    pub r#compartment: Vec<super::super::types::Canonical>,
}
impl serde::ser::Serialize for CapabilityStatementRest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#mode.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("mode", &some)?;
                }
                if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#mode.id.as_ref(),
                        extension: &self.r#mode.extension,
                    };
                    state.serialize_entry("_mode", &primitive_element)?;
                }
            } else {
                state.serialize_entry("mode", &self.r#mode)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
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
            if _ctx.output_json {
                if !self.r#compartment.is_empty() {
                    let values = self
                        .r#compartment
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#compartment.is_empty() {
                    state.serialize_entry("compartment", &self.r#compartment)?;
                }
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Mode => {
                                if _ctx.from_json {
                                    let some = r#mode.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("mode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#mode.is_some() {
                                        return Err(serde::de::Error::duplicate_field("mode"));
                                    }
                                    r#mode = Some(map_access.next_value()?);
                                }
                            }
                            Field::ModePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "mode",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "documentation",
                                            "security",
                                            "resource",
                                            "interaction",
                                            "searchParam",
                                            "operation",
                                            "compartment",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "documentation",
                                            "security",
                                            "resource",
                                            "interaction",
                                            "searchParam",
                                            "operation",
                                            "compartment",
                                        ],
                                    ));
                                }
                            }
                            Field::Security => {
                                if r#security.is_some() {
                                    return Err(serde::de::Error::duplicate_field("security"));
                                }
                                r#security = Some(map_access.next_value()?);
                            }
                            Field::Resource => {
                                if _ctx.from_json {
                                    if r#resource.is_some() {
                                        return Err(serde::de::Error::duplicate_field("resource"));
                                    }
                                    r#resource = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#resource.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Interaction => {
                                if _ctx.from_json {
                                    if r#interaction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "interaction",
                                        ));
                                    }
                                    r#interaction = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#interaction.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SearchParam => {
                                if _ctx.from_json {
                                    if r#search_param.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "searchParam",
                                        ));
                                    }
                                    r#search_param = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#search_param.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Operation => {
                                if _ctx.from_json {
                                    if r#operation.is_some() {
                                        return Err(serde::de::Error::duplicate_field("operation"));
                                    }
                                    r#operation = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#operation.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Compartment => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#compartment.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "compartment",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#compartment.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::CompartmentPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#compartment.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_compartment",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "compartment",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "documentation",
                                            "security",
                                            "resource",
                                            "interaction",
                                            "searchParam",
                                            "operation",
                                            "compartment",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "mode",
                                        "documentation",
                                        "security",
                                        "resource",
                                        "interaction",
                                        "searchParam",
                                        "operation",
                                        "compartment",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementRest {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#mode: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#mode.unwrap_or(Default::default())
                        } else {
                            r#mode.ok_or(serde::de::Error::missing_field("mode"))?
                        },
                        r#documentation,
                        r#security,
                        r#resource: r#resource.unwrap_or(vec![]),
                        r#interaction: r#interaction.unwrap_or(vec![]),
                        r#search_param: r#search_param.unwrap_or(vec![]),
                        r#operation: r#operation.unwrap_or(vec![]),
                        r#compartment: r#compartment.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An endpoint (network accessible address) to which messages and/or replies are to be sent."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementMessagingEndpoint {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A list of the messaging transport protocol(s) identifiers, supported by this endpoint."]
    pub r#protocol: Box<super::super::types::Coding>,
    #[doc = "The network address of the endpoint. For solutions that do not use network addresses for routing, it can be just an identifier."]
    pub r#address: super::super::types::Url,
}
impl serde::ser::Serialize for CapabilityStatementMessagingEndpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#address.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("address", &some)?;
                }
                if self.r#address.id.is_some() || !self.r#address.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#address.id.as_ref(),
                        extension: &self.r#address.extension,
                    };
                    state.serialize_entry("_address", &primitive_element)?;
                }
            } else {
                state.serialize_entry("address", &self.r#address)?;
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Protocol => {
                                if r#protocol.is_some() {
                                    return Err(serde::de::Error::duplicate_field("protocol"));
                                }
                                r#protocol = Some(map_access.next_value()?);
                            }
                            Field::Address => {
                                if _ctx.from_json {
                                    let some = r#address.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("address"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#address.is_some() {
                                        return Err(serde::de::Error::duplicate_field("address"));
                                    }
                                    r#address = Some(map_access.next_value()?);
                                }
                            }
                            Field::AddressPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "address",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "protocol",
                                            "address",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "protocol",
                                        "address",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementMessagingEndpoint {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#protocol: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#protocol.unwrap_or(Default::default())
                        } else {
                            r#protocol.ok_or(serde::de::Error::missing_field("protocol"))?
                        },
                        r#address: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#address.unwrap_or(Default::default())
                        } else {
                            r#address.ok_or(serde::de::Error::missing_field("address"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "References to message definitions for messages this system can send or receive."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementMessagingSupportedMessage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The mode of this event declaration - whether application is sender or receiver."]
    pub r#mode: super::super::types::Code,
    #[doc = "Points to a message definition that identifies the messaging event, message structure, allowed responses, etc."]
    pub r#definition: super::super::types::Canonical,
}
impl serde::ser::Serialize for CapabilityStatementMessagingSupportedMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#mode.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("mode", &some)?;
                }
                if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#mode.id.as_ref(),
                        extension: &self.r#mode.extension,
                    };
                    state.serialize_entry("_mode", &primitive_element)?;
                }
            } else {
                state.serialize_entry("mode", &self.r#mode)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#definition.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("definition", &some)?;
                }
                if self.r#definition.id.is_some() || !self.r#definition.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#definition.id.as_ref(),
                        extension: &self.r#definition.extension,
                    };
                    state.serialize_entry("_definition", &primitive_element)?;
                }
            } else {
                state.serialize_entry("definition", &self.r#definition)?;
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Mode => {
                                if _ctx.from_json {
                                    let some = r#mode.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("mode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#mode.is_some() {
                                        return Err(serde::de::Error::duplicate_field("mode"));
                                    }
                                    r#mode = Some(map_access.next_value()?);
                                }
                            }
                            Field::ModePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "mode",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "definition",
                                        ],
                                    ));
                                }
                            }
                            Field::Definition => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#definition.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    r#definition = Some(map_access.next_value()?);
                                }
                            }
                            Field::DefinitionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_definition",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "definition",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "definition",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "mode", "definition"],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementMessagingSupportedMessage {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#mode: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#mode.unwrap_or(Default::default())
                        } else {
                            r#mode.ok_or(serde::de::Error::missing_field("mode"))?
                        },
                        r#definition: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#definition.unwrap_or(Default::default())
                        } else {
                            r#definition.ok_or(serde::de::Error::missing_field("definition"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A description of the messaging capabilities of the solution."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementMessaging {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An endpoint (network accessible address) to which messages and/or replies are to be sent."]
    pub r#endpoint: Vec<CapabilityStatementMessagingEndpoint>,
    #[doc = "Length if the receiver's reliable messaging cache in minutes (if a receiver) or how long the cache length on the receiver should be (if a sender)."]
    pub r#reliable_cache: Option<super::super::types::UnsignedInt>,
    #[doc = "Documentation about the system's messaging capabilities for this endpoint not otherwise documented by the capability statement.  For example, the process for becoming an authorized messaging exchange partner."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "References to message definitions for messages this system can send or receive."]
    pub r#supported_message: Vec<CapabilityStatementMessagingSupportedMessage>,
}
impl serde::ser::Serialize for CapabilityStatementMessaging {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#reliable_cache.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("reliableCache", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_reliableCache", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#reliable_cache.as_ref() {
                    state.serialize_entry("reliableCache", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            if !self.r#supported_message.is_empty() {
                state.serialize_entry("supportedMessage", &self.r#supported_message)?;
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Endpoint => {
                                if _ctx.from_json {
                                    if r#endpoint.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    r#endpoint = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#endpoint.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ReliableCache => {
                                if _ctx.from_json {
                                    let some = r#reliable_cache.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reliableCache",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#reliable_cache.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reliableCache",
                                        ));
                                    }
                                    r#reliable_cache = Some(map_access.next_value()?);
                                }
                            }
                            Field::ReliableCachePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#reliable_cache.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_reliableCache",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "reliableCache",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "endpoint",
                                            "reliableCache",
                                            "documentation",
                                            "supportedMessage",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "endpoint",
                                            "reliableCache",
                                            "documentation",
                                            "supportedMessage",
                                        ],
                                    ));
                                }
                            }
                            Field::SupportedMessage => {
                                if _ctx.from_json {
                                    if r#supported_message.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "supportedMessage",
                                        ));
                                    }
                                    r#supported_message = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#supported_message.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "endpoint",
                                        "reliableCache",
                                        "documentation",
                                        "supportedMessage",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A document definition."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatementDocument {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Mode of this document declaration - whether an application is a producer or consumer."]
    pub r#mode: super::super::types::Code,
    #[doc = "A description of how the application supports or uses the specified document profile.  For example, when documents are created, what action is taken with consumed documents, etc."]
    pub r#documentation: Option<super::super::types::Markdown>,
    #[doc = "A profile on the document Bundle that constrains which resources are present, and their contents."]
    pub r#profile: super::super::types::Canonical,
}
impl serde::ser::Serialize for CapabilityStatementDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#mode.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("mode", &some)?;
                }
                if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#mode.id.as_ref(),
                        extension: &self.r#mode.extension,
                    };
                    state.serialize_entry("_mode", &primitive_element)?;
                }
            } else {
                state.serialize_entry("mode", &self.r#mode)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#profile.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("profile", &some)?;
                }
                if self.r#profile.id.is_some() || !self.r#profile.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#profile.id.as_ref(),
                        extension: &self.r#profile.extension,
                    };
                    state.serialize_entry("_profile", &primitive_element)?;
                }
            } else {
                state.serialize_entry("profile", &self.r#profile)?;
            }
            state.end()
        })
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
            Unknown(std::string::String),
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Mode => {
                                if _ctx.from_json {
                                    let some = r#mode.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("mode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#mode.is_some() {
                                        return Err(serde::de::Error::duplicate_field("mode"));
                                    }
                                    r#mode = Some(map_access.next_value()?);
                                }
                            }
                            Field::ModePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "mode",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "documentation",
                                            "profile",
                                        ],
                                    ));
                                }
                            }
                            Field::Documentation => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#documentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "documentation",
                                        ));
                                    }
                                    r#documentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::DocumentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#documentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_documentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "documentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "documentation",
                                            "profile",
                                        ],
                                    ));
                                }
                            }
                            Field::Profile => {
                                if _ctx.from_json {
                                    let some = r#profile.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("profile"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#profile.is_some() {
                                        return Err(serde::de::Error::duplicate_field("profile"));
                                    }
                                    r#profile = Some(map_access.next_value()?);
                                }
                            }
                            Field::ProfilePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "profile",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "mode",
                                            "documentation",
                                            "profile",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "mode",
                                        "documentation",
                                        "profile",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CapabilityStatementDocument {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#mode: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#mode.unwrap_or(Default::default())
                        } else {
                            r#mode.ok_or(serde::de::Error::missing_field("mode"))?
                        },
                        r#documentation,
                        r#profile: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#profile.unwrap_or(Default::default())
                        } else {
                            r#profile.ok_or(serde::de::Error::missing_field("profile"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server for a particular version of FHIR that may be used as a statement of actual server functionality or a statement of required or desired server implementation."]
#[derive(Default, Debug, Clone)]
pub struct CapabilityStatement {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An absolute URI that is used to identify this capability statement when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this capability statement is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the capability statement is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "The identifier that is used to identify this version of the capability statement when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the capability statement author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the capability statement. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the capability statement."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this capability statement. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this capability statement is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the capability statement was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the capability statement changes."]
    pub r#date: super::super::types::DateTime,
    #[doc = "The name of the organization or individual that published the capability statement."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the capability statement from a consumer's perspective. Typically, this is used when the capability statement describes a desired rather than an actual solution, for example as a formal expression of requirements as part of an RFP."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate capability statement instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the capability statement is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this capability statement is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the capability statement and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the capability statement."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The way that this statement is intended to be used, to describe an actual running instance of software, a particular product (kind, not instance of software) or a class of implementation (e.g. a desired purchase)."]
    pub r#kind: super::super::types::Code,
    #[doc = "Reference to a canonical URL of another CapabilityStatement that this software implements. This capability statement is a published API description that corresponds to a business service. The server may actually implement a subset of the capability statement it claims to implement, so the capability statement must specify the full capability details."]
    pub r#instantiates: Vec<super::super::types::Canonical>,
    #[doc = "Reference to a canonical URL of another CapabilityStatement that this software adds to. The capability statement automatically includes everything in the other statement, and it is not duplicated, though the server may repeat the same resources, interactions and operations to add additional details to them."]
    pub r#imports: Vec<super::super::types::Canonical>,
    #[doc = "Software that is covered by this capability statement.  It is used when the capability statement describes the capabilities of a particular software version, independent of an installation."]
    pub r#software: Option<CapabilityStatementSoftware>,
    #[doc = "Identifies a specific implementation instance that is described by the capability statement - i.e. a particular installation, rather than the capabilities of a software program."]
    pub r#implementation: Option<CapabilityStatementImplementation>,
    #[doc = "The version of the FHIR specification that this CapabilityStatement describes (which SHALL be the same as the FHIR version of the CapabilityStatement itself). There is no default value."]
    pub r#fhir_version: super::super::types::Code,
    #[doc = "A list of the formats supported by this implementation using their content types."]
    pub r#format: Vec<super::super::types::Code>,
    #[doc = "A list of the patch formats supported by this implementation using their content types."]
    pub r#patch_format: Vec<super::super::types::Code>,
    #[doc = "A list of implementation guides that the server does (or should) support in their entirety."]
    pub r#implementation_guide: Vec<super::super::types::Canonical>,
    #[doc = "A definition of the restful capabilities of the solution, if any."]
    pub r#rest: Vec<CapabilityStatementRest>,
    #[doc = "A description of the messaging capabilities of the solution."]
    pub r#messaging: Vec<CapabilityStatementMessaging>,
    #[doc = "A document definition."]
    pub r#document: Vec<CapabilityStatementDocument>,
}
impl crate::AnyResource for CapabilityStatement {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for CapabilityStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "CapabilityStatement")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#experimental.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("experimental", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_experimental", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#experimental.as_ref() {
                    state.serialize_entry("experimental", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("date", &some)?;
                }
                if self.r#date.id.is_some() || !self.r#date.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#date.id.as_ref(),
                        extension: &self.r#date.extension,
                    };
                    state.serialize_entry("_date", &primitive_element)?;
                }
            } else {
                state.serialize_entry("date", &self.r#date)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publisher.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publisher", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publisher", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publisher.as_ref() {
                    state.serialize_entry("publisher", some)?;
                }
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if !self.r#use_context.is_empty() {
                state.serialize_entry("useContext", &self.r#use_context)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#purpose.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("purpose", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_purpose", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#purpose.as_ref() {
                    state.serialize_entry("purpose", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#kind.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("kind", &some)?;
                }
                if self.r#kind.id.is_some() || !self.r#kind.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#kind.id.as_ref(),
                        extension: &self.r#kind.extension,
                    };
                    state.serialize_entry("_kind", &primitive_element)?;
                }
            } else {
                state.serialize_entry("kind", &self.r#kind)?;
            }
            if _ctx.output_json {
                if !self.r#instantiates.is_empty() {
                    let values = self
                        .r#instantiates
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#instantiates.is_empty() {
                    state.serialize_entry("instantiates", &self.r#instantiates)?;
                }
            }
            if _ctx.output_json {
                if !self.r#imports.is_empty() {
                    let values = self
                        .r#imports
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#imports.is_empty() {
                    state.serialize_entry("imports", &self.r#imports)?;
                }
            }
            if let Some(some) = self.r#software.as_ref() {
                state.serialize_entry("software", some)?;
            }
            if let Some(some) = self.r#implementation.as_ref() {
                state.serialize_entry("implementation", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#fhir_version.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("fhirVersion", &some)?;
                }
                if self.r#fhir_version.id.is_some() || !self.r#fhir_version.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#fhir_version.id.as_ref(),
                        extension: &self.r#fhir_version.extension,
                    };
                    state.serialize_entry("_fhirVersion", &primitive_element)?;
                }
            } else {
                state.serialize_entry("fhirVersion", &self.r#fhir_version)?;
            }
            if _ctx.output_json {
                if !self.r#format.is_empty() {
                    let values = self
                        .r#format
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#format.is_empty() {
                    state.serialize_entry("format", &self.r#format)?;
                }
            }
            if _ctx.output_json {
                if !self.r#patch_format.is_empty() {
                    let values = self
                        .r#patch_format
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#patch_format.is_empty() {
                    state.serialize_entry("patchFormat", &self.r#patch_format)?;
                }
            }
            if _ctx.output_json {
                if !self.r#implementation_guide.is_empty() {
                    let values = self
                        .r#implementation_guide
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#implementation_guide.is_empty() {
                    state.serialize_entry("implementationGuide", &self.r#implementation_guide)?;
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
        })
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
            #[serde(rename = "resourceType")]
            ResourceType,
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
            Unknown(std::string::String),
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "CapabilityStatement" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"CapabilityStatement",
                                    ));
                                }
                            }
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
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_implicitRules",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implicitRules",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Url => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#url.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    r#url = Some(map_access.next_value()?);
                                }
                            }
                            Field::UrlPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "url",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Version => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Title => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#title.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    r#title = Some(map_access.next_value()?);
                                }
                            }
                            Field::TitlePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "title",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Experimental => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#experimental.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    r#experimental = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExperimentalPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_experimental",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "experimental",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Date => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    r#date = Some(map_access.next_value()?);
                                }
                            }
                            Field::DatePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "date",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Publisher => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#publisher.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    r#publisher = Some(map_access.next_value()?);
                                }
                            }
                            Field::PublisherPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_publisher",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "publisher",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Contact => {
                                if _ctx.from_json {
                                    if r#contact.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contact"));
                                    }
                                    r#contact = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contact.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "description",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::UseContext => {
                                if _ctx.from_json {
                                    if r#use_context.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "useContext",
                                        ));
                                    }
                                    r#use_context = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#use_context.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Jurisdiction => {
                                if _ctx.from_json {
                                    if r#jurisdiction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "jurisdiction",
                                        ));
                                    }
                                    r#jurisdiction = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#jurisdiction.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Purpose => {
                                if _ctx.from_json {
                                    let some = r#purpose.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#purpose.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    r#purpose = Some(map_access.next_value()?);
                                }
                            }
                            Field::PurposePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "purpose",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Copyright => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#copyright.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    r#copyright = Some(map_access.next_value()?);
                                }
                            }
                            Field::CopyrightPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_copyright",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "copyright",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Kind => {
                                if _ctx.from_json {
                                    let some = r#kind.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("kind"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#kind.is_some() {
                                        return Err(serde::de::Error::duplicate_field("kind"));
                                    }
                                    r#kind = Some(map_access.next_value()?);
                                }
                            }
                            Field::KindPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "kind",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Instantiates => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#instantiates.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiates",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#instantiates.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::InstantiatesPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#instantiates.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_instantiates",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "instantiates",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Imports => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#imports.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("imports"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#imports.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ImportsPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#imports.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_imports"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "imports",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "implementation",
                                    ));
                                }
                                r#implementation = Some(map_access.next_value()?);
                            }
                            Field::FhirVersion => {
                                if _ctx.from_json {
                                    let some = r#fhir_version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fhirVersion",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#fhir_version.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fhirVersion",
                                        ));
                                    }
                                    r#fhir_version = Some(map_access.next_value()?);
                                }
                            }
                            Field::FhirVersionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#fhir_version.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fhirVersion",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "fhirVersion",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Format => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#format.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("format"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#format.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::FormatPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#format.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_format"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "format",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::PatchFormat => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#patch_format.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "patchFormat",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#patch_format.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PatchFormatPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#patch_format.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patchFormat",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "patchFormat",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::ImplementationGuide => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#implementation_guide.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "implementationGuide",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec =
                                        r#implementation_guide.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ImplementationGuidePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#implementation_guide.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_implementationGuide",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implementationGuide",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "kind",
                                            "instantiates",
                                            "imports",
                                            "software",
                                            "implementation",
                                            "fhirVersion",
                                            "format",
                                            "patchFormat",
                                            "implementationGuide",
                                            "rest",
                                            "messaging",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Rest => {
                                if _ctx.from_json {
                                    if r#rest.is_some() {
                                        return Err(serde::de::Error::duplicate_field("rest"));
                                    }
                                    r#rest = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#rest.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Messaging => {
                                if _ctx.from_json {
                                    if r#messaging.is_some() {
                                        return Err(serde::de::Error::duplicate_field("messaging"));
                                    }
                                    r#messaging = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#messaging.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Document => {
                                if _ctx.from_json {
                                    if r#document.is_some() {
                                        return Err(serde::de::Error::duplicate_field("document"));
                                    }
                                    r#document = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#document.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "url",
                                        "version",
                                        "name",
                                        "title",
                                        "status",
                                        "experimental",
                                        "date",
                                        "publisher",
                                        "contact",
                                        "description",
                                        "useContext",
                                        "jurisdiction",
                                        "purpose",
                                        "copyright",
                                        "kind",
                                        "instantiates",
                                        "imports",
                                        "software",
                                        "implementation",
                                        "fhirVersion",
                                        "format",
                                        "patchFormat",
                                        "implementationGuide",
                                        "rest",
                                        "messaging",
                                        "document",
                                    ],
                                ));
                            },
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
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#experimental,
                        r#date: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#date.unwrap_or(Default::default())
                        } else {
                            r#date.ok_or(serde::de::Error::missing_field("date"))?
                        },
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#description,
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#purpose,
                        r#copyright,
                        r#kind: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#kind.unwrap_or(Default::default())
                        } else {
                            r#kind.ok_or(serde::de::Error::missing_field("kind"))?
                        },
                        r#instantiates: r#instantiates.unwrap_or(vec![]),
                        r#imports: r#imports.unwrap_or(vec![]),
                        r#software,
                        r#implementation,
                        r#fhir_version: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#fhir_version.unwrap_or(Default::default())
                        } else {
                            r#fhir_version.ok_or(serde::de::Error::missing_field("fhirVersion"))?
                        },
                        r#format: r#format.unwrap_or(vec![]),
                        r#patch_format: r#patch_format.unwrap_or(vec![]),
                        r#implementation_guide: r#implementation_guide.unwrap_or(vec![]),
                        r#rest: r#rest.unwrap_or(vec![]),
                        r#messaging: r#messaging.unwrap_or(vec![]),
                        r#document: r#document.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
