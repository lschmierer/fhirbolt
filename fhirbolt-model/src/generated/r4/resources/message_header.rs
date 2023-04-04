// Generated on 2023-04-04 by fhirbolt-codegen v0.1.0
#[doc = "Code that identifies the event this message represents and connects it with its definition. Events defined as part of the FHIR specification have the system value \"<http://terminology.hl7.org/CodeSystem/message>-events\".  Alternatively uri to the EventDefinition."]
#[derive(Debug, Clone)]
pub enum MessageHeaderEvent {
    Coding(Box<super::super::types::Coding>),
    Uri(Box<super::super::types::Uri>),
    Invalid,
}
impl Default for MessageHeaderEvent {
    fn default() -> MessageHeaderEvent {
        MessageHeaderEvent::Invalid
    }
}
#[doc = "The destination application which the message is intended for."]
#[derive(Default, Debug, Clone)]
pub struct MessageHeaderDestination {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Human-readable name for the target system."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Identifies the target end system in situations where the initial message transmission is to an intermediary system."]
    pub r#target: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates where the message should be routed to."]
    pub r#endpoint: super::super::types::Url,
    #[doc = "Allows data conveyed by a message to be addressed to a particular person or department when routing to a specific application isn't sufficient."]
    pub r#receiver: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MessageHeaderDestination {
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
            if let Some(some) = self.r#target.as_ref() {
                state.serialize_entry("target", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#endpoint.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("endpoint", &some)?;
                }
                if self.r#endpoint.id.is_some() || !self.r#endpoint.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#endpoint.id.as_ref(),
                        extension: &self.r#endpoint.extension,
                    };
                    state.serialize_entry("_endpoint", &primitive_element)?;
                }
            } else {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            if let Some(some) = self.r#receiver.as_ref() {
                state.serialize_entry("receiver", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MessageHeaderDestination {
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
            #[serde(rename = "target")]
            Target,
            #[serde(rename = "endpoint")]
            Endpoint,
            #[serde(rename = "_endpoint")]
            EndpointPrimitiveElement,
            #[serde(rename = "receiver")]
            Receiver,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MessageHeaderDestination;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MessageHeaderDestination")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MessageHeaderDestination, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#target: Option<Box<super::super::types::Reference>> = None;
                let mut r#endpoint: Option<super::super::types::Url> = None;
                let mut r#receiver: Option<Box<super::super::types::Reference>> = None;
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
                                            "target",
                                            "endpoint",
                                            "receiver",
                                        ],
                                    ));
                                }
                            }
                            Field::Target => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target = Some(map_access.next_value()?);
                            }
                            Field::Endpoint => {
                                if _ctx.from_json {
                                    let some = r#endpoint.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#endpoint.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    r#endpoint = Some(map_access.next_value()?);
                                }
                            }
                            Field::EndpointPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#endpoint.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_endpoint"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "endpoint",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "target",
                                            "endpoint",
                                            "receiver",
                                        ],
                                    ));
                                }
                            }
                            Field::Receiver => {
                                if r#receiver.is_some() {
                                    return Err(serde::de::Error::duplicate_field("receiver"));
                                }
                                r#receiver = Some(map_access.next_value()?);
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
                                        "target",
                                        "endpoint",
                                        "receiver",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MessageHeaderDestination {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name,
                        r#target,
                        r#endpoint: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#endpoint.unwrap_or(Default::default())
                        } else {
                            r#endpoint.ok_or(serde::de::Error::missing_field("endpoint"))?
                        },
                        r#receiver,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The source application from which this message originated."]
