// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "An abstract server used in operations within this test script in the origin element."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptOrigin {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Abstract name given to an origin server in this test script.  The name is provided as a number starting at 1."]
    pub r#index: super::super::types::Integer,
    #[doc = "The type of origin profile the test system supports."]
    pub r#profile: Box<super::super::types::Coding>,
}
impl serde::ser::Serialize for TestScriptOrigin {
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
                if let Some(some) = self.r#index.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("index", &some)?;
                }
                if self.r#index.id.is_some() || !self.r#index.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#index.id.as_ref(),
                        extension: &self.r#index.extension,
                    };
                    state.serialize_entry("_index", &primitive_element)?;
                }
            } else {
                state.serialize_entry("index", &self.r#index)?;
            }
            state.serialize_entry("profile", &self.r#profile)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptOrigin {
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
            #[serde(rename = "index")]
            Index,
            #[serde(rename = "_index")]
            IndexPrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptOrigin;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptOrigin")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptOrigin, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#index: Option<super::super::types::Integer> = None;
                let mut r#profile: Option<Box<super::super::types::Coding>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Index => {
                                if _ctx.from_json {
                                    let some = r#index.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("index"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#index.is_some() {
                                        return Err(serde::de::Error::duplicate_field("index"));
                                    }
                                    r#index = Some(map_access.next_value()?);
                                }
                            }
                            Field::IndexPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#index.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_index"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "index",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "index",
                                            "profile",
                                        ],
                                    ));
                                }
                            }
                            Field::Profile => {
                                if r#profile.is_some() {
                                    return Err(serde::de::Error::duplicate_field("profile"));
                                }
                                r#profile = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "index", "profile"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptOrigin {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#index: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#index.unwrap_or(Default::default())
                        } else {
                            r#index.ok_or(serde::de::Error::missing_field("index"))?
                        },
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
#[doc = "An abstract server used in operations within this test script in the destination element."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptDestination {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Abstract name given to a destination server in this test script.  The name is provided as a number starting at 1."]
    pub r#index: super::super::types::Integer,
    #[doc = "The type of destination profile the test system supports."]
    pub r#profile: Box<super::super::types::Coding>,
}
impl serde::ser::Serialize for TestScriptDestination {
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
                if let Some(some) = self.r#index.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("index", &some)?;
                }
                if self.r#index.id.is_some() || !self.r#index.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#index.id.as_ref(),
                        extension: &self.r#index.extension,
                    };
                    state.serialize_entry("_index", &primitive_element)?;
                }
            } else {
                state.serialize_entry("index", &self.r#index)?;
            }
            state.serialize_entry("profile", &self.r#profile)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptDestination {
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
            #[serde(rename = "index")]
            Index,
            #[serde(rename = "_index")]
            IndexPrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptDestination;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptDestination")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptDestination, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#index: Option<super::super::types::Integer> = None;
                let mut r#profile: Option<Box<super::super::types::Coding>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Index => {
                                if _ctx.from_json {
                                    let some = r#index.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("index"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#index.is_some() {
                                        return Err(serde::de::Error::duplicate_field("index"));
                                    }
                                    r#index = Some(map_access.next_value()?);
                                }
                            }
                            Field::IndexPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#index.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_index"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "index",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "index",
                                            "profile",
                                        ],
                                    ));
                                }
                            }
                            Field::Profile => {
                                if r#profile.is_some() {
                                    return Err(serde::de::Error::duplicate_field("profile"));
                                }
                                r#profile = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "index", "profile"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptDestination {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#index: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#index.unwrap_or(Default::default())
                        } else {
                            r#index.ok_or(serde::de::Error::missing_field("index"))?
                        },
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
#[doc = "A link to the FHIR specification that this test is covering."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptMetadataLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "URL to a particular requirement or feature within the FHIR specification."]
    pub r#url: super::super::types::Uri,
    #[doc = "Short description of the link."]
    pub r#description: Option<super::super::types::String>,
}
impl serde::ser::Serialize for TestScriptMetadataLink {
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
                if let Some(some) = self.r#url.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#url.id.as_ref(),
                        extension: &self.r#url.extension,
                    };
                    state.serialize_entry("_url", &primitive_element)?;
                }
            } else {
                state.serialize_entry("url", &self.r#url)?;
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
impl<'de> serde::de::Deserialize<'de> for TestScriptMetadataLink {
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
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptMetadataLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptMetadataLink")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptMetadataLink, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#description: Option<super::super::types::String> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
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
                                            "url",
                                            "description",
                                        ],
                                    ));
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
                                            "url",
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
                                    &["id", "extension", "modifierExtension", "url", "description"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptMetadataLink {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#url.unwrap_or(Default::default())
                        } else {
                            r#url.ok_or(serde::de::Error::missing_field("url"))?
                        },
                        r#description,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Capabilities that must exist and are assumed to function correctly on the FHIR server being tested."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptMetadataCapability {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Whether or not the test execution will require the given capabilities of the server in order for this test script to execute."]
    pub r#required: super::super::types::Boolean,
    #[doc = "Whether or not the test execution will validate the given capabilities of the server in order for this test script to execute."]
    pub r#validated: super::super::types::Boolean,
    #[doc = "Description of the capabilities that this test script is requiring the server to support."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Which origin server these requirements apply to."]
    pub r#origin: Vec<super::super::types::Integer>,
    #[doc = "Which server these requirements apply to."]
    pub r#destination: Option<super::super::types::Integer>,
    #[doc = "Links to the FHIR specification that describes this interaction and the resources involved in more detail."]
    pub r#link: Vec<super::super::types::Uri>,
    #[doc = "Minimum capabilities required of server for test script to execute successfully.   If server does not meet at a minimum the referenced capability statement, then all tests in this script are skipped."]
    pub r#capabilities: super::super::types::Canonical,
}
impl serde::ser::Serialize for TestScriptMetadataCapability {
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
                if let Some(some) = self.r#required.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("required", &some)?;
                }
                if self.r#required.id.is_some() || !self.r#required.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#required.id.as_ref(),
                        extension: &self.r#required.extension,
                    };
                    state.serialize_entry("_required", &primitive_element)?;
                }
            } else {
                state.serialize_entry("required", &self.r#required)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#validated.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("validated", &some)?;
                }
                if self.r#validated.id.is_some() || !self.r#validated.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#validated.id.as_ref(),
                        extension: &self.r#validated.extension,
                    };
                    state.serialize_entry("_validated", &primitive_element)?;
                }
            } else {
                state.serialize_entry("validated", &self.r#validated)?;
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
            if _ctx.output_json {
                if !self.r#origin.is_empty() {
                    let values = self
                        .r#origin
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("origin", &values)?;
                    }
                    let requires_elements = self
                        .r#origin
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#origin
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
                        state.serialize_entry("_origin", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#origin.is_empty() {
                    state.serialize_entry("origin", &self.r#origin)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#destination.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("destination", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_destination", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#destination.as_ref() {
                    state.serialize_entry("destination", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#link.is_empty() {
                    let values = self
                        .r#link
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("link", &values)?;
                    }
                    let requires_elements = self
                        .r#link
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link
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
                        state.serialize_entry("_link", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link.is_empty() {
                    state.serialize_entry("link", &self.r#link)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#capabilities.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("capabilities", &some)?;
                }
                if self.r#capabilities.id.is_some() || !self.r#capabilities.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#capabilities.id.as_ref(),
                        extension: &self.r#capabilities.extension,
                    };
                    state.serialize_entry("_capabilities", &primitive_element)?;
                }
            } else {
                state.serialize_entry("capabilities", &self.r#capabilities)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptMetadataCapability {
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
            #[serde(rename = "required")]
            Required,
            #[serde(rename = "_required")]
            RequiredPrimitiveElement,
            #[serde(rename = "validated")]
            Validated,
            #[serde(rename = "_validated")]
            ValidatedPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "_origin")]
            OriginPrimitiveElement,
            #[serde(rename = "destination")]
            Destination,
            #[serde(rename = "_destination")]
            DestinationPrimitiveElement,
            #[serde(rename = "link")]
            Link,
            #[serde(rename = "_link")]
            LinkPrimitiveElement,
            #[serde(rename = "capabilities")]
            Capabilities,
            #[serde(rename = "_capabilities")]
            CapabilitiesPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptMetadataCapability;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptMetadataCapability")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptMetadataCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#required: Option<super::super::types::Boolean> = None;
                let mut r#validated: Option<super::super::types::Boolean> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#origin: Option<Vec<super::super::types::Integer>> = None;
                let mut r#destination: Option<super::super::types::Integer> = None;
                let mut r#link: Option<Vec<super::super::types::Uri>> = None;
                let mut r#capabilities: Option<super::super::types::Canonical> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Required => {
                                if _ctx.from_json {
                                    let some = r#required.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("required"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#required.is_some() {
                                        return Err(serde::de::Error::duplicate_field("required"));
                                    }
                                    r#required = Some(map_access.next_value()?);
                                }
                            }
                            Field::RequiredPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#required.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_required"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "required",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "required",
                                            "validated",
                                            "description",
                                            "origin",
                                            "destination",
                                            "link",
                                            "capabilities",
                                        ],
                                    ));
                                }
                            }
                            Field::Validated => {
                                if _ctx.from_json {
                                    let some = r#validated.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("validated"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#validated.is_some() {
                                        return Err(serde::de::Error::duplicate_field("validated"));
                                    }
                                    r#validated = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValidatedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#validated.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_validated",
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
                                        "validated",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "required",
                                            "validated",
                                            "description",
                                            "origin",
                                            "destination",
                                            "link",
                                            "capabilities",
                                        ],
                                    ));
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
                                            "required",
                                            "validated",
                                            "description",
                                            "origin",
                                            "destination",
                                            "link",
                                            "capabilities",
                                        ],
                                    ));
                                }
                            }
                            Field::Origin => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#origin.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("origin"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#origin.is_some() {
                                        return Err(serde::de::Error::duplicate_field("origin"));
                                    }
                                    r#origin = Some(map_access.next_value()?);
                                }
                            }
                            Field::OriginPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#origin.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_origin"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "origin",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "required",
                                            "validated",
                                            "description",
                                            "origin",
                                            "destination",
                                            "link",
                                            "capabilities",
                                        ],
                                    ));
                                }
                            }
                            Field::Destination => {
                                if _ctx.from_json {
                                    let some = r#destination.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "destination",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#destination.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "destination",
                                        ));
                                    }
                                    r#destination = Some(map_access.next_value()?);
                                }
                            }
                            Field::DestinationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#destination.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_destination",
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
                                        "destination",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "required",
                                            "validated",
                                            "description",
                                            "origin",
                                            "destination",
                                            "link",
                                            "capabilities",
                                        ],
                                    ));
                                }
                            }
                            Field::Link => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#link.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("link"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#link.is_some() {
                                        return Err(serde::de::Error::duplicate_field("link"));
                                    }
                                    r#link = Some(map_access.next_value()?);
                                }
                            }
                            Field::LinkPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#link.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_link"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "link",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "required",
                                            "validated",
                                            "description",
                                            "origin",
                                            "destination",
                                            "link",
                                            "capabilities",
                                        ],
                                    ));
                                }
                            }
                            Field::Capabilities => {
                                if _ctx.from_json {
                                    let some = r#capabilities.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "capabilities",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#capabilities.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "capabilities",
                                        ));
                                    }
                                    r#capabilities = Some(map_access.next_value()?);
                                }
                            }
                            Field::CapabilitiesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#capabilities.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_capabilities",
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
                                        "capabilities",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "required",
                                            "validated",
                                            "description",
                                            "origin",
                                            "destination",
                                            "link",
                                            "capabilities",
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
                                        "required",
                                        "validated",
                                        "description",
                                        "origin",
                                        "destination",
                                        "link",
                                        "capabilities",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptMetadataCapability {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#required: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#required.unwrap_or(Default::default())
                        } else {
                            r#required.ok_or(serde::de::Error::missing_field("required"))?
                        },
                        r#validated: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#validated.unwrap_or(Default::default())
                        } else {
                            r#validated.ok_or(serde::de::Error::missing_field("validated"))?
                        },
                        r#description,
                        r#origin: r#origin.unwrap_or(vec![]),
                        r#destination,
                        r#link: r#link.unwrap_or(vec![]),
                        r#capabilities: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#capabilities.unwrap_or(Default::default())
                        } else {
                            r#capabilities.ok_or(serde::de::Error::missing_field("capabilities"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The required capability must exist and are assumed to function correctly on the FHIR server being tested."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptMetadata {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A link to the FHIR specification that this test is covering."]
    pub r#link: Vec<TestScriptMetadataLink>,
    #[doc = "Capabilities that must exist and are assumed to function correctly on the FHIR server being tested."]
    pub r#capability: Vec<TestScriptMetadataCapability>,
}
impl serde::ser::Serialize for TestScriptMetadata {
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
            if !self.r#link.is_empty() {
                state.serialize_entry("link", &self.r#link)?;
            }
            if !self.r#capability.is_empty() {
                state.serialize_entry("capability", &self.r#capability)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptMetadata {
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
            #[serde(rename = "link")]
            Link,
            #[serde(rename = "capability")]
            Capability,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptMetadata;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptMetadata")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptMetadata, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#link: Option<Vec<TestScriptMetadataLink>> = None;
                let mut r#capability: Option<Vec<TestScriptMetadataCapability>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Link => {
                                if r#link.is_some() {
                                    return Err(serde::de::Error::duplicate_field("link"));
                                }
                                r#link = Some(map_access.next_value()?);
                            }
                            Field::Capability => {
                                if r#capability.is_some() {
                                    return Err(serde::de::Error::duplicate_field("capability"));
                                }
                                r#capability = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "link", "capability"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptMetadata {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#link: r#link.unwrap_or(vec![]),
                        r#capability: r#capability.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Fixture in the test script - by reference (uri). All fixtures are required for the test script to execute."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptFixture {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Whether or not to implicitly create the fixture during setup. If true, the fixture is automatically created on each server being tested during setup, therefore no create operation is required for this fixture in the TestScript.setup section."]
    pub r#autocreate: super::super::types::Boolean,
    #[doc = "Whether or not to implicitly delete the fixture during teardown. If true, the fixture is automatically deleted on each server being tested during teardown, therefore no delete operation is required for this fixture in the TestScript.teardown section."]
    pub r#autodelete: super::super::types::Boolean,
    #[doc = "Reference to the resource (containing the contents of the resource needed for operations)."]
    pub r#resource: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for TestScriptFixture {
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
                if let Some(some) = self.r#autocreate.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("autocreate", &some)?;
                }
                if self.r#autocreate.id.is_some() || !self.r#autocreate.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#autocreate.id.as_ref(),
                        extension: &self.r#autocreate.extension,
                    };
                    state.serialize_entry("_autocreate", &primitive_element)?;
                }
            } else {
                state.serialize_entry("autocreate", &self.r#autocreate)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#autodelete.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("autodelete", &some)?;
                }
                if self.r#autodelete.id.is_some() || !self.r#autodelete.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#autodelete.id.as_ref(),
                        extension: &self.r#autodelete.extension,
                    };
                    state.serialize_entry("_autodelete", &primitive_element)?;
                }
            } else {
                state.serialize_entry("autodelete", &self.r#autodelete)?;
            }
            if let Some(some) = self.r#resource.as_ref() {
                state.serialize_entry("resource", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptFixture {
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
            #[serde(rename = "autocreate")]
            Autocreate,
            #[serde(rename = "_autocreate")]
            AutocreatePrimitiveElement,
            #[serde(rename = "autodelete")]
            Autodelete,
            #[serde(rename = "_autodelete")]
            AutodeletePrimitiveElement,
            #[serde(rename = "resource")]
            Resource,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptFixture;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptFixture")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptFixture, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#autocreate: Option<super::super::types::Boolean> = None;
                let mut r#autodelete: Option<super::super::types::Boolean> = None;
                let mut r#resource: Option<Box<super::super::types::Reference>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Autocreate => {
                                if _ctx.from_json {
                                    let some = r#autocreate.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "autocreate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#autocreate.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "autocreate",
                                        ));
                                    }
                                    r#autocreate = Some(map_access.next_value()?);
                                }
                            }
                            Field::AutocreatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#autocreate.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_autocreate",
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
                                        "autocreate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "autocreate",
                                            "autodelete",
                                            "resource",
                                        ],
                                    ));
                                }
                            }
                            Field::Autodelete => {
                                if _ctx.from_json {
                                    let some = r#autodelete.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "autodelete",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#autodelete.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "autodelete",
                                        ));
                                    }
                                    r#autodelete = Some(map_access.next_value()?);
                                }
                            }
                            Field::AutodeletePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#autodelete.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_autodelete",
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
                                        "autodelete",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "autocreate",
                                            "autodelete",
                                            "resource",
                                        ],
                                    ));
                                }
                            }
                            Field::Resource => {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                r#resource = Some(map_access.next_value()?);
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
                                        "autocreate",
                                        "autodelete",
                                        "resource",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptFixture {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#autocreate: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#autocreate.unwrap_or(Default::default())
                        } else {
                            r#autocreate.ok_or(serde::de::Error::missing_field("autocreate"))?
                        },
                        r#autodelete: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#autodelete.unwrap_or(Default::default())
                        } else {
                            r#autodelete.ok_or(serde::de::Error::missing_field("autodelete"))?
                        },
                        r#resource,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Variable is set based either on element value in response body or on header field value in the response headers."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptVariable {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Descriptive name for this variable."]
    pub r#name: super::super::types::String,
    #[doc = "A default, hard-coded, or user-defined value for this variable."]
    pub r#default_value: Option<super::super::types::String>,
    #[doc = "A free text natural language description of the variable and its purpose."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The FHIRPath expression to evaluate against the fixture body. When variables are defined, only one of either expression, headerField or path must be specified."]
    pub r#expression: Option<super::super::types::String>,
    #[doc = "Will be used to grab the HTTP header field value from the headers that sourceId is pointing to."]
    pub r#header_field: Option<super::super::types::String>,
    #[doc = "Displayable text string with hint help information to the user when entering a default value."]
    pub r#hint: Option<super::super::types::String>,
    #[doc = "XPath or JSONPath to evaluate against the fixture body.  When variables are defined, only one of either expression, headerField or path must be specified."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "Fixture to evaluate the XPath/JSONPath expression or the headerField  against within this variable."]
    pub r#source_id: Option<super::super::types::Id>,
}
impl serde::ser::Serialize for TestScriptVariable {
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
                if let Some(some) = self.r#default_value.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("defaultValue", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_defaultValue", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#default_value.as_ref() {
                    state.serialize_entry("defaultValue", some)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#expression.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("expression", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_expression", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#expression.as_ref() {
                    state.serialize_entry("expression", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#header_field.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("headerField", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_headerField", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#header_field.as_ref() {
                    state.serialize_entry("headerField", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#hint.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("hint", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_hint", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#hint.as_ref() {
                    state.serialize_entry("hint", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#path.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("path", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_path", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#path.as_ref() {
                    state.serialize_entry("path", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#source_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sourceId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sourceId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source_id.as_ref() {
                    state.serialize_entry("sourceId", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptVariable {
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
            #[serde(rename = "defaultValue")]
            DefaultValue,
            #[serde(rename = "_defaultValue")]
            DefaultValuePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "expression")]
            Expression,
            #[serde(rename = "_expression")]
            ExpressionPrimitiveElement,
            #[serde(rename = "headerField")]
            HeaderField,
            #[serde(rename = "_headerField")]
            HeaderFieldPrimitiveElement,
            #[serde(rename = "hint")]
            Hint,
            #[serde(rename = "_hint")]
            HintPrimitiveElement,
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "sourceId")]
            SourceId,
            #[serde(rename = "_sourceId")]
            SourceIdPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptVariable;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptVariable")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptVariable, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#default_value: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#expression: Option<super::super::types::String> = None;
                let mut r#header_field: Option<super::super::types::String> = None;
                let mut r#hint: Option<super::super::types::String> = None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#source_id: Option<super::super::types::Id> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
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
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
                                        ],
                                    ));
                                }
                            }
                            Field::DefaultValue => {
                                if _ctx.from_json {
                                    let some = r#default_value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValue",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#default_value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValue",
                                        ));
                                    }
                                    r#default_value = Some(map_access.next_value()?);
                                }
                            }
                            Field::DefaultValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#default_value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValue",
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
                                        "defaultValue",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
                                        ],
                                    ));
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
                                            "name",
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
                                        ],
                                    ));
                                }
                            }
                            Field::Expression => {
                                if _ctx.from_json {
                                    let some = r#expression.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "expression",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#expression.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "expression",
                                        ));
                                    }
                                    r#expression = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExpressionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#expression.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_expression",
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
                                        "expression",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
                                        ],
                                    ));
                                }
                            }
                            Field::HeaderField => {
                                if _ctx.from_json {
                                    let some = r#header_field.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "headerField",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#header_field.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "headerField",
                                        ));
                                    }
                                    r#header_field = Some(map_access.next_value()?);
                                }
                            }
                            Field::HeaderFieldPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#header_field.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_headerField",
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
                                        "headerField",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
                                        ],
                                    ));
                                }
                            }
                            Field::Hint => {
                                if _ctx.from_json {
                                    let some = r#hint.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("hint"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#hint.is_some() {
                                        return Err(serde::de::Error::duplicate_field("hint"));
                                    }
                                    r#hint = Some(map_access.next_value()?);
                                }
                            }
                            Field::HintPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#hint.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_hint"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "hint",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
                                        ],
                                    ));
                                }
                            }
                            Field::Path => {
                                if _ctx.from_json {
                                    let some = r#path.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("path"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#path.is_some() {
                                        return Err(serde::de::Error::duplicate_field("path"));
                                    }
                                    r#path = Some(map_access.next_value()?);
                                }
                            }
                            Field::PathPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#path.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_path"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "path",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
                                        ],
                                    ));
                                }
                            }
                            Field::SourceId => {
                                if _ctx.from_json {
                                    let some = r#source_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sourceId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#source_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sourceId"));
                                    }
                                    r#source_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::SourceIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#source_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_sourceId"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sourceId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "defaultValue",
                                            "description",
                                            "expression",
                                            "headerField",
                                            "hint",
                                            "path",
                                            "sourceId",
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
                                        "defaultValue",
                                        "description",
                                        "expression",
                                        "headerField",
                                        "hint",
                                        "path",
                                        "sourceId",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptVariable {
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
                        r#default_value,
                        r#description,
                        r#expression,
                        r#header_field,
                        r#hint,
                        r#path,
                        r#source_id,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Header elements would be used to set HTTP headers."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptSetupActionOperationRequestHeader {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The HTTP header field e.g. \"Accept\"."]
    pub r#field: super::super::types::String,
    #[doc = "The value of the header e.g. \"application/fhir+xml\"."]
    pub r#value: super::super::types::String,
}
impl serde::ser::Serialize for TestScriptSetupActionOperationRequestHeader {
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
                if let Some(some) = self.r#field.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("field", &some)?;
                }
                if self.r#field.id.is_some() || !self.r#field.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#field.id.as_ref(),
                        extension: &self.r#field.extension,
                    };
                    state.serialize_entry("_field", &primitive_element)?;
                }
            } else {
                state.serialize_entry("field", &self.r#field)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptSetupActionOperationRequestHeader {
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
            #[serde(rename = "field")]
            Field,
            #[serde(rename = "_field")]
            FieldPrimitiveElement,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptSetupActionOperationRequestHeader;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupActionOperationRequestHeader")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptSetupActionOperationRequestHeader, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#field: Option<super::super::types::String> = None;
                let mut r#value: Option<super::super::types::String> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Field => {
                                if _ctx.from_json {
                                    let some = r#field.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("field"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#field.is_some() {
                                        return Err(serde::de::Error::duplicate_field("field"));
                                    }
                                    r#field = Some(map_access.next_value()?);
                                }
                            }
                            Field::FieldPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#field.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_field"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "field",
                                        &["id", "extension", "modifierExtension", "field", "value"],
                                    ));
                                }
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &["id", "extension", "modifierExtension", "field", "value"],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "field", "value"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptSetupActionOperationRequestHeader {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#field: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#field.unwrap_or(Default::default())
                        } else {
                            r#field.ok_or(serde::de::Error::missing_field("field"))?
                        },
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The operation to perform."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptSetupActionOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Server interaction or operation type."]
    pub r#type: Option<Box<super::super::types::Coding>>,
    #[doc = "The type of the resource.  See <http://build.fhir.org/resourcelist.html>."]
    pub r#resource: Option<super::super::types::Code>,
    #[doc = "The label would be used for tracking/logging purposes by test engines."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "The description would be used by test engines for tracking and reporting purposes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The mime-type to use for RESTful operation in the 'Accept' header."]
    pub r#accept: Option<super::super::types::Code>,
    #[doc = "The mime-type to use for RESTful operation in the 'Content-Type' header."]
    pub r#content_type: Option<super::super::types::Code>,
    #[doc = "The server where the request message is destined for.  Must be one of the server numbers listed in TestScript.destination section."]
    pub r#destination: Option<super::super::types::Integer>,
    #[doc = "Whether or not to implicitly send the request url in encoded format. The default is true to match the standard RESTful client behavior. Set to false when communicating with a server that does not support encoded url paths."]
    pub r#encode_request_url: super::super::types::Boolean,
    #[doc = "The HTTP method the test engine MUST use for this operation regardless of any other operation details."]
    pub r#method: Option<super::super::types::Code>,
    #[doc = "The server where the request message originates from.  Must be one of the server numbers listed in TestScript.origin section."]
    pub r#origin: Option<super::super::types::Integer>,
    #[doc = "Path plus parameters after `type`.  Used to set parts of the request URL explicitly."]
    pub r#params: Option<super::super::types::String>,
    #[doc = "Header elements would be used to set HTTP headers."]
    pub r#request_header: Vec<TestScriptSetupActionOperationRequestHeader>,
    #[doc = "The fixture id (maybe new) to map to the request."]
    pub r#request_id: Option<super::super::types::Id>,
    #[doc = "The fixture id (maybe new) to map to the response."]
    pub r#response_id: Option<super::super::types::Id>,
    #[doc = "The id of the fixture used as the body of a PUT or POST request."]
    pub r#source_id: Option<super::super::types::Id>,
    #[doc = "Id of fixture used for extracting the `id`,  `type`, and `vid` for GET requests."]
    pub r#target_id: Option<super::super::types::Id>,
    #[doc = "Complete request URL."]
    pub r#url: Option<super::super::types::String>,
}
impl serde::ser::Serialize for TestScriptSetupActionOperation {
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#resource.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("resource", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_resource", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#resource.as_ref() {
                    state.serialize_entry("resource", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#label.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("label", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_label", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#label.as_ref() {
                    state.serialize_entry("label", some)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#accept.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("accept", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_accept", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#accept.as_ref() {
                    state.serialize_entry("accept", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#content_type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("contentType", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_contentType", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#content_type.as_ref() {
                    state.serialize_entry("contentType", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#destination.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("destination", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_destination", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#destination.as_ref() {
                    state.serialize_entry("destination", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#encode_request_url.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("encodeRequestUrl", &some)?;
                }
                if self.r#encode_request_url.id.is_some()
                    || !self.r#encode_request_url.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#encode_request_url.id.as_ref(),
                        extension: &self.r#encode_request_url.extension,
                    };
                    state.serialize_entry("_encodeRequestUrl", &primitive_element)?;
                }
            } else {
                state.serialize_entry("encodeRequestUrl", &self.r#encode_request_url)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#method.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("method", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_method", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#method.as_ref() {
                    state.serialize_entry("method", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#origin.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("origin", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_origin", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#origin.as_ref() {
                    state.serialize_entry("origin", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#params.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("params", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_params", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#params.as_ref() {
                    state.serialize_entry("params", some)?;
                }
            }
            if !self.r#request_header.is_empty() {
                state.serialize_entry("requestHeader", &self.r#request_header)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#request_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requestId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requestId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#request_id.as_ref() {
                    state.serialize_entry("requestId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#response_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("responseId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_responseId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#response_id.as_ref() {
                    state.serialize_entry("responseId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#source_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sourceId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sourceId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source_id.as_ref() {
                    state.serialize_entry("sourceId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#target_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("targetId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_targetId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#target_id.as_ref() {
                    state.serialize_entry("targetId", some)?;
                }
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
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptSetupActionOperation {
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
            #[serde(rename = "resource")]
            Resource,
            #[serde(rename = "_resource")]
            ResourcePrimitiveElement,
            #[serde(rename = "label")]
            Label,
            #[serde(rename = "_label")]
            LabelPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "accept")]
            Accept,
            #[serde(rename = "_accept")]
            AcceptPrimitiveElement,
            #[serde(rename = "contentType")]
            ContentType,
            #[serde(rename = "_contentType")]
            ContentTypePrimitiveElement,
            #[serde(rename = "destination")]
            Destination,
            #[serde(rename = "_destination")]
            DestinationPrimitiveElement,
            #[serde(rename = "encodeRequestUrl")]
            EncodeRequestUrl,
            #[serde(rename = "_encodeRequestUrl")]
            EncodeRequestUrlPrimitiveElement,
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "_method")]
            MethodPrimitiveElement,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "_origin")]
            OriginPrimitiveElement,
            #[serde(rename = "params")]
            Params,
            #[serde(rename = "_params")]
            ParamsPrimitiveElement,
            #[serde(rename = "requestHeader")]
            RequestHeader,
            #[serde(rename = "requestId")]
            RequestId,
            #[serde(rename = "_requestId")]
            RequestIdPrimitiveElement,
            #[serde(rename = "responseId")]
            ResponseId,
            #[serde(rename = "_responseId")]
            ResponseIdPrimitiveElement,
            #[serde(rename = "sourceId")]
            SourceId,
            #[serde(rename = "_sourceId")]
            SourceIdPrimitiveElement,
            #[serde(rename = "targetId")]
            TargetId,
            #[serde(rename = "_targetId")]
            TargetIdPrimitiveElement,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptSetupActionOperation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupActionOperation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptSetupActionOperation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
                let mut r#resource: Option<super::super::types::Code> = None;
                let mut r#label: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#accept: Option<super::super::types::Code> = None;
                let mut r#content_type: Option<super::super::types::Code> = None;
                let mut r#destination: Option<super::super::types::Integer> = None;
                let mut r#encode_request_url: Option<super::super::types::Boolean> = None;
                let mut r#method: Option<super::super::types::Code> = None;
                let mut r#origin: Option<super::super::types::Integer> = None;
                let mut r#params: Option<super::super::types::String> = None;
                let mut r#request_header: Option<Vec<TestScriptSetupActionOperationRequestHeader>> =
                    None;
                let mut r#request_id: Option<super::super::types::Id> = None;
                let mut r#response_id: Option<super::super::types::Id> = None;
                let mut r#source_id: Option<super::super::types::Id> = None;
                let mut r#target_id: Option<super::super::types::Id> = None;
                let mut r#url: Option<super::super::types::String> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Resource => {
                                if _ctx.from_json {
                                    let some = r#resource.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("resource"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#resource.is_some() {
                                        return Err(serde::de::Error::duplicate_field("resource"));
                                    }
                                    r#resource = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResourcePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#resource.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_resource"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "resource",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::Label => {
                                if _ctx.from_json {
                                    let some = r#label.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("label"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#label.is_some() {
                                        return Err(serde::de::Error::duplicate_field("label"));
                                    }
                                    r#label = Some(map_access.next_value()?);
                                }
                            }
                            Field::LabelPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#label.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_label"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "label",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
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
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::Accept => {
                                if _ctx.from_json {
                                    let some = r#accept.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("accept"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#accept.is_some() {
                                        return Err(serde::de::Error::duplicate_field("accept"));
                                    }
                                    r#accept = Some(map_access.next_value()?);
                                }
                            }
                            Field::AcceptPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#accept.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_accept"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "accept",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::ContentType => {
                                if _ctx.from_json {
                                    let some = r#content_type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "contentType",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#content_type.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "contentType",
                                        ));
                                    }
                                    r#content_type = Some(map_access.next_value()?);
                                }
                            }
                            Field::ContentTypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#content_type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_contentType",
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
                                        "contentType",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::Destination => {
                                if _ctx.from_json {
                                    let some = r#destination.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "destination",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#destination.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "destination",
                                        ));
                                    }
                                    r#destination = Some(map_access.next_value()?);
                                }
                            }
                            Field::DestinationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#destination.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_destination",
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
                                        "destination",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::EncodeRequestUrl => {
                                if _ctx.from_json {
                                    let some =
                                        r#encode_request_url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "encodeRequestUrl",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#encode_request_url.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "encodeRequestUrl",
                                        ));
                                    }
                                    r#encode_request_url = Some(map_access.next_value()?);
                                }
                            }
                            Field::EncodeRequestUrlPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#encode_request_url.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_encodeRequestUrl",
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
                                        "encodeRequestUrl",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::Method => {
                                if _ctx.from_json {
                                    let some = r#method.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("method"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#method.is_some() {
                                        return Err(serde::de::Error::duplicate_field("method"));
                                    }
                                    r#method = Some(map_access.next_value()?);
                                }
                            }
                            Field::MethodPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#method.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_method"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "method",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::Origin => {
                                if _ctx.from_json {
                                    let some = r#origin.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("origin"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#origin.is_some() {
                                        return Err(serde::de::Error::duplicate_field("origin"));
                                    }
                                    r#origin = Some(map_access.next_value()?);
                                }
                            }
                            Field::OriginPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#origin.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_origin"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "origin",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::Params => {
                                if _ctx.from_json {
                                    let some = r#params.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("params"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#params.is_some() {
                                        return Err(serde::de::Error::duplicate_field("params"));
                                    }
                                    r#params = Some(map_access.next_value()?);
                                }
                            }
                            Field::ParamsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#params.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_params"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "params",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::RequestHeader => {
                                if r#request_header.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestHeader"));
                                }
                                r#request_header = Some(map_access.next_value()?);
                            }
                            Field::RequestId => {
                                if _ctx.from_json {
                                    let some = r#request_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("requestId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#request_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("requestId"));
                                    }
                                    r#request_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::RequestIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#request_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_requestId",
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
                                        "requestId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::ResponseId => {
                                if _ctx.from_json {
                                    let some = r#response_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "responseId",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#response_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "responseId",
                                        ));
                                    }
                                    r#response_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResponseIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#response_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_responseId",
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
                                        "responseId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::SourceId => {
                                if _ctx.from_json {
                                    let some = r#source_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sourceId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#source_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sourceId"));
                                    }
                                    r#source_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::SourceIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#source_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_sourceId"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sourceId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
                                        ],
                                    ));
                                }
                            }
                            Field::TargetId => {
                                if _ctx.from_json {
                                    let some = r#target_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("targetId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#target_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("targetId"));
                                    }
                                    r#target_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::TargetIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#target_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_targetId"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "targetId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
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
                                            "type",
                                            "resource",
                                            "label",
                                            "description",
                                            "accept",
                                            "contentType",
                                            "destination",
                                            "encodeRequestUrl",
                                            "method",
                                            "origin",
                                            "params",
                                            "requestHeader",
                                            "requestId",
                                            "responseId",
                                            "sourceId",
                                            "targetId",
                                            "url",
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
                                        "type",
                                        "resource",
                                        "label",
                                        "description",
                                        "accept",
                                        "contentType",
                                        "destination",
                                        "encodeRequestUrl",
                                        "method",
                                        "origin",
                                        "params",
                                        "requestHeader",
                                        "requestId",
                                        "responseId",
                                        "sourceId",
                                        "targetId",
                                        "url",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptSetupActionOperation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#resource,
                        r#label,
                        r#description,
                        r#accept,
                        r#content_type,
                        r#destination,
                        r#encode_request_url: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#encode_request_url.unwrap_or(Default::default())
                        } else {
                            r#encode_request_url
                                .ok_or(serde::de::Error::missing_field("encodeRequestUrl"))?
                        },
                        r#method,
                        r#origin,
                        r#params,
                        r#request_header: r#request_header.unwrap_or(vec![]),
                        r#request_id,
                        r#response_id,
                        r#source_id,
                        r#target_id,
                        r#url,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Evaluates the results of previous operations to determine if the server under test behaves appropriately."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptSetupActionAssert {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The label would be used for tracking/logging purposes by test engines."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "The description would be used by test engines for tracking and reporting purposes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The direction to use for the assertion."]
    pub r#direction: Option<super::super::types::Code>,
    #[doc = "Id of the source fixture used as the contents to be evaluated by either the \"source/expression\" or \"sourceId/path\" definition."]
    pub r#compare_to_source_id: Option<super::super::types::String>,
    #[doc = "The FHIRPath expression to evaluate against the source fixture. When compareToSourceId is defined, either compareToSourceExpression or compareToSourcePath must be defined, but not both."]
    pub r#compare_to_source_expression: Option<super::super::types::String>,
    #[doc = "XPath or JSONPath expression to evaluate against the source fixture. When compareToSourceId is defined, either compareToSourceExpression or compareToSourcePath must be defined, but not both."]
    pub r#compare_to_source_path: Option<super::super::types::String>,
    #[doc = "The mime-type contents to compare against the request or response message 'Content-Type' header."]
    pub r#content_type: Option<super::super::types::Code>,
    #[doc = "The FHIRPath expression to be evaluated against the request or response message contents - HTTP headers and payload."]
    pub r#expression: Option<super::super::types::String>,
    #[doc = "The HTTP header field name e.g. 'Location'."]
    pub r#header_field: Option<super::super::types::String>,
    #[doc = "The ID of a fixture.  Asserts that the response contains at a minimum the fixture specified by minimumId."]
    pub r#minimum_id: Option<super::super::types::String>,
    #[doc = "Whether or not the test execution performs validation on the bundle navigation links."]
    pub r#navigation_links: Option<super::super::types::Boolean>,
    #[doc = "The operator type defines the conditional behavior of the assert. If not defined, the default is equals."]
    pub r#operator: Option<super::super::types::Code>,
    #[doc = "The XPath or JSONPath expression to be evaluated against the fixture representing the response received from server."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "The request method or HTTP operation code to compare against that used by the client system under test."]
    pub r#request_method: Option<super::super::types::Code>,
    #[doc = "The value to use in a comparison against the request URL path string."]
    pub r#request_url: Option<super::super::types::String>,
    #[doc = "The type of the resource.  See <http://build.fhir.org/resourcelist.html>."]
    pub r#resource: Option<super::super::types::Code>,
    #[doc = "okay | created | noContent | notModified | bad | forbidden | notFound | methodNotAllowed | conflict | gone | preconditionFailed | unprocessable."]
    pub r#response: Option<super::super::types::Code>,
    #[doc = "The value of the HTTP response code to be tested."]
    pub r#response_code: Option<super::super::types::String>,
    #[doc = "Fixture to evaluate the XPath/JSONPath expression or the headerField  against."]
    pub r#source_id: Option<super::super::types::Id>,
    #[doc = "The ID of the Profile to validate against."]
    pub r#validate_profile_id: Option<super::super::types::Id>,
    #[doc = "The value to compare to."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "Whether or not the test execution will produce a warning only on error for this assert."]
    pub r#warning_only: super::super::types::Boolean,
}
impl serde::ser::Serialize for TestScriptSetupActionAssert {
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
                if let Some(some) = self.r#label.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("label", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_label", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#label.as_ref() {
                    state.serialize_entry("label", some)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#direction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("direction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_direction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#direction.as_ref() {
                    state.serialize_entry("direction", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#compare_to_source_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("compareToSourceId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_compareToSourceId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#compare_to_source_id.as_ref() {
                    state.serialize_entry("compareToSourceId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#compare_to_source_expression.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("compareToSourceExpression", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_compareToSourceExpression", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#compare_to_source_expression.as_ref() {
                    state.serialize_entry("compareToSourceExpression", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#compare_to_source_path.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("compareToSourcePath", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_compareToSourcePath", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#compare_to_source_path.as_ref() {
                    state.serialize_entry("compareToSourcePath", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#content_type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("contentType", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_contentType", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#content_type.as_ref() {
                    state.serialize_entry("contentType", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#expression.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("expression", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_expression", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#expression.as_ref() {
                    state.serialize_entry("expression", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#header_field.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("headerField", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_headerField", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#header_field.as_ref() {
                    state.serialize_entry("headerField", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#minimum_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("minimumId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_minimumId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#minimum_id.as_ref() {
                    state.serialize_entry("minimumId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#navigation_links.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("navigationLinks", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_navigationLinks", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#navigation_links.as_ref() {
                    state.serialize_entry("navigationLinks", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#operator.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("operator", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_operator", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#operator.as_ref() {
                    state.serialize_entry("operator", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#path.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("path", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_path", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#path.as_ref() {
                    state.serialize_entry("path", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#request_method.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requestMethod", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requestMethod", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#request_method.as_ref() {
                    state.serialize_entry("requestMethod", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#request_url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requestURL", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requestURL", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#request_url.as_ref() {
                    state.serialize_entry("requestURL", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#resource.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("resource", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_resource", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#resource.as_ref() {
                    state.serialize_entry("resource", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#response.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("response", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_response", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#response.as_ref() {
                    state.serialize_entry("response", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#response_code.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("responseCode", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_responseCode", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#response_code.as_ref() {
                    state.serialize_entry("responseCode", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#source_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sourceId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sourceId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source_id.as_ref() {
                    state.serialize_entry("sourceId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#validate_profile_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("validateProfileId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_validateProfileId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#validate_profile_id.as_ref() {
                    state.serialize_entry("validateProfileId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("value", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_value", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value.as_ref() {
                    state.serialize_entry("value", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#warning_only.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("warningOnly", &some)?;
                }
                if self.r#warning_only.id.is_some() || !self.r#warning_only.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#warning_only.id.as_ref(),
                        extension: &self.r#warning_only.extension,
                    };
                    state.serialize_entry("_warningOnly", &primitive_element)?;
                }
            } else {
                state.serialize_entry("warningOnly", &self.r#warning_only)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptSetupActionAssert {
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
            #[serde(rename = "label")]
            Label,
            #[serde(rename = "_label")]
            LabelPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "direction")]
            Direction,
            #[serde(rename = "_direction")]
            DirectionPrimitiveElement,
            #[serde(rename = "compareToSourceId")]
            CompareToSourceId,
            #[serde(rename = "_compareToSourceId")]
            CompareToSourceIdPrimitiveElement,
            #[serde(rename = "compareToSourceExpression")]
            CompareToSourceExpression,
            #[serde(rename = "_compareToSourceExpression")]
            CompareToSourceExpressionPrimitiveElement,
            #[serde(rename = "compareToSourcePath")]
            CompareToSourcePath,
            #[serde(rename = "_compareToSourcePath")]
            CompareToSourcePathPrimitiveElement,
            #[serde(rename = "contentType")]
            ContentType,
            #[serde(rename = "_contentType")]
            ContentTypePrimitiveElement,
            #[serde(rename = "expression")]
            Expression,
            #[serde(rename = "_expression")]
            ExpressionPrimitiveElement,
            #[serde(rename = "headerField")]
            HeaderField,
            #[serde(rename = "_headerField")]
            HeaderFieldPrimitiveElement,
            #[serde(rename = "minimumId")]
            MinimumId,
            #[serde(rename = "_minimumId")]
            MinimumIdPrimitiveElement,
            #[serde(rename = "navigationLinks")]
            NavigationLinks,
            #[serde(rename = "_navigationLinks")]
            NavigationLinksPrimitiveElement,
            #[serde(rename = "operator")]
            Operator,
            #[serde(rename = "_operator")]
            OperatorPrimitiveElement,
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "requestMethod")]
            RequestMethod,
            #[serde(rename = "_requestMethod")]
            RequestMethodPrimitiveElement,
            #[serde(rename = "requestURL")]
            RequestUrl,
            #[serde(rename = "_requestURL")]
            RequestUrlPrimitiveElement,
            #[serde(rename = "resource")]
            Resource,
            #[serde(rename = "_resource")]
            ResourcePrimitiveElement,
            #[serde(rename = "response")]
            Response,
            #[serde(rename = "_response")]
            ResponsePrimitiveElement,
            #[serde(rename = "responseCode")]
            ResponseCode,
            #[serde(rename = "_responseCode")]
            ResponseCodePrimitiveElement,
            #[serde(rename = "sourceId")]
            SourceId,
            #[serde(rename = "_sourceId")]
            SourceIdPrimitiveElement,
            #[serde(rename = "validateProfileId")]
            ValidateProfileId,
            #[serde(rename = "_validateProfileId")]
            ValidateProfileIdPrimitiveElement,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            #[serde(rename = "warningOnly")]
            WarningOnly,
            #[serde(rename = "_warningOnly")]
            WarningOnlyPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptSetupActionAssert;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupActionAssert")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptSetupActionAssert, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#label: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#direction: Option<super::super::types::Code> = None;
                let mut r#compare_to_source_id: Option<super::super::types::String> = None;
                let mut r#compare_to_source_expression: Option<super::super::types::String> = None;
                let mut r#compare_to_source_path: Option<super::super::types::String> = None;
                let mut r#content_type: Option<super::super::types::Code> = None;
                let mut r#expression: Option<super::super::types::String> = None;
                let mut r#header_field: Option<super::super::types::String> = None;
                let mut r#minimum_id: Option<super::super::types::String> = None;
                let mut r#navigation_links: Option<super::super::types::Boolean> = None;
                let mut r#operator: Option<super::super::types::Code> = None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#request_method: Option<super::super::types::Code> = None;
                let mut r#request_url: Option<super::super::types::String> = None;
                let mut r#resource: Option<super::super::types::Code> = None;
                let mut r#response: Option<super::super::types::Code> = None;
                let mut r#response_code: Option<super::super::types::String> = None;
                let mut r#source_id: Option<super::super::types::Id> = None;
                let mut r#validate_profile_id: Option<super::super::types::Id> = None;
                let mut r#value: Option<super::super::types::String> = None;
                let mut r#warning_only: Option<super::super::types::Boolean> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Label => {
                                if _ctx.from_json {
                                    let some = r#label.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("label"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#label.is_some() {
                                        return Err(serde::de::Error::duplicate_field("label"));
                                    }
                                    r#label = Some(map_access.next_value()?);
                                }
                            }
                            Field::LabelPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#label.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_label"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "label",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
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
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::Direction => {
                                if _ctx.from_json {
                                    let some = r#direction.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("direction"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#direction.is_some() {
                                        return Err(serde::de::Error::duplicate_field("direction"));
                                    }
                                    r#direction = Some(map_access.next_value()?);
                                }
                            }
                            Field::DirectionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#direction.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_direction",
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
                                        "direction",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::CompareToSourceId => {
                                if _ctx.from_json {
                                    let some =
                                        r#compare_to_source_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compareToSourceId",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#compare_to_source_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compareToSourceId",
                                        ));
                                    }
                                    r#compare_to_source_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::CompareToSourceIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#compare_to_source_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_compareToSourceId",
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
                                        "compareToSourceId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::CompareToSourceExpression => {
                                if _ctx.from_json {
                                    let some = r#compare_to_source_expression
                                        .get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compareToSourceExpression",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#compare_to_source_expression.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compareToSourceExpression",
                                        ));
                                    }
                                    r#compare_to_source_expression = Some(map_access.next_value()?);
                                }
                            }
                            Field::CompareToSourceExpressionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#compare_to_source_expression
                                        .get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_compareToSourceExpression",
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
                                        "compareToSourceExpression",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::CompareToSourcePath => {
                                if _ctx.from_json {
                                    let some =
                                        r#compare_to_source_path.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compareToSourcePath",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#compare_to_source_path.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compareToSourcePath",
                                        ));
                                    }
                                    r#compare_to_source_path = Some(map_access.next_value()?);
                                }
                            }
                            Field::CompareToSourcePathPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#compare_to_source_path.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_compareToSourcePath",
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
                                        "compareToSourcePath",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::ContentType => {
                                if _ctx.from_json {
                                    let some = r#content_type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "contentType",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#content_type.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "contentType",
                                        ));
                                    }
                                    r#content_type = Some(map_access.next_value()?);
                                }
                            }
                            Field::ContentTypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#content_type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_contentType",
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
                                        "contentType",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::Expression => {
                                if _ctx.from_json {
                                    let some = r#expression.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "expression",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#expression.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "expression",
                                        ));
                                    }
                                    r#expression = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExpressionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#expression.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_expression",
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
                                        "expression",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::HeaderField => {
                                if _ctx.from_json {
                                    let some = r#header_field.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "headerField",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#header_field.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "headerField",
                                        ));
                                    }
                                    r#header_field = Some(map_access.next_value()?);
                                }
                            }
                            Field::HeaderFieldPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#header_field.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_headerField",
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
                                        "headerField",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::MinimumId => {
                                if _ctx.from_json {
                                    let some = r#minimum_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("minimumId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#minimum_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("minimumId"));
                                    }
                                    r#minimum_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::MinimumIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#minimum_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minimumId",
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
                                        "minimumId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::NavigationLinks => {
                                if _ctx.from_json {
                                    let some = r#navigation_links.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "navigationLinks",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#navigation_links.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "navigationLinks",
                                        ));
                                    }
                                    r#navigation_links = Some(map_access.next_value()?);
                                }
                            }
                            Field::NavigationLinksPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#navigation_links.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_navigationLinks",
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
                                        "navigationLinks",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::Operator => {
                                if _ctx.from_json {
                                    let some = r#operator.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("operator"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#operator.is_some() {
                                        return Err(serde::de::Error::duplicate_field("operator"));
                                    }
                                    r#operator = Some(map_access.next_value()?);
                                }
                            }
                            Field::OperatorPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#operator.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_operator"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "operator",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::Path => {
                                if _ctx.from_json {
                                    let some = r#path.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("path"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#path.is_some() {
                                        return Err(serde::de::Error::duplicate_field("path"));
                                    }
                                    r#path = Some(map_access.next_value()?);
                                }
                            }
                            Field::PathPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#path.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_path"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "path",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::RequestMethod => {
                                if _ctx.from_json {
                                    let some = r#request_method.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requestMethod",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#request_method.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requestMethod",
                                        ));
                                    }
                                    r#request_method = Some(map_access.next_value()?);
                                }
                            }
                            Field::RequestMethodPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#request_method.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_requestMethod",
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
                                        "requestMethod",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::RequestUrl => {
                                if _ctx.from_json {
                                    let some = r#request_url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requestURL",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#request_url.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requestURL",
                                        ));
                                    }
                                    r#request_url = Some(map_access.next_value()?);
                                }
                            }
                            Field::RequestUrlPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#request_url.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_requestURL",
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
                                        "requestURL",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::Resource => {
                                if _ctx.from_json {
                                    let some = r#resource.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("resource"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#resource.is_some() {
                                        return Err(serde::de::Error::duplicate_field("resource"));
                                    }
                                    r#resource = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResourcePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#resource.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_resource"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "resource",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::Response => {
                                if _ctx.from_json {
                                    let some = r#response.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("response"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#response.is_some() {
                                        return Err(serde::de::Error::duplicate_field("response"));
                                    }
                                    r#response = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResponsePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#response.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_response"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "response",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::ResponseCode => {
                                if _ctx.from_json {
                                    let some = r#response_code.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "responseCode",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#response_code.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "responseCode",
                                        ));
                                    }
                                    r#response_code = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResponseCodePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#response_code.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_responseCode",
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
                                        "responseCode",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::SourceId => {
                                if _ctx.from_json {
                                    let some = r#source_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sourceId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#source_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sourceId"));
                                    }
                                    r#source_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::SourceIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#source_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_sourceId"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sourceId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::ValidateProfileId => {
                                if _ctx.from_json {
                                    let some =
                                        r#validate_profile_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "validateProfileId",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#validate_profile_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "validateProfileId",
                                        ));
                                    }
                                    r#validate_profile_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValidateProfileIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#validate_profile_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_validateProfileId",
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
                                        "validateProfileId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
                                        ],
                                    ));
                                }
                            }
                            Field::WarningOnly => {
                                if _ctx.from_json {
                                    let some = r#warning_only.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "warningOnly",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#warning_only.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "warningOnly",
                                        ));
                                    }
                                    r#warning_only = Some(map_access.next_value()?);
                                }
                            }
                            Field::WarningOnlyPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#warning_only.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_warningOnly",
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
                                        "warningOnly",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "label",
                                            "description",
                                            "direction",
                                            "compareToSourceId",
                                            "compareToSourceExpression",
                                            "compareToSourcePath",
                                            "contentType",
                                            "expression",
                                            "headerField",
                                            "minimumId",
                                            "navigationLinks",
                                            "operator",
                                            "path",
                                            "requestMethod",
                                            "requestURL",
                                            "resource",
                                            "response",
                                            "responseCode",
                                            "sourceId",
                                            "validateProfileId",
                                            "value",
                                            "warningOnly",
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
                                        "label",
                                        "description",
                                        "direction",
                                        "compareToSourceId",
                                        "compareToSourceExpression",
                                        "compareToSourcePath",
                                        "contentType",
                                        "expression",
                                        "headerField",
                                        "minimumId",
                                        "navigationLinks",
                                        "operator",
                                        "path",
                                        "requestMethod",
                                        "requestURL",
                                        "resource",
                                        "response",
                                        "responseCode",
                                        "sourceId",
                                        "validateProfileId",
                                        "value",
                                        "warningOnly",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptSetupActionAssert {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#label,
                        r#description,
                        r#direction,
                        r#compare_to_source_id,
                        r#compare_to_source_expression,
                        r#compare_to_source_path,
                        r#content_type,
                        r#expression,
                        r#header_field,
                        r#minimum_id,
                        r#navigation_links,
                        r#operator,
                        r#path,
                        r#request_method,
                        r#request_url,
                        r#resource,
                        r#response,
                        r#response_code,
                        r#source_id,
                        r#validate_profile_id,
                        r#value,
                        r#warning_only: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#warning_only.unwrap_or(Default::default())
                        } else {
                            r#warning_only.ok_or(serde::de::Error::missing_field("warningOnly"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Action would contain either an operation or an assertion."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptSetupAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The operation to perform."]
    pub r#operation: Option<TestScriptSetupActionOperation>,
    #[doc = "Evaluates the results of previous operations to determine if the server under test behaves appropriately."]
    pub r#assert: Option<TestScriptSetupActionAssert>,
}
impl serde::ser::Serialize for TestScriptSetupAction {
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
            if let Some(some) = self.r#operation.as_ref() {
                state.serialize_entry("operation", some)?;
            }
            if let Some(some) = self.r#assert.as_ref() {
                state.serialize_entry("assert", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptSetupAction {
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
            #[serde(rename = "operation")]
            Operation,
            #[serde(rename = "assert")]
            Assert,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptSetupAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptSetupAction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#operation: Option<TestScriptSetupActionOperation> = None;
                let mut r#assert: Option<TestScriptSetupActionAssert> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Operation => {
                                if r#operation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operation"));
                                }
                                r#operation = Some(map_access.next_value()?);
                            }
                            Field::Assert => {
                                if r#assert.is_some() {
                                    return Err(serde::de::Error::duplicate_field("assert"));
                                }
                                r#assert = Some(map_access.next_value()?);
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
                                        "operation",
                                        "assert",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptSetupAction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#operation,
                        r#assert,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A series of required setup operations before tests are executed."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptSetup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Action would contain either an operation or an assertion."]
    pub r#action: Vec<TestScriptSetupAction>,
}
impl serde::ser::Serialize for TestScriptSetup {
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
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptSetup {
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
            #[serde(rename = "action")]
            Action,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptSetup;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetup")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptSetup, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#action: Option<Vec<TestScriptSetupAction>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Action => {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "action"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptSetup {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#action: r#action.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Action would contain either an operation or an assertion."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptTestAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An operation would involve a REST request to a server."]
    pub r#operation: Option<TestScriptSetupActionOperation>,
    #[doc = "Evaluates the results of previous operations to determine if the server under test behaves appropriately."]
    pub r#assert: Option<TestScriptSetupActionAssert>,
}
impl serde::ser::Serialize for TestScriptTestAction {
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
            if let Some(some) = self.r#operation.as_ref() {
                state.serialize_entry("operation", some)?;
            }
            if let Some(some) = self.r#assert.as_ref() {
                state.serialize_entry("assert", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptTestAction {
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
            #[serde(rename = "operation")]
            Operation,
            #[serde(rename = "assert")]
            Assert,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptTestAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTestAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTestAction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#operation: Option<TestScriptSetupActionOperation> = None;
                let mut r#assert: Option<TestScriptSetupActionAssert> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Operation => {
                                if r#operation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operation"));
                                }
                                r#operation = Some(map_access.next_value()?);
                            }
                            Field::Assert => {
                                if r#assert.is_some() {
                                    return Err(serde::de::Error::duplicate_field("assert"));
                                }
                                r#assert = Some(map_access.next_value()?);
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
                                        "operation",
                                        "assert",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptTestAction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#operation,
                        r#assert,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A test in this script."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptTest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of this test used for tracking/logging purposes by test engines."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short description of the test used by test engines for tracking and reporting purposes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Action would contain either an operation or an assertion."]
    pub r#action: Vec<TestScriptTestAction>,
}
impl serde::ser::Serialize for TestScriptTest {
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
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptTest {
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
            #[serde(rename = "action")]
            Action,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptTest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#action: Option<Vec<TestScriptTestAction>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
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
                                            "description",
                                            "action",
                                        ],
                                    ));
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
                                            "name",
                                            "description",
                                            "action",
                                        ],
                                    ));
                                }
                            }
                            Field::Action => {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action = Some(map_access.next_value()?);
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
                                        "description",
                                        "action",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptTest {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name,
                        r#description,
                        r#action: r#action.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The teardown action will only contain an operation."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptTeardownAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An operation would involve a REST request to a server."]
    pub r#operation: TestScriptSetupActionOperation,
}
impl serde::ser::Serialize for TestScriptTeardownAction {
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
            state.serialize_entry("operation", &self.r#operation)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptTeardownAction {
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
            #[serde(rename = "operation")]
            Operation,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptTeardownAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTeardownAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTeardownAction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#operation: Option<TestScriptSetupActionOperation> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Operation => {
                                if r#operation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operation"));
                                }
                                r#operation = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "operation"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptTeardownAction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#operation: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#operation.unwrap_or(Default::default())
                        } else {
                            r#operation.ok_or(serde::de::Error::missing_field("operation"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A series of operations required to clean up after all the tests are executed (successfully or otherwise)."]
#[derive(Default, Debug, Clone)]
pub struct TestScriptTeardown {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The teardown action will only contain an operation."]
    pub r#action: Vec<TestScriptTeardownAction>,
}
impl serde::ser::Serialize for TestScriptTeardown {
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
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScriptTeardown {
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
            #[serde(rename = "action")]
            Action,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScriptTeardown;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTeardown")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTeardown, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#action: Option<Vec<TestScriptTeardownAction>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Action => {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "action"],
                                ));
                            },
                        }
                    }
                    Ok(TestScriptTeardown {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#action: r#action.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification."]
#[derive(Default, Debug, Clone)]
pub struct TestScript {
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
    #[doc = "An absolute URI that is used to identify this test script when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this test script is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the test script is stored on different servers."]
    pub r#url: super::super::types::Uri,
    #[doc = "A formal identifier that is used to identify this test script when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the test script when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the test script author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the test script. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "A short, descriptive, user-friendly title for the test script."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this test script. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this test script is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the test script was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the test script changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the test script."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the test script from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate test script instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the test script is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this test script is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the test script and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the test script."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "An abstract server used in operations within this test script in the origin element."]
    pub r#origin: Vec<TestScriptOrigin>,
    #[doc = "An abstract server used in operations within this test script in the destination element."]
    pub r#destination: Vec<TestScriptDestination>,
    #[doc = "The required capability must exist and are assumed to function correctly on the FHIR server being tested."]
    pub r#metadata: Option<TestScriptMetadata>,
    #[doc = "Fixture in the test script - by reference (uri). All fixtures are required for the test script to execute."]
    pub r#fixture: Vec<TestScriptFixture>,
    #[doc = "Reference to the profile to be used for validation."]
    pub r#profile: Vec<Box<super::super::types::Reference>>,
    #[doc = "Variable is set based either on element value in response body or on header field value in the response headers."]
    pub r#variable: Vec<TestScriptVariable>,
    #[doc = "A series of required setup operations before tests are executed."]
    pub r#setup: Option<TestScriptSetup>,
    #[doc = "A test in this script."]
    pub r#test: Vec<TestScriptTest>,
    #[doc = "A series of operations required to clean up after all the tests are executed (successfully or otherwise)."]
    pub r#teardown: Option<TestScriptTeardown>,
}
impl crate::AnyResource for TestScript {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for TestScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "TestScript")?;
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
                if let Some(some) = self.r#url.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#url.id.as_ref(),
                        extension: &self.r#url.extension,
                    };
                    state.serialize_entry("_url", &primitive_element)?;
                }
            } else {
                state.serialize_entry("url", &self.r#url)?;
            }
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
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
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
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
            if !self.r#origin.is_empty() {
                state.serialize_entry("origin", &self.r#origin)?;
            }
            if !self.r#destination.is_empty() {
                state.serialize_entry("destination", &self.r#destination)?;
            }
            if let Some(some) = self.r#metadata.as_ref() {
                state.serialize_entry("metadata", some)?;
            }
            if !self.r#fixture.is_empty() {
                state.serialize_entry("fixture", &self.r#fixture)?;
            }
            if !self.r#profile.is_empty() {
                state.serialize_entry("profile", &self.r#profile)?;
            }
            if !self.r#variable.is_empty() {
                state.serialize_entry("variable", &self.r#variable)?;
            }
            if let Some(some) = self.r#setup.as_ref() {
                state.serialize_entry("setup", some)?;
            }
            if !self.r#test.is_empty() {
                state.serialize_entry("test", &self.r#test)?;
            }
            if let Some(some) = self.r#teardown.as_ref() {
                state.serialize_entry("teardown", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for TestScript {
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
            #[serde(rename = "identifier")]
            Identifier,
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
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "destination")]
            Destination,
            #[serde(rename = "metadata")]
            Metadata,
            #[serde(rename = "fixture")]
            Fixture,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "variable")]
            Variable,
            #[serde(rename = "setup")]
            Setup,
            #[serde(rename = "test")]
            Test,
            #[serde(rename = "teardown")]
            Teardown,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TestScript;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScript")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScript, V::Error>
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
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
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
                let mut r#origin: Option<Vec<TestScriptOrigin>> = None;
                let mut r#destination: Option<Vec<TestScriptDestination>> = None;
                let mut r#metadata: Option<TestScriptMetadata> = None;
                let mut r#fixture: Option<Vec<TestScriptFixture>> = None;
                let mut r#profile: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#variable: Option<Vec<TestScriptVariable>> = None;
                let mut r#setup: Option<TestScriptSetup> = None;
                let mut r#test: Option<Vec<TestScriptTest>> = None;
                let mut r#teardown: Option<TestScriptTeardown> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "TestScript" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"TestScript",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
                                        ],
                                    ));
                                }
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
                                        ],
                                    ));
                                }
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
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
                                            "identifier",
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
                                            "origin",
                                            "destination",
                                            "metadata",
                                            "fixture",
                                            "profile",
                                            "variable",
                                            "setup",
                                            "test",
                                            "teardown",
                                        ],
                                    ));
                                }
                            }
                            Field::Origin => {
                                if r#origin.is_some() {
                                    return Err(serde::de::Error::duplicate_field("origin"));
                                }
                                r#origin = Some(map_access.next_value()?);
                            }
                            Field::Destination => {
                                if r#destination.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                r#destination = Some(map_access.next_value()?);
                            }
                            Field::Metadata => {
                                if r#metadata.is_some() {
                                    return Err(serde::de::Error::duplicate_field("metadata"));
                                }
                                r#metadata = Some(map_access.next_value()?);
                            }
                            Field::Fixture => {
                                if r#fixture.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixture"));
                                }
                                r#fixture = Some(map_access.next_value()?);
                            }
                            Field::Profile => {
                                if r#profile.is_some() {
                                    return Err(serde::de::Error::duplicate_field("profile"));
                                }
                                r#profile = Some(map_access.next_value()?);
                            }
                            Field::Variable => {
                                if r#variable.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                r#variable = Some(map_access.next_value()?);
                            }
                            Field::Setup => {
                                if r#setup.is_some() {
                                    return Err(serde::de::Error::duplicate_field("setup"));
                                }
                                r#setup = Some(map_access.next_value()?);
                            }
                            Field::Test => {
                                if r#test.is_some() {
                                    return Err(serde::de::Error::duplicate_field("test"));
                                }
                                r#test = Some(map_access.next_value()?);
                            }
                            Field::Teardown => {
                                if r#teardown.is_some() {
                                    return Err(serde::de::Error::duplicate_field("teardown"));
                                }
                                r#teardown = Some(map_access.next_value()?);
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
                                        "identifier",
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
                                        "origin",
                                        "destination",
                                        "metadata",
                                        "fixture",
                                        "profile",
                                        "variable",
                                        "setup",
                                        "test",
                                        "teardown",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(TestScript {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#url.unwrap_or(Default::default())
                        } else {
                            r#url.ok_or(serde::de::Error::missing_field("url"))?
                        },
                        r#identifier,
                        r#version,
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#title,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#experimental,
                        r#date,
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#description,
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#purpose,
                        r#copyright,
                        r#origin: r#origin.unwrap_or(vec![]),
                        r#destination: r#destination.unwrap_or(vec![]),
                        r#metadata,
                        r#fixture: r#fixture.unwrap_or(vec![]),
                        r#profile: r#profile.unwrap_or(vec![]),
                        r#variable: r#variable.unwrap_or(vec![]),
                        r#setup,
                        r#test: r#test.unwrap_or(vec![]),
                        r#teardown,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
