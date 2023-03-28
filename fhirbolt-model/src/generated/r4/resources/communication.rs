// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "A communicated content (or for multi-part communications, one portion of the communication)."]
#[derive(Debug, Clone)]
pub enum CommunicationPayloadContent {
    String(Box<super::super::types::String>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for CommunicationPayloadContent {
    fn default() -> CommunicationPayloadContent {
        CommunicationPayloadContent::Invalid
    }
}
#[doc = "Text, attachment(s), or resource(s) that was communicated to the recipient."]
#[derive(Default, Debug, Clone)]
pub struct CommunicationPayload {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A communicated content (or for multi-part communications, one portion of the communication)."]
    pub r#content: CommunicationPayloadContent,
}
impl serde::ser::Serialize for CommunicationPayload {
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
            match self.r#content {
                CommunicationPayloadContent::String(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("contentString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_contentString", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("contentString", value)?;
                    }
                }
                CommunicationPayloadContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                CommunicationPayloadContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                CommunicationPayloadContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CommunicationPayload {
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
            #[serde(rename = "contentString")]
            ContentString,
            #[serde(rename = "_contentString")]
            ContentStringPrimitiveElement,
            #[serde(rename = "contentAttachment")]
            ContentAttachment,
            #[serde(rename = "contentReference")]
            ContentReference,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CommunicationPayload;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CommunicationPayload")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CommunicationPayload, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<CommunicationPayloadContent> = None;
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
                            Field::ContentString => {
                                if _ctx.from_json {
                                    let r#enum = r#content.get_or_insert(
                                        CommunicationPayloadContent::String(Default::default()),
                                    );
                                    if let CommunicationPayloadContent::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "contentString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "content[x]",
                                        ));
                                    }
                                } else {
                                    if r#content.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "contentString",
                                        ));
                                    }
                                    r#content = Some(CommunicationPayloadContent::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ContentStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#content.get_or_insert(
                                        CommunicationPayloadContent::String(Default::default()),
                                    );
                                    if let CommunicationPayloadContent::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_contentString",
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
                                            "_content[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "contentString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "contentString",
                                            "contentAttachment",
                                            "contentReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ContentAttachment => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentAttachment",
                                    ));
                                }
                                r#content = Some(CommunicationPayloadContent::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ContentReference => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                r#content = Some(CommunicationPayloadContent::Reference(
                                    map_access.next_value()?,
                                ));
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
                                        "contentString",
                                        "contentAttachment",
                                        "contentReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CommunicationPayload {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#content: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#content.unwrap_or(Default::default())
                        } else {
                            r#content.ok_or(serde::de::Error::missing_field("content[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An occurrence of information being transmitted; e.g. an alert that was sent to a responsible provider, a public health agency that was notified about a reportable condition."]
#[derive(Default, Debug, Clone)]
pub struct Communication {
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
    #[doc = "Business identifiers assigned to this communication by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Communication."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Communication."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "An order, proposal or plan fulfilled in whole or in part by this Communication."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "Part of this action."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "Prior communication that this communication is in response to."]
    pub r#in_response_to: Vec<Box<super::super::types::Reference>>,
    #[doc = "The status of the transmission."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the Communication."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of message conveyed such as alert, notification, reminder, instruction, etc."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Characterizes how quickly the planned or in progress communication must be addressed. Includes concepts such as stat, urgent, routine."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A channel that was used for this communication (e.g. email, fax)."]
    pub r#medium: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient or group that was the focus of this communication."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Description of the purpose/content, similar to a subject line in an email."]
    pub r#topic: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Other resources that pertain to this communication and to which this communication should be associated."]
    pub r#about: Vec<Box<super::super::types::Reference>>,
    #[doc = "The Encounter during which this Communication was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The time when this communication was sent."]
    pub r#sent: Option<super::super::types::DateTime>,
    #[doc = "The time when this communication arrived at the destination."]
    pub r#received: Option<super::super::types::DateTime>,
    #[doc = "The entity (e.g. person, organization, clinical information system, care team or device) which was the target of the communication. If receipts need to be tracked by an individual, a separate resource instance will need to be created for each recipient.  Multiple recipient communications are intended where either receipts are not tracked (e.g. a mass mail-out) or a receipt is captured in aggregate (all emails confirmed received by a particular time)."]
    pub r#recipient: Vec<Box<super::super::types::Reference>>,
    #[doc = "The entity (e.g. person, organization, clinical information system, or device) which was the source of the communication."]
    pub r#sender: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason or justification for the communication."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource whose existence justifies this communication."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Text, attachment(s), or resource(s) that was communicated to the recipient."]
    pub r#payload: Vec<CommunicationPayload>,
    #[doc = "Additional notes or commentary about the communication by the sender, receiver or other interested parties."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl crate::AnyResource for Communication {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for Communication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Communication")?;
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if !self.r#instantiates_canonical.is_empty() {
                    let values = self
                        .r#instantiates_canonical
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesCanonical", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_canonical
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_canonical
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
                        state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_canonical.is_empty() {
                    state
                        .serialize_entry("instantiatesCanonical", &self.r#instantiates_canonical)?;
                }
            }
            if _ctx.output_json {
                if !self.r#instantiates_uri.is_empty() {
                    let values = self
                        .r#instantiates_uri
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesUri", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_uri
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_uri
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
                        state.serialize_entry("_instantiatesUri", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_uri.is_empty() {
                    state.serialize_entry("instantiatesUri", &self.r#instantiates_uri)?;
                }
            }
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
            }
            if !self.r#in_response_to.is_empty() {
                state.serialize_entry("inResponseTo", &self.r#in_response_to)?;
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
            if let Some(some) = self.r#status_reason.as_ref() {
                state.serialize_entry("statusReason", some)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#priority.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("priority", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_priority", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#priority.as_ref() {
                    state.serialize_entry("priority", some)?;
                }
            }
            if !self.r#medium.is_empty() {
                state.serialize_entry("medium", &self.r#medium)?;
            }
            if let Some(some) = self.r#subject.as_ref() {
                state.serialize_entry("subject", some)?;
            }
            if let Some(some) = self.r#topic.as_ref() {
                state.serialize_entry("topic", some)?;
            }
            if !self.r#about.is_empty() {
                state.serialize_entry("about", &self.r#about)?;
            }
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#sent.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sent", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sent", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sent.as_ref() {
                    state.serialize_entry("sent", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#received.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("received", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_received", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#received.as_ref() {
                    state.serialize_entry("received", some)?;
                }
            }
            if !self.r#recipient.is_empty() {
                state.serialize_entry("recipient", &self.r#recipient)?;
            }
            if let Some(some) = self.r#sender.as_ref() {
                state.serialize_entry("sender", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#payload.is_empty() {
                state.serialize_entry("payload", &self.r#payload)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Communication {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "inResponseTo")]
            InResponseTo,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "_priority")]
            PriorityPrimitiveElement,
            #[serde(rename = "medium")]
            Medium,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "topic")]
            Topic,
            #[serde(rename = "about")]
            About,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "sent")]
            Sent,
            #[serde(rename = "_sent")]
            SentPrimitiveElement,
            #[serde(rename = "received")]
            Received,
            #[serde(rename = "_received")]
            ReceivedPrimitiveElement,
            #[serde(rename = "recipient")]
            Recipient,
            #[serde(rename = "sender")]
            Sender,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "payload")]
            Payload,
            #[serde(rename = "note")]
            Note,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Communication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Communication")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Communication, V::Error>
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
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#in_response_to: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#medium: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#topic: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#about: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#sent: Option<super::super::types::DateTime> = None;
                let mut r#received: Option<super::super::types::DateTime> = None;
                let mut r#recipient: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#sender: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#payload: Option<Vec<CommunicationPayload>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Communication" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Communication",
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
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
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
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::InstantiatesCanonical => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#instantiates_canonical.get_or_insert(
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
                                            "instantiatesCanonical",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#instantiates_canonical.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesCanonical",
                                        ));
                                    }
                                    r#instantiates_canonical = Some(map_access.next_value()?);
                                }
                            }
                            Field::InstantiatesCanonicalPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#instantiates_canonical.get_or_insert(
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
                                            "_instantiatesCanonical",
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
                                        "instantiatesCanonical",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::InstantiatesUri => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#instantiates_uri.get_or_insert(
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
                                            "instantiatesUri",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#instantiates_uri.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesUri",
                                        ));
                                    }
                                    r#instantiates_uri = Some(map_access.next_value()?);
                                }
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#instantiates_uri.get_or_insert(
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
                                            "_instantiatesUri",
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
                                        "instantiatesUri",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
                            }
                            Field::InResponseTo => {
                                if r#in_response_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("inResponseTo"));
                                }
                                r#in_response_to = Some(map_access.next_value()?);
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
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::StatusReason => {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Priority => {
                                if _ctx.from_json {
                                    let some = r#priority.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("priority"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#priority.is_some() {
                                        return Err(serde::de::Error::duplicate_field("priority"));
                                    }
                                    r#priority = Some(map_access.next_value()?);
                                }
                            }
                            Field::PriorityPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#priority.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_priority"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "priority",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Medium => {
                                if r#medium.is_some() {
                                    return Err(serde::de::Error::duplicate_field("medium"));
                                }
                                r#medium = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Topic => {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field("topic"));
                                }
                                r#topic = Some(map_access.next_value()?);
                            }
                            Field::About => {
                                if r#about.is_some() {
                                    return Err(serde::de::Error::duplicate_field("about"));
                                }
                                r#about = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::Sent => {
                                if _ctx.from_json {
                                    let some = r#sent.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sent"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sent.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sent"));
                                    }
                                    r#sent = Some(map_access.next_value()?);
                                }
                            }
                            Field::SentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#sent.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_sent"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sent",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Received => {
                                if _ctx.from_json {
                                    let some = r#received.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("received"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#received.is_some() {
                                        return Err(serde::de::Error::duplicate_field("received"));
                                    }
                                    r#received = Some(map_access.next_value()?);
                                }
                            }
                            Field::ReceivedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#received.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_received"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "received",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "partOf",
                                            "inResponseTo",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "medium",
                                            "subject",
                                            "topic",
                                            "about",
                                            "encounter",
                                            "sent",
                                            "received",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "payload",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Recipient => {
                                if r#recipient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recipient"));
                                }
                                r#recipient = Some(map_access.next_value()?);
                            }
                            Field::Sender => {
                                if r#sender.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sender"));
                                }
                                r#sender = Some(map_access.next_value()?);
                            }
                            Field::ReasonCode => {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code = Some(map_access.next_value()?);
                            }
                            Field::ReasonReference => {
                                if r#reason_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some(map_access.next_value()?);
                            }
                            Field::Payload => {
                                if r#payload.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payload"));
                                }
                                r#payload = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "instantiatesCanonical",
                                        "instantiatesUri",
                                        "basedOn",
                                        "partOf",
                                        "inResponseTo",
                                        "status",
                                        "statusReason",
                                        "category",
                                        "priority",
                                        "medium",
                                        "subject",
                                        "topic",
                                        "about",
                                        "encounter",
                                        "sent",
                                        "received",
                                        "recipient",
                                        "sender",
                                        "reasonCode",
                                        "reasonReference",
                                        "payload",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Communication {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                        r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#part_of: r#part_of.unwrap_or(vec![]),
                        r#in_response_to: r#in_response_to.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_reason,
                        r#category: r#category.unwrap_or(vec![]),
                        r#priority,
                        r#medium: r#medium.unwrap_or(vec![]),
                        r#subject,
                        r#topic,
                        r#about: r#about.unwrap_or(vec![]),
                        r#encounter,
                        r#sent,
                        r#received,
                        r#recipient: r#recipient.unwrap_or(vec![]),
                        r#sender,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#payload: r#payload.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
