// Generated on 2023-04-04 by fhirbolt-codegen v0.1.0
#[doc = "The period during which the activity occurred."]
#[derive(Debug, Clone)]
pub enum ProvenanceOccurred {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for ProvenanceOccurred {
    fn default() -> ProvenanceOccurred {
        ProvenanceOccurred::Invalid
    }
}
#[doc = "An actor taking a role in an activity  for which it can be assigned some degree of responsibility for the activity taking place."]
#[derive(Default, Debug, Clone)]
pub struct ProvenanceAgent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The participation the agent had with respect to the activity."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The function of the agent with respect to the activity. The security role enabling the agent with respect to the activity."]
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The individual, device or organization that participated in the event."]
    pub r#who: Box<super::super::types::Reference>,
    #[doc = "The individual, device, or organization for whom the change was made."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ProvenanceAgent {
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
            if !self.r#role.is_empty() {
                state.serialize_entry("role", &self.r#role)?;
            }
            state.serialize_entry("who", &self.r#who)?;
            if let Some(some) = self.r#on_behalf_of.as_ref() {
                state.serialize_entry("onBehalfOf", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ProvenanceAgent {
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
            #[serde(rename = "onBehalfOf")]
            OnBehalfOf,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProvenanceAgent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProvenanceAgent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProvenanceAgent, V::Error>
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
                let mut r#on_behalf_of: Option<Box<super::super::types::Reference>> = None;
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
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if _ctx.from_json {
                                    if r#role.is_some() {
                                        return Err(serde::de::Error::duplicate_field("role"));
                                    }
                                    r#role = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#role.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Who => {
                                if r#who.is_some() {
                                    return Err(serde::de::Error::duplicate_field("who"));
                                }
                                r#who = Some(map_access.next_value()?);
                            }
                            Field::OnBehalfOf => {
                                if r#on_behalf_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                                }
                                r#on_behalf_of = Some(map_access.next_value()?);
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
                                        "onBehalfOf",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ProvenanceAgent {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#role: r#role.unwrap_or(vec![]),
                        r#who: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#who.unwrap_or(Default::default())
                        } else {
                            r#who.ok_or(serde::de::Error::missing_field("who"))?
                        },
                        r#on_behalf_of,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An entity used in this activity."]
#[derive(Default, Debug, Clone)]
pub struct ProvenanceEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the entity was used during the activity."]
    pub r#role: super::super::types::Code,
    #[doc = "Identity of the  Entity used. May be a logical or physical uri and maybe absolute or relative."]
    pub r#what: Box<super::super::types::Reference>,
    #[doc = "The entity is attributed to an agent to express the agent's responsibility for that entity, possibly along with other agents. This description can be understood as shorthand for saying that the agent was responsible for the activity which generated the entity."]
    pub r#agent: Vec<ProvenanceAgent>,
}
impl serde::ser::Serialize for ProvenanceEntity {
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
                if let Some(some) = self.r#role.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("role", &some)?;
                }
                if self.r#role.id.is_some() || !self.r#role.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#role.id.as_ref(),
                        extension: &self.r#role.extension,
                    };
                    state.serialize_entry("_role", &primitive_element)?;
                }
            } else {
                state.serialize_entry("role", &self.r#role)?;
            }
            state.serialize_entry("what", &self.r#what)?;
            if !self.r#agent.is_empty() {
                state.serialize_entry("agent", &self.r#agent)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ProvenanceEntity {
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
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "_role")]
            RolePrimitiveElement,
            #[serde(rename = "what")]
            What,
            #[serde(rename = "agent")]
            Agent,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProvenanceEntity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProvenanceEntity")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProvenanceEntity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#role: Option<super::super::types::Code> = None;
                let mut r#what: Option<Box<super::super::types::Reference>> = None;
                let mut r#agent: Option<Vec<ProvenanceAgent>> = None;
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
                            Field::Role => {
                                if _ctx.from_json {
                                    let some = r#role.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("role"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#role.is_some() {
                                        return Err(serde::de::Error::duplicate_field("role"));
                                    }
                                    r#role = Some(map_access.next_value()?);
                                }
                            }
                            Field::RolePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#role.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_role"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "role",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "role",
                                            "what",
                                            "agent",
                                        ],
                                    ));
                                }
                            }
                            Field::What => {
                                if r#what.is_some() {
                                    return Err(serde::de::Error::duplicate_field("what"));
                                }
                                r#what = Some(map_access.next_value()?);
                            }
                            Field::Agent => {
                                if _ctx.from_json {
                                    if r#agent.is_some() {
                                        return Err(serde::de::Error::duplicate_field("agent"));
                                    }
                                    r#agent = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#agent.get_or_insert(Default::default());
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
                                        "role",
                                        "what",
                                        "agent",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ProvenanceEntity {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#role: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#role.unwrap_or(Default::default())
                        } else {
                            r#role.ok_or(serde::de::Error::missing_field("role"))?
                        },
                        r#what: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#what.unwrap_or(Default::default())
                        } else {
                            r#what.ok_or(serde::de::Error::missing_field("what"))?
                        },
                        r#agent: r#agent.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for assessing authenticity, enabling trust, and allowing reproducibility. Provenance assertions are a form of contextual metadata and can themselves become important records with their own provenance. Provenance statement indicates clinical significance in terms of confidence in authenticity, reliability, and trustworthiness, integrity, and stage in lifecycle (e.g. Document Completion - has the artifact been legally authenticated), all of which may impact security, privacy, and trust policies."]
