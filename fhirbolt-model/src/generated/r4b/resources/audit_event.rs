// Generated on 2022-12-15 by fhirbolt-codegen v0.1.0
#[doc = "The  value of the extra detail."]
#[derive(Debug, Clone)]
pub enum AuditEventEntityDetailValue {
    String(Box<super::super::types::String>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    Invalid,
}
impl Default for AuditEventEntityDetailValue {
    fn default() -> AuditEventEntityDetailValue {
        AuditEventEntityDetailValue::Invalid
    }
}
#[doc = "Logical network location for application activity, if the activity has a network location."]
#[derive(Default, Debug, Clone)]
pub struct AuditEventAgentNetwork {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An identifier for the network access point of the user device for the audit event."]
    pub r#address: Option<super::super::types::String>,
    #[doc = "An identifier for the type of network access point that originated the audit event."]
    pub r#type: Option<super::super::types::Code>,
}
impl crate::AnyResource for AuditEventAgentNetwork {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for AuditEventAgentNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
                if let Some(some) = self.r#address.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("address", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_address", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#address.as_ref() {
                    state.serialize_entry("address", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventAgentNetwork {
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
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "_address")]
            AddressPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventAgentNetwork;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventAgentNetwork")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventAgentNetwork, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#address: Option<super::super::types::String> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Address => {
                                let some = r#address.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Type => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "address", "type"],
                                ));
                            },
                        }
                    }
                    Ok(AuditEventAgentNetwork {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#address,
                        r#type,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An actor taking an active role in the event or activity that is logged."]
#[derive(Default, Debug, Clone)]
pub struct AuditEventAgent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specification of the participation type the user plays when performing the event."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The security role that the user was acting under, that come from local codes defined by the access control security system (e.g. RBAC, ABAC) used in the local context."]
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reference to who this agent is that was involved in the event."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "Alternative agent Identifier. For a human, this should be a user identifier text string from authentication system. This identifier would be one known to a common authentication system (e.g. single sign-on), if available."]
    pub r#alt_id: Option<super::super::types::String>,
    #[doc = "Human-meaningful name for the agent."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Indicator that the user is or is not the requestor, or initiator, for the event being audited."]
    pub r#requestor: super::super::types::Boolean,
    #[doc = "Where the event occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The policy or plan that authorized the activity being recorded. Typically, a single activity may have multiple applicable policies, such as patient consent, guarantor funding, etc. The policy would also indicate the security token used."]
    pub r#policy: Vec<super::super::types::Uri>,
    #[doc = "Type of media involved. Used when the event is about exporting/importing onto media."]
    pub r#media: Option<Box<super::super::types::Coding>>,
    #[doc = "Logical network location for application activity, if the activity has a network location."]
    pub r#network: Option<AuditEventAgentNetwork>,
    #[doc = "The reason (purpose of use), specific to this agent, that was used during the event being recorded."]
    pub r#purpose_of_use: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for AuditEventAgent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if !self.r#role.is_empty() {
                state.serialize_entry("role", &self.r#role)?;
            }
            if let Some(some) = self.r#who.as_ref() {
                state.serialize_entry("who", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#alt_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("altId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_altId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#alt_id.as_ref() {
                    state.serialize_entry("altId", some)?;
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
                if let Some(some) = self.r#requestor.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("requestor", &some)?;
                }
                if self.r#requestor.id.is_some() || !self.r#requestor.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#requestor.id.as_ref(),
                        extension: &self.r#requestor.extension,
                    };
                    state.serialize_entry("_requestor", &primitive_element)?;
                }
            } else {
                state.serialize_entry("requestor", &self.r#requestor)?;
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if _ctx.output_json {
                if !self.r#policy.is_empty() {
                    let values = self
                        .r#policy
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("policy", &values)?;
                    }
                    let requires_elements = self
                        .r#policy
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#policy
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
                        state.serialize_entry("_policy", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#policy.is_empty() {
                    state.serialize_entry("policy", &self.r#policy)?;
                }
            }
            if let Some(some) = self.r#media.as_ref() {
                state.serialize_entry("media", some)?;
            }
            if let Some(some) = self.r#network.as_ref() {
                state.serialize_entry("network", some)?;
            }
            if !self.r#purpose_of_use.is_empty() {
                state.serialize_entry("purposeOfUse", &self.r#purpose_of_use)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventAgent {
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
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "who")]
            Who,
            #[serde(rename = "altId")]
            AltId,
            #[serde(rename = "_altId")]
            AltIdPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "requestor")]
            Requestor,
            #[serde(rename = "_requestor")]
            RequestorPrimitiveElement,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "policy")]
            Policy,
            #[serde(rename = "_policy")]
            PolicyPrimitiveElement,
            #[serde(rename = "media")]
            Media,
            #[serde(rename = "network")]
            Network,
            #[serde(rename = "purposeOfUse")]
            PurposeOfUse,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventAgent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventAgent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventAgent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#role: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#who: Option<Box<super::super::types::Reference>> = None;
                let mut r#alt_id: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#requestor: Option<super::super::types::Boolean> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#policy: Option<Vec<super::super::types::Uri>> = None;
                let mut r#media: Option<Box<super::super::types::Coding>> = None;
                let mut r#network: Option<AuditEventAgentNetwork> = None;
                let mut r#purpose_of_use: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Who => {
                                if r#who.is_some() {
                                    return Err(serde::de::Error::duplicate_field("who"));
                                }
                                r#who = Some(map_access.next_value()?);
                            }
                            Field::AltId => {
                                let some = r#alt_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("altId"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AltIdPrimitiveElement => {
                                let some = r#alt_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_altId"));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Requestor => {
                                let some = r#requestor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestor"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RequestorPrimitiveElement => {
                                let some = r#requestor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requestor"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::Policy => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#policy.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("policy"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::PolicyPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#policy.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_policy"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Media => {
                                if r#media.is_some() {
                                    return Err(serde::de::Error::duplicate_field("media"));
                                }
                                r#media = Some(map_access.next_value()?);
                            }
                            Field::Network => {
                                if r#network.is_some() {
                                    return Err(serde::de::Error::duplicate_field("network"));
                                }
                                r#network = Some(map_access.next_value()?);
                            }
                            Field::PurposeOfUse => {
                                if r#purpose_of_use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purposeOfUse"));
                                }
                                r#purpose_of_use = Some(map_access.next_value()?);
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
                                        "role",
                                        "who",
                                        "altId",
                                        "name",
                                        "requestor",
                                        "location",
                                        "policy",
                                        "media",
                                        "network",
                                        "purposeOfUse",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AuditEventAgent {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#role: r#role.unwrap_or(vec![]),
                        r#who,
                        r#alt_id,
                        r#name,
                        r#requestor: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#requestor.unwrap_or(Default::default())
                        } else {
                            r#requestor.ok_or(serde::de::Error::missing_field("requestor"))?
                        },
                        r#location,
                        r#policy: r#policy.unwrap_or(vec![]),
                        r#media,
                        r#network,
                        r#purpose_of_use: r#purpose_of_use.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The system that is reporting the event."]