#[derive(Default, Debug, Clone)]
pub struct MessageHeaderSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Human-readable name for the source system."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "May include configuration or other information useful in debugging."]
    pub r#software: Option<super::super::types::String>,
    #[doc = "Can convey versions of multiple systems in situations where a message passes through multiple hands."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "An e-mail, phone, website or other contact point to use to resolve issues with message communications."]
    pub r#contact: Option<Box<super::super::types::ContactPoint>>,
    #[doc = "Identifies the routing target to send acknowledgements to."]
    pub r#endpoint: super::super::types::Url,
}
impl serde::ser::Serialize for MessageHeaderSource {
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
                if let Some(some) = self.r#software.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("software", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_software", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#software.as_ref() {
                    state.serialize_entry("software", some)?;
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
            if let Some(some) = self.r#contact.as_ref() {
                state.serialize_entry("contact", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#endpoint.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("endpoint", &some)?;
                }
                if self.r#endpoint.id.is_some() || !self.r#endpoint.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#endpoint.id.as_ref(),
                        extension: &self.r#endpoint.extension,
                    };
                    state.serialize_entry("_endpoint", &primitive_element)?;
                }
            } else {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MessageHeaderSource {
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
            #[serde(rename = "software")]
            Software,
            #[serde(rename = "_software")]
            SoftwarePrimitiveElement,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "endpoint")]
            Endpoint,
            #[serde(rename = "_endpoint")]
            EndpointPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MessageHeaderSource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MessageHeaderSource")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MessageHeaderSource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#software: Option<super::super::types::String> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#contact: Option<Box<super::super::types::ContactPoint>> = None;
                let mut r#endpoint: Option<super::super::types::Url> = None;
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
                                            "software",
                                            "version",
                                            "contact",
                                            "endpoint",
                                        ],
                                    ));
                                }
                            }
                            Field::Software => {
                                if _ctx.from_json {
                                    let some = r#software.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("software"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#software.is_some() {
                                        return Err(serde::de::Error::duplicate_field("software"));
                                    }
                                    r#software = Some(map_access.next_value()?);
                                }
                            }
                            Field::SoftwarePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#software.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_software"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "software",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "software",
                                            "version",
                                            "contact",
                                            "endpoint",
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
                                            "software",
                                            "version",
                                            "contact",
                                            "endpoint",
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
                            Field::Endpoint => {
                                if _ctx.from_json {
                                    let some = r#endpoint.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#endpoint.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    r#endpoint = Some(map_access.next_value()?);
                                }
                            }
                            Field::EndpointPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#endpoint.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_endpoint"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "endpoint",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "software",
                                            "version",
                                            "contact",
                                            "endpoint",
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
                                        "software",
                                        "version",
                                        "contact",
                                        "endpoint",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MessageHeaderSource {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name,
                        r#software,
                        r#version,
                        r#contact,
                        r#endpoint: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#endpoint.unwrap_or(Default::default())
                        } else {
                            r#endpoint.ok_or(serde::de::Error::missing_field("endpoint"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Information about the message that this message is a response to.  Only present if this message is a response."]
#[derive(Default, Debug, Clone)]
pub struct MessageHeaderResponse {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The MessageHeader.id of the message to which this message is a response."]
    pub r#identifier: super::super::types::Id,
    #[doc = "Code that identifies the type of response to the message - whether it was successful or not, and whether it should be resent or not."]
    pub r#code: super::super::types::Code,
    #[doc = "Full details of any issues found in the message."]
    pub r#details: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MessageHeaderResponse {
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
                if let Some(some) = self.r#identifier.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("identifier", &some)?;
                }
                if self.r#identifier.id.is_some() || !self.r#identifier.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#identifier.id.as_ref(),
                        extension: &self.r#identifier.extension,
                    };
                    state.serialize_entry("_identifier", &primitive_element)?;
                }
            } else {
                state.serialize_entry("identifier", &self.r#identifier)?;
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
            if let Some(some) = self.r#details.as_ref() {
                state.serialize_entry("details", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MessageHeaderResponse {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "_identifier")]
            IdentifierPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "details")]
            Details,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MessageHeaderResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MessageHeaderResponse")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MessageHeaderResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<super::super::types::Id> = None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#details: Option<Box<super::super::types::Reference>> = None;
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
                            Field::Identifier => {
                                if _ctx.from_json {
                                    let some = r#identifier.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                }
                            }
                            Field::IdentifierPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#identifier.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_identifier",
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
                                        "identifier",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "code",
                                            "details",
                                        ],
                                    ));
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
                                            "identifier",
                                            "code",
                                            "details",
                                        ],
                                    ));
                                }
                            }
                            Field::Details => {
                                if r#details.is_some() {
                                    return Err(serde::de::Error::duplicate_field("details"));
                                }
                                r#details = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "code",
                                        "details",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MessageHeaderResponse {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#identifier.unwrap_or(Default::default())
                        } else {
                            r#identifier.ok_or(serde::de::Error::missing_field("identifier"))?
                        },
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#details,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The header for a message exchange that is either requesting or responding to an action.  The reference(s) that are the subject of the action as well as other information related to the action are typically transmitted in a bundle in which the MessageHeader resource instance is the first resource in the bundle.\n\nMany implementations are not prepared to use REST and need a messaging based infrastructure."]
#[derive(Default, Debug, Clone)]
pub struct MessageHeader {
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
    #[doc = "Code that identifies the event this message represents and connects it with its definition. Events defined as part of the FHIR specification have the system value \"<http://terminology.hl7.org/CodeSystem/message>-events\".  Alternatively uri to the EventDefinition."]
    pub r#event: MessageHeaderEvent,
    #[doc = "The destination application which the message is intended for."]
    pub r#destination: Vec<MessageHeaderDestination>,
    #[doc = "Identifies the sending system to allow the use of a trust relationship."]
    pub r#sender: Option<Box<super::super::types::Reference>>,
    #[doc = "The person or device that performed the data entry leading to this message. When there is more than one candidate, pick the most proximal to the message. Can provide other enterers in extensions."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "The logical author of the message - the person or device that decided the described event should happen. When there is more than one candidate, pick the most proximal to the MessageHeader. Can provide other authors in extensions."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "The source application from which this message originated."]
    pub r#source: MessageHeaderSource,
    #[doc = "The person or organization that accepts overall responsibility for the contents of the message. The implication is that the message event happened under the policies of the responsible party."]
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    #[doc = "Coded indication of the cause for the event - indicates  a reason for the occurrence of the event that is a focus of this message."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information about the message that this message is a response to.  Only present if this message is a response."]
    pub r#response: Option<MessageHeaderResponse>,
    #[doc = "The actual data of the message - a reference to the root/focus class of the event."]
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    #[doc = "Permanent link to the MessageDefinition for this message."]
    pub r#definition: Option<super::super::types::Canonical>,
}
impl crate::AnyResource for MessageHeader {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for MessageHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MessageHeader")?;
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
            match self.r#event {
                MessageHeaderEvent::Coding(ref value) => {
                    state.serialize_entry("eventCoding", value)?;
                }
                MessageHeaderEvent::Uri(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("eventUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_eventUri", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("eventUri", value)?;
                    }
                }
                MessageHeaderEvent::Invalid => {
                    return Err(serde::ser::Error::custom("event is a required field"))
                }
            }
            if !self.r#destination.is_empty() {
                state.serialize_entry("destination", &self.r#destination)?;
            }
            if let Some(some) = self.r#sender.as_ref() {
                state.serialize_entry("sender", some)?;
            }
            if let Some(some) = self.r#enterer.as_ref() {
                state.serialize_entry("enterer", some)?;
            }
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            state.serialize_entry("source", &self.r#source)?;
            if let Some(some) = self.r#responsible.as_ref() {
                state.serialize_entry("responsible", some)?;
            }
            if let Some(some) = self.r#reason.as_ref() {
                state.serialize_entry("reason", some)?;
            }
            if let Some(some) = self.r#response.as_ref() {
                state.serialize_entry("response", some)?;
            }
            if !self.r#focus.is_empty() {
                state.serialize_entry("focus", &self.r#focus)?;
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
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MessageHeader {
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
            #[serde(rename = "eventCoding")]
            EventCoding,
            #[serde(rename = "eventUri")]
            EventUri,
            #[serde(rename = "_eventUri")]
            EventUriPrimitiveElement,
            #[serde(rename = "destination")]
            Destination,
            #[serde(rename = "sender")]
            Sender,
            #[serde(rename = "enterer")]
            Enterer,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "responsible")]
            Responsible,
            #[serde(rename = "reason")]
            Reason,
            #[serde(rename = "response")]
            Response,
            #[serde(rename = "focus")]
            Focus,
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "_definition")]
            DefinitionPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MessageHeader;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MessageHeader")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MessageHeader, V::Error>
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
                let mut r#event: Option<MessageHeaderEvent> = None;
                let mut r#destination: Option<Vec<MessageHeaderDestination>> = None;
                let mut r#sender: Option<Box<super::super::types::Reference>> = None;
                let mut r#enterer: Option<Box<super::super::types::Reference>> = None;
                let mut r#author: Option<Box<super::super::types::Reference>> = None;
                let mut r#source: Option<MessageHeaderSource> = None;
                let mut r#responsible: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#response: Option<MessageHeaderResponse> = None;
                let mut r#focus: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#definition: Option<super::super::types::Canonical> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "MessageHeader" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"MessageHeader",
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
                                            "eventCoding",
                                            "eventUri",
                                            "destination",
                                            "sender",
                                            "enterer",
                                            "author",
                                            "source",
                                            "responsible",
                                            "reason",
                                            "response",
                                            "focus",
                                            "definition",
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
                                            "eventCoding",
                                            "eventUri",
                                            "destination",
                                            "sender",
                                            "enterer",
                                            "author",
                                            "source",
                                            "responsible",
                                            "reason",
                                            "response",
                                            "focus",
                                            "definition",
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
                            Field::EventCoding => {
                                if r#event.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eventCoding"));
                                }
                                r#event =
                                    Some(MessageHeaderEvent::Coding(map_access.next_value()?));
                            }
                            Field::EventUri => {
                                if _ctx.from_json {
                                    let r#enum = r#event
                                        .get_or_insert(MessageHeaderEvent::Uri(Default::default()));
                                    if let MessageHeaderEvent::Uri(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "eventUri",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("event[x]"));
                                    }
                                } else {
                                    if r#event.is_some() {
                                        return Err(serde::de::Error::duplicate_field("eventUri"));
                                    }
                                    r#event =
                                        Some(MessageHeaderEvent::Uri(map_access.next_value()?));
                                }
                            }
                            Field::EventUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#event
                                        .get_or_insert(MessageHeaderEvent::Uri(Default::default()));
                                    if let MessageHeaderEvent::Uri(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_eventUri",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_event[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "eventUri",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "eventCoding",
                                            "eventUri",
                                            "destination",
                                            "sender",
                                            "enterer",
                                            "author",
                                            "source",
                                            "responsible",
                                            "reason",
                                            "response",
                                            "focus",
                                            "definition",
                                        ],
                                    ));
                                }
                            }
                            Field::Destination => {
                                if _ctx.from_json {
                                    if r#destination.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "destination",
                                        ));
                                    }
                                    r#destination = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#destination.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Sender => {
                                if r#sender.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sender"));
                                }
                                r#sender = Some(map_access.next_value()?);
                            }
                            Field::Enterer => {
                                if r#enterer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("enterer"));
                                }
                                r#enterer = Some(map_access.next_value()?);
                            }
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
                            }
                            Field::Responsible => {
                                if r#responsible.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responsible"));
                                }
                                r#responsible = Some(map_access.next_value()?);
                            }
                            Field::Reason => {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason = Some(map_access.next_value()?);
                            }
                            Field::Response => {
                                if r#response.is_some() {
                                    return Err(serde::de::Error::duplicate_field("response"));
                                }
                                r#response = Some(map_access.next_value()?);
                            }
                            Field::Focus => {
                                if _ctx.from_json {
                                    if r#focus.is_some() {
                                        return Err(serde::de::Error::duplicate_field("focus"));
                                    }
                                    r#focus = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#focus.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "eventCoding",
                                            "eventUri",
                                            "destination",
                                            "sender",
                                            "enterer",
                                            "author",
                                            "source",
                                            "responsible",
                                            "reason",
                                            "response",
                                            "focus",
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
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "eventCoding",
                                        "eventUri",
                                        "destination",
                                        "sender",
                                        "enterer",
                                        "author",
                                        "source",
                                        "responsible",
                                        "reason",
                                        "response",
                                        "focus",
                                        "definition",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MessageHeader {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#event: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#event.unwrap_or(Default::default())
                        } else {
                            r#event.ok_or(serde::de::Error::missing_field("event[x]"))?
                        },
                        r#destination: r#destination.unwrap_or(vec![]),
                        r#sender,
                        r#enterer,
                        r#author,
                        r#source: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#source.unwrap_or(Default::default())
                        } else {
                            r#source.ok_or(serde::de::Error::missing_field("source"))?
                        },
                        r#responsible,
                        r#reason,
                        r#response,
                        r#focus: r#focus.unwrap_or(vec![]),
                        r#definition,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