#[derive(Default, Debug, Clone)]
pub struct Provenance {
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
    #[doc = "The Reference(s) that were generated or updated by  the activity described in this resource. A provenance can point to more than one target if multiple resources were created/updated by the same activity."]
    pub r#target: Vec<Box<super::super::types::Reference>>,
    #[doc = "The period during which the activity occurred."]
    pub r#occurred: Option<ProvenanceOccurred>,
    #[doc = "The instant of time at which the activity was recorded."]
    pub r#recorded: super::super::types::Instant,
    #[doc = "Policy or plan the activity was defined by. Typically, a single activity may have multiple applicable policy documents, such as patient consent, guarantor funding, etc."]
    pub r#policy: Vec<super::super::types::Uri>,
    #[doc = "Where the activity occurred, if relevant."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason that the activity was taking place."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An activity is something that occurs over a period of time and acts upon or with entities; it may include consuming, processing, transforming, modifying, relocating, using, or generating entities."]
    pub r#activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An actor taking a role in an activity  for which it can be assigned some degree of responsibility for the activity taking place."]
    pub r#agent: Vec<ProvenanceAgent>,
    #[doc = "An entity used in this activity."]
    pub r#entity: Vec<ProvenanceEntity>,
    #[doc = "A digital signature on the target Reference(s). The signer should match a Provenance.agent. The purpose of the signature is indicated."]
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
impl crate::AnyResource for Provenance {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for Provenance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Provenance")?;
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
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            if let Some(some) = self.r#occurred.as_ref() {
                match some {
                    ProvenanceOccurred::Period(ref value) => {
                        state.serialize_entry("occurredPeriod", value)?;
                    }
                    ProvenanceOccurred::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("occurredDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_occurredDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("occurredDateTime", value)?;
                        }
                    }
                    ProvenanceOccurred::Invalid => {
                        return Err(serde::ser::Error::custom("occurred is invalid"))
                    }
                }
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
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if !self.r#reason.is_empty() {
                state.serialize_entry("reason", &self.r#reason)?;
            }
            if let Some(some) = self.r#activity.as_ref() {
                state.serialize_entry("activity", some)?;
            }
            if !self.r#agent.is_empty() {
                state.serialize_entry("agent", &self.r#agent)?;
            }
            if !self.r#entity.is_empty() {
                state.serialize_entry("entity", &self.r#entity)?;
            }
            if !self.r#signature.is_empty() {
                state.serialize_entry("signature", &self.r#signature)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Provenance {
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
            #[serde(rename = "target")]
            Target,
            #[serde(rename = "occurredPeriod")]
            OccurredPeriod,
            #[serde(rename = "occurredDateTime")]
            OccurredDateTime,
            #[serde(rename = "_occurredDateTime")]
            OccurredDateTimePrimitiveElement,
            #[serde(rename = "recorded")]
            Recorded,
            #[serde(rename = "_recorded")]
            RecordedPrimitiveElement,
            #[serde(rename = "policy")]
            Policy,
            #[serde(rename = "_policy")]
            PolicyPrimitiveElement,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "reason")]
            Reason,
            #[serde(rename = "activity")]
            Activity,
            #[serde(rename = "agent")]
            Agent,
            #[serde(rename = "entity")]
            Entity,
            #[serde(rename = "signature")]
            Signature,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Provenance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Provenance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Provenance, V::Error>
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
                let mut r#target: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#occurred: Option<ProvenanceOccurred> = None;
                let mut r#recorded: Option<super::super::types::Instant> = None;
                let mut r#policy: Option<Vec<super::super::types::Uri>> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#activity: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#agent: Option<Vec<ProvenanceAgent>> = None;
                let mut r#entity: Option<Vec<ProvenanceEntity>> = None;
                let mut r#signature: Option<Vec<Box<super::super::types::Signature>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Provenance" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Provenance",
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
                                            "target",
                                            "occurredPeriod",
                                            "occurredDateTime",
                                            "recorded",
                                            "policy",
                                            "location",
                                            "reason",
                                            "activity",
                                            "agent",
                                            "entity",
                                            "signature",
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
                                            "target",
                                            "occurredPeriod",
                                            "occurredDateTime",
                                            "recorded",
                                            "policy",
                                            "location",
                                            "reason",
                                            "activity",
                                            "agent",
                                            "entity",
                                            "signature",
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
                            Field::Target => {
                                if _ctx.from_json {
                                    if r#target.is_some() {
                                        return Err(serde::de::Error::duplicate_field("target"));
                                    }
                                    r#target = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#target.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::OccurredPeriod => {
                                if r#occurred.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurredPeriod",
                                    ));
                                }
                                r#occurred =
                                    Some(ProvenanceOccurred::Period(map_access.next_value()?));
                            }
                            Field::OccurredDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#occurred.get_or_insert(
                                        ProvenanceOccurred::DateTime(Default::default()),
                                    );
                                    if let ProvenanceOccurred::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "occurredDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurred[x]",
                                        ));
                                    }
                                } else {
                                    if r#occurred.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurredDateTime",
                                        ));
                                    }
                                    r#occurred = Some(ProvenanceOccurred::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::OccurredDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#occurred.get_or_insert(
                                        ProvenanceOccurred::DateTime(Default::default()),
                                    );
                                    if let ProvenanceOccurred::DateTime(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_occurredDateTime",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_occurred[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "occurredDateTime",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "occurredPeriod",
                                            "occurredDateTime",
                                            "recorded",
                                            "policy",
                                            "location",
                                            "reason",
                                            "activity",
                                            "agent",
                                            "entity",
                                            "signature",
                                        ],
                                    ));
                                }
                            }
                            Field::Recorded => {
                                if _ctx.from_json {
                                    let some = r#recorded.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("recorded"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#recorded.is_some() {
                                        return Err(serde::de::Error::duplicate_field("recorded"));
                                    }
                                    r#recorded = Some(map_access.next_value()?);
                                }
                            }
                            Field::RecordedPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "recorded",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "occurredPeriod",
                                            "occurredDateTime",
                                            "recorded",
                                            "policy",
                                            "location",
                                            "reason",
                                            "activity",
                                            "agent",
                                            "entity",
                                            "signature",
                                        ],
                                    ));
                                }
                            }
                            Field::Policy => {
                                if _ctx.from_json {
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
                                } else {
                                    let vec = r#policy.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PolicyPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "policy",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "occurredPeriod",
                                            "occurredDateTime",
                                            "recorded",
                                            "policy",
                                            "location",
                                            "reason",
                                            "activity",
                                            "agent",
                                            "entity",
                                            "signature",
                                        ],
                                    ));
                                }
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::Reason => {
                                if _ctx.from_json {
                                    if r#reason.is_some() {
                                        return Err(serde::de::Error::duplicate_field("reason"));
                                    }
                                    r#reason = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reason.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Activity => {
                                if r#activity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("activity"));
                                }
                                r#activity = Some(map_access.next_value()?);
                            }
                            Field::Agent => {
                                if _ctx.from_json {
                                    if r#agent.is_some() {
                                        return Err(serde::de::Error::duplicate_field("agent"));
                                    }
                                    r#agent = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#agent.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Entity => {
                                if _ctx.from_json {
                                    if r#entity.is_some() {
                                        return Err(serde::de::Error::duplicate_field("entity"));
                                    }
                                    r#entity = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#entity.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Signature => {
                                if _ctx.from_json {
                                    if r#signature.is_some() {
                                        return Err(serde::de::Error::duplicate_field("signature"));
                                    }
                                    r#signature = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#signature.get_or_insert(Default::default());
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
                                        "target",
                                        "occurredPeriod",
                                        "occurredDateTime",
                                        "recorded",
                                        "policy",
                                        "location",
                                        "reason",
                                        "activity",
                                        "agent",
                                        "entity",
                                        "signature",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Provenance {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#target: r#target.unwrap_or(vec![]),
                        r#occurred,
                        r#recorded: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#recorded.unwrap_or(Default::default())
                        } else {
                            r#recorded.ok_or(serde::de::Error::missing_field("recorded"))?
                        },
                        r#policy: r#policy.unwrap_or(vec![]),
                        r#location,
                        r#reason: r#reason.unwrap_or(vec![]),
                        r#activity,
                        r#agent: r#agent.unwrap_or(vec![]),
                        r#entity: r#entity.unwrap_or(vec![]),
                        r#signature: r#signature.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