#[derive(Default, Debug, Clone)]
pub struct AuditEventSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Logical source location within the healthcare enterprise network.  For example, a hospital or other provider location within a multi-entity provider group."]
    pub r#site: Option<super::super::types::String>,
    #[doc = "Identifier of the source where the event was detected."]
    pub r#observer: Box<super::super::types::Reference>,
    #[doc = "Code specifying the type of source where event originated."]
    pub r#type: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for AuditEventSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
                if let Some(some) = self.r#site.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("site", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_site", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#site.as_ref() {
                    state.serialize_entry("site", some)?;
                }
            }
            state.serialize_entry("observer", &self.r#observer)?;
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventSource {
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
            #[serde(rename = "site")]
            Site,
            #[serde(rename = "_site")]
            SitePrimitiveElement,
            #[serde(rename = "observer")]
            Observer,
            #[serde(rename = "type")]
            Type,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventSource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventSource")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventSource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#site: Option<super::super::types::String> = None;
                let mut r#observer: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::Coding>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Site => {
                                let some = r#site.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SitePrimitiveElement => {
                                let some = r#site.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_site"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Observer => {
                                if r#observer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("observer"));
                                }
                                r#observer = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
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
                                        "site",
                                        "observer",
                                        "type",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AuditEventSource {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#site,
                        r#observer: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#observer.unwrap_or(Default::default())
                        } else {
                            r#observer.ok_or(serde::de::Error::missing_field("observer"))?
                        },
                        r#type: r#type.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Tagged value pairs for conveying additional information about the entity."]
#[derive(Default, Debug, Clone)]
pub struct AuditEventEntityDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of extra detail provided in the value."]
    pub r#type: super::super::types::String,
    #[doc = "The  value of the extra detail."]
    pub r#value: AuditEventEntityDetailValue,
}
impl serde::ser::Serialize for AuditEventEntityDetail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            match self.r#value {
                AuditEventEntityDetailValue::String(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueString", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueString", value)?;
                    }
                }
                AuditEventEntityDetailValue::Base64Binary(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueBase64Binary", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueBase64Binary", value)?;
                    }
                }
                AuditEventEntityDetailValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventEntityDetail {
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
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueBase64Binary")]
            ValueBase64Binary,
            #[serde(rename = "_valueBase64Binary")]
            ValueBase64BinaryPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventEntityDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventEntityDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventEntityDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::String> = None;
                let mut r#value: Option<AuditEventEntityDetailValue> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::ValueString => {
                                let r#enum = r#value.get_or_insert(
                                    AuditEventEntityDetailValue::String(Default::default()),
                                );
                                if let AuditEventEntityDetailValue::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    AuditEventEntityDetailValue::String(Default::default()),
                                );
                                if let AuditEventEntityDetailValue::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            }
                            Field::ValueBase64Binary => {
                                let r#enum = r#value.get_or_insert(
                                    AuditEventEntityDetailValue::Base64Binary(Default::default()),
                                );
                                if let AuditEventEntityDetailValue::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBase64Binary",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueBase64BinaryPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    AuditEventEntityDetailValue::Base64Binary(Default::default()),
                                );
                                if let AuditEventEntityDetailValue::Base64Binary(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBase64Binary",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
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
                                        "valueString",
                                        "valueBase64Binary",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AuditEventEntityDetail {
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
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specific instances of data or objects that have been accessed."]
#[derive(Default, Debug, Clone)]
pub struct AuditEventEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies a specific instance of the entity. The reference should be version specific."]
    pub r#what: Option<Box<super::super::types::Reference>>,
    #[doc = "The type of the object that was involved in this audit event."]
    pub r#type: Option<Box<super::super::types::Coding>>,
    #[doc = "Code representing the role the entity played in the event being audited."]
    pub r#role: Option<Box<super::super::types::Coding>>,
    #[doc = "Identifier for the data life-cycle stage for the entity."]
    pub r#lifecycle: Option<Box<super::super::types::Coding>>,
    #[doc = "Security labels for the identified entity."]
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    #[doc = "A name of the entity in the audit event."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Text that describes the entity in more detail."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The query parameters for a query-type entities."]
    pub r#query: Option<super::super::types::Base64Binary>,
    #[doc = "Tagged value pairs for conveying additional information about the entity."]
    pub r#detail: Vec<AuditEventEntityDetail>,
}
impl serde::ser::Serialize for AuditEventEntity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#what.as_ref() {
                state.serialize_entry("what", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            if let Some(some) = self.r#lifecycle.as_ref() {
                state.serialize_entry("lifecycle", some)?;
            }
            if !self.r#security_label.is_empty() {
                state.serialize_entry("securityLabel", &self.r#security_label)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#query.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("query", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_query", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#query.as_ref() {
                    state.serialize_entry("query", some)?;
                }
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventEntity {
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
            #[serde(rename = "what")]
            What,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "lifecycle")]
            Lifecycle,
            #[serde(rename = "securityLabel")]
            SecurityLabel,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "query")]
            Query,
            #[serde(rename = "_query")]
            QueryPrimitiveElement,
            #[serde(rename = "detail")]
            Detail,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventEntity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventEntity")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventEntity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#what: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
                let mut r#role: Option<Box<super::super::types::Coding>> = None;
                let mut r#lifecycle: Option<Box<super::super::types::Coding>> = None;
                let mut r#security_label: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#query: Option<super::super::types::Base64Binary> = None;
                let mut r#detail: Option<Vec<AuditEventEntityDetail>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::What => {
                                if r#what.is_some() {
                                    return Err(serde::de::Error::duplicate_field("what"));
                                }
                                r#what = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Lifecycle => {
                                if r#lifecycle.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lifecycle"));
                                }
                                r#lifecycle = Some(map_access.next_value()?);
                            }
                            Field::SecurityLabel => {
                                if r#security_label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("securityLabel"));
                                }
                                r#security_label = Some(map_access.next_value()?);
                            }
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Query => {
                                let some = r#query.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("query"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::QueryPrimitiveElement => {
                                let some = r#query.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_query"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Detail => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                r#detail = Some(map_access.next_value()?);
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
                                        "what",
                                        "type",
                                        "role",
                                        "lifecycle",
                                        "securityLabel",
                                        "name",
                                        "description",
                                        "query",
                                        "detail",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AuditEventEntity {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#what,
                        r#type,
                        r#role,
                        r#lifecycle,
                        r#security_label: r#security_label.unwrap_or(vec![]),
                        r#name,
                        r#description,
                        r#query,
                        r#detail: r#detail.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A record of an event made for purposes of maintaining a security log. Typical uses include detection of intrusion attempts and monitoring for inappropriate usage."]
#[derive(Default, Debug, Clone)]
pub struct AuditEvent {
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
    #[doc = "Identifier for a family of the event.  For example, a menu item, program, rule, policy, function code, application name or URL. It identifies the performed function."]
    pub r#type: Box<super::super::types::Coding>,
    #[doc = "Identifier for the category of event."]
    pub r#subtype: Vec<Box<super::super::types::Coding>>,
    #[doc = "Indicator for type of action performed during the event that generated the audit."]
    pub r#action: Option<super::super::types::Code>,
    #[doc = "The period during which the activity occurred."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The time when the event was recorded."]
    pub r#recorded: super::super::types::Instant,
    #[doc = "Indicates whether the event succeeded or failed."]
    pub r#outcome: Option<super::super::types::Code>,
    #[doc = "A free text description of the outcome of the event."]
    pub r#outcome_desc: Option<super::super::types::String>,
    #[doc = "The purposeOfUse (reason) that was used during the event being recorded."]
    pub r#purpose_of_event: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An actor taking an active role in the event or activity that is logged."]
    pub r#agent: Vec<AuditEventAgent>,
    #[doc = "The system that is reporting the event."]
    pub r#source: AuditEventSource,
    #[doc = "Specific instances of data or objects that have been accessed."]
    pub r#entity: Vec<AuditEventEntity>,
}
impl serde::ser::Serialize for AuditEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "AuditEvent")?;
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
            state.serialize_entry("type", &self.r#type)?;
            if !self.r#subtype.is_empty() {
                state.serialize_entry("subtype", &self.r#subtype)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#action.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("action", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_action", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#action.as_ref() {
                    state.serialize_entry("action", some)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#recorded.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("recorded", &some)?;
                }
                if self.r#recorded.id.is_some() || !self.r#recorded.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#recorded.id.as_ref(),
                        extension: &self.r#recorded.extension,
                    };
                    state.serialize_entry("_recorded", &primitive_element)?;
                }
            } else {
                state.serialize_entry("recorded", &self.r#recorded)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#outcome.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("outcome", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_outcome", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#outcome.as_ref() {
                    state.serialize_entry("outcome", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#outcome_desc.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("outcomeDesc", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_outcomeDesc", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#outcome_desc.as_ref() {
                    state.serialize_entry("outcomeDesc", some)?;
                }
            }
            if !self.r#purpose_of_event.is_empty() {
                state.serialize_entry("purposeOfEvent", &self.r#purpose_of_event)?;
            }
            if !self.r#agent.is_empty() {
                state.serialize_entry("agent", &self.r#agent)?;
            }
            state.serialize_entry("source", &self.r#source)?;
            if !self.r#entity.is_empty() {
                state.serialize_entry("entity", &self.r#entity)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEvent {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subtype")]
            Subtype,
            #[serde(rename = "action")]
            Action,
            #[serde(rename = "_action")]
            ActionPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "recorded")]
            Recorded,
            #[serde(rename = "_recorded")]
            RecordedPrimitiveElement,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "_outcome")]
            OutcomePrimitiveElement,
            #[serde(rename = "outcomeDesc")]
            OutcomeDesc,
            #[serde(rename = "_outcomeDesc")]
            OutcomeDescPrimitiveElement,
            #[serde(rename = "purposeOfEvent")]
            PurposeOfEvent,
            #[serde(rename = "agent")]
            Agent,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "entity")]
            Entity,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEvent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEvent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEvent, V::Error>
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
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
                let mut r#subtype: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#action: Option<super::super::types::Code> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#recorded: Option<super::super::types::Instant> = None;
                let mut r#outcome: Option<super::super::types::Code> = None;
                let mut r#outcome_desc: Option<super::super::types::String> = None;
                let mut r#purpose_of_event: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#agent: Option<Vec<AuditEventAgent>> = None;
                let mut r#source: Option<AuditEventSource> = None;
                let mut r#entity: Option<Vec<AuditEventEntity>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "AuditEvent" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"AuditEvent",
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
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
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
                            }
                            Field::Language => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Subtype => {
                                if r#subtype.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtype"));
                                }
                                r#subtype = Some(map_access.next_value()?);
                            }
                            Field::Action => {
                                let some = r#action.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ActionPrimitiveElement => {
                                let some = r#action.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_action"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Recorded => {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RecordedPrimitiveElement => {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recorded"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Outcome => {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::OutcomePrimitiveElement => {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_outcome"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::OutcomeDesc => {
                                let some = r#outcome_desc.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcomeDesc"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::OutcomeDescPrimitiveElement => {
                                let some = r#outcome_desc.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_outcomeDesc"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::PurposeOfEvent => {
                                if r#purpose_of_event.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "purposeOfEvent",
                                    ));
                                }
                                r#purpose_of_event = Some(map_access.next_value()?);
                            }
                            Field::Agent => {
                                if r#agent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("agent"));
                                }
                                r#agent = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
                            }
                            Field::Entity => {
                                if r#entity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entity"));
                                }
                                r#entity = Some(map_access.next_value()?);
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
                                        "type",
                                        "subtype",
                                        "action",
                                        "period",
                                        "recorded",
                                        "outcome",
                                        "outcomeDesc",
                                        "purposeOfEvent",
                                        "agent",
                                        "source",
                                        "entity",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AuditEvent {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#subtype: r#subtype.unwrap_or(vec![]),
                        r#action,
                        r#period,
                        r#recorded: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#recorded.unwrap_or(Default::default())
                        } else {
                            r#recorded.ok_or(serde::de::Error::missing_field("recorded"))?
                        },
                        r#outcome,
                        r#outcome_desc,
                        r#purpose_of_event: r#purpose_of_event.unwrap_or(vec![]),
                        r#agent: r#agent.unwrap_or(vec![]),
                        r#source: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#source.unwrap_or(Default::default())
                        } else {
                            r#source.ok_or(serde::de::Error::missing_field("source"))?
                        },
                        r#entity: r#entity.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
