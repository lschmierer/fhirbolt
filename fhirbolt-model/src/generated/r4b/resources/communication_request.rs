// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "The communicated content (or for multi-part communications, one portion of the communication)."]
#[derive(Debug, Clone)]
pub enum CommunicationRequestPayloadContent {
    String(Box<super::super::types::String>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for CommunicationRequestPayloadContent {
    fn default() -> CommunicationRequestPayloadContent {
        CommunicationRequestPayloadContent::Invalid
    }
}
#[doc = "The time when this communication is to occur."]
#[derive(Debug, Clone)]
pub enum CommunicationRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for CommunicationRequestOccurrence {
    fn default() -> CommunicationRequestOccurrence {
        CommunicationRequestOccurrence::Invalid
    }
}
#[doc = "Text, attachment(s), or resource(s) to be communicated to the recipient."]
#[derive(Default, Debug, Clone)]
pub struct CommunicationRequestPayload {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The communicated content (or for multi-part communications, one portion of the communication)."]
    pub r#content: CommunicationRequestPayloadContent,
}
impl serde::ser::Serialize for CommunicationRequestPayload {
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
                CommunicationRequestPayloadContent::String(ref value) => {
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
                CommunicationRequestPayloadContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                CommunicationRequestPayloadContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                CommunicationRequestPayloadContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CommunicationRequestPayload {
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
            type Value = CommunicationRequestPayload;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CommunicationRequestPayload")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CommunicationRequestPayload, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<CommunicationRequestPayloadContent> = None;
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
                                        CommunicationRequestPayloadContent::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let CommunicationRequestPayloadContent::String(variant) =
                                        r#enum
                                    {
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
                                    r#content = Some(CommunicationRequestPayloadContent::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ContentStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#content.get_or_insert(
                                        CommunicationRequestPayloadContent::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let CommunicationRequestPayloadContent::String(variant) =
                                        r#enum
                                    {
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
                                r#content = Some(CommunicationRequestPayloadContent::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ContentReference => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                r#content = Some(CommunicationRequestPayloadContent::Reference(
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
                    Ok(CommunicationRequestPayload {
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
#[doc = "A request to convey information; e.g. the CDS system proposes that an alert be sent to a responsible provider, the CDS system proposes that the public health agency be notified about a reportable condition."]
#[derive(Default, Debug, Clone)]
pub struct CommunicationRequest {
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
    #[doc = "Business identifiers assigned to this communication request by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A plan or proposal that is fulfilled in whole or in part by this request."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "Completed or terminated request(s) whose function is taken by this new request."]
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    #[doc = "A shared identifier common to all requests that were authorized more or less simultaneously by a single author, representing the identifier of the requisition, prescription or similar form."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of the proposal or order."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the CommunicationRequest."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of message to be sent such as alert, notification, reminder, instruction, etc."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Characterizes how quickly the proposed act must be initiated. Includes concepts such as stat, urgent, routine."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "If true indicates that the CommunicationRequest is asking for the specified action to *not* occur."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "A channel that was used for this communication (e.g. email, fax)."]
    pub r#medium: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient or group that is the focus of this communication request."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Other resources that pertain to this communication request and to which this communication request should be associated."]
    pub r#about: Vec<Box<super::super::types::Reference>>,
    #[doc = "The Encounter during which this CommunicationRequest was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Text, attachment(s), or resource(s) to be communicated to the recipient."]
    pub r#payload: Vec<CommunicationRequestPayload>,
    #[doc = "The time when this communication is to occur."]
    pub r#occurrence: Option<CommunicationRequestOccurrence>,
    #[doc = "For draft requests, indicates the date of initial creation.  For requests with other statuses, indicates the date of activation."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The device, individual, or organization who initiated the request and has responsibility for its activation."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "The entity (e.g. person, organization, clinical information system, device, group, or care team) which is the intended target of the communication."]
    pub r#recipient: Vec<Box<super::super::types::Reference>>,
    #[doc = "The entity (e.g. person, organization, clinical information system, or device) which is to be the source of the communication."]
    pub r#sender: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes why the request is being made in coded or textual form."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource whose existence justifies this request."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Comments made about the request by the requester, sender, recipient, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl crate::AnyResource for CommunicationRequest {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize for CommunicationRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "CommunicationRequest")?;
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
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if !self.r#replaces.is_empty() {
                state.serialize_entry("replaces", &self.r#replaces)?;
            }
            if let Some(some) = self.r#group_identifier.as_ref() {
                state.serialize_entry("groupIdentifier", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doNotPerform", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_doNotPerform", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    state.serialize_entry("doNotPerform", some)?;
                }
            }
            if !self.r#medium.is_empty() {
                state.serialize_entry("medium", &self.r#medium)?;
            }
            if let Some(some) = self.r#subject.as_ref() {
                state.serialize_entry("subject", some)?;
            }
            if !self.r#about.is_empty() {
                state.serialize_entry("about", &self.r#about)?;
            }
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if !self.r#payload.is_empty() {
                state.serialize_entry("payload", &self.r#payload)?;
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    CommunicationRequestOccurrence::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("occurrenceDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("occurrenceDateTime", value)?;
                        }
                    }
                    CommunicationRequestOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    CommunicationRequestOccurrence::Invalid => {
                        return Err(serde::ser::Error::custom("occurrence is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#authored_on.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("authoredOn", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_authoredOn", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#authored_on.as_ref() {
                    state.serialize_entry("authoredOn", some)?;
                }
            }
            if let Some(some) = self.r#requester.as_ref() {
                state.serialize_entry("requester", some)?;
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
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CommunicationRequest {
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
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "replaces")]
            Replaces,
            #[serde(rename = "groupIdentifier")]
            GroupIdentifier,
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
            #[serde(rename = "doNotPerform")]
            DoNotPerform,
            #[serde(rename = "_doNotPerform")]
            DoNotPerformPrimitiveElement,
            #[serde(rename = "medium")]
            Medium,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "about")]
            About,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "payload")]
            Payload,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "occurrencePeriod")]
            OccurrencePeriod,
            #[serde(rename = "authoredOn")]
            AuthoredOn,
            #[serde(rename = "_authoredOn")]
            AuthoredOnPrimitiveElement,
            #[serde(rename = "requester")]
            Requester,
            #[serde(rename = "recipient")]
            Recipient,
            #[serde(rename = "sender")]
            Sender,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "note")]
            Note,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CommunicationRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CommunicationRequest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CommunicationRequest, V::Error>
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
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#replaces: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#group_identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#medium: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#about: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#payload: Option<Vec<CommunicationRequestPayload>> = None;
                let mut r#occurrence: Option<CommunicationRequestOccurrence> = None;
                let mut r#authored_on: Option<super::super::types::DateTime> = None;
                let mut r#requester: Option<Box<super::super::types::Reference>> = None;
                let mut r#recipient: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#sender: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "CommunicationRequest" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"CommunicationRequest",
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
                                            "basedOn",
                                            "replaces",
                                            "groupIdentifier",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "medium",
                                            "subject",
                                            "about",
                                            "encounter",
                                            "payload",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "authoredOn",
                                            "requester",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
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
                                            "basedOn",
                                            "replaces",
                                            "groupIdentifier",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "medium",
                                            "subject",
                                            "about",
                                            "encounter",
                                            "payload",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "authoredOn",
                                            "requester",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
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
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::Replaces => {
                                if r#replaces.is_some() {
                                    return Err(serde::de::Error::duplicate_field("replaces"));
                                }
                                r#replaces = Some(map_access.next_value()?);
                            }
                            Field::GroupIdentifier => {
                                if r#group_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "groupIdentifier",
                                    ));
                                }
                                r#group_identifier = Some(map_access.next_value()?);
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
                                            "basedOn",
                                            "replaces",
                                            "groupIdentifier",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "medium",
                                            "subject",
                                            "about",
                                            "encounter",
                                            "payload",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "authoredOn",
                                            "requester",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
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
                                            "basedOn",
                                            "replaces",
                                            "groupIdentifier",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "medium",
                                            "subject",
                                            "about",
                                            "encounter",
                                            "payload",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "authoredOn",
                                            "requester",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::DoNotPerform => {
                                if _ctx.from_json {
                                    let some = r#do_not_perform.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "doNotPerform",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#do_not_perform.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "doNotPerform",
                                        ));
                                    }
                                    r#do_not_perform = Some(map_access.next_value()?);
                                }
                            }
                            Field::DoNotPerformPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#do_not_perform.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_doNotPerform",
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
                                        "doNotPerform",
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
                                            "basedOn",
                                            "replaces",
                                            "groupIdentifier",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "medium",
                                            "subject",
                                            "about",
                                            "encounter",
                                            "payload",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "authoredOn",
                                            "requester",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
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
                            Field::Payload => {
                                if r#payload.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payload"));
                                }
                                r#payload = Some(map_access.next_value()?);
                            }
                            Field::OccurrenceDateTime => {
                                if _ctx.from_json {
                                    let r#enum =
                                        r#occurrence.get_or_insert(
                                            CommunicationRequestOccurrence::DateTime(
                                                Default::default(),
                                            ),
                                        );
                                    if let CommunicationRequestOccurrence::DateTime(variant) =
                                        r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "occurrenceDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrence[x]",
                                        ));
                                    }
                                } else {
                                    if r#occurrence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrenceDateTime",
                                        ));
                                    }
                                    r#occurrence = Some(CommunicationRequestOccurrence::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::OccurrenceDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum =
                                        r#occurrence.get_or_insert(
                                            CommunicationRequestOccurrence::DateTime(
                                                Default::default(),
                                            ),
                                        );
                                    if let CommunicationRequestOccurrence::DateTime(variant) =
                                        r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_occurrenceDateTime",
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
                                            "_occurrence[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "occurrenceDateTime",
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
                                            "basedOn",
                                            "replaces",
                                            "groupIdentifier",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "medium",
                                            "subject",
                                            "about",
                                            "encounter",
                                            "payload",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "authoredOn",
                                            "requester",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::OccurrencePeriod => {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrencePeriod",
                                    ));
                                }
                                r#occurrence = Some(CommunicationRequestOccurrence::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AuthoredOn => {
                                if _ctx.from_json {
                                    let some = r#authored_on.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "authoredOn",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#authored_on.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "authoredOn",
                                        ));
                                    }
                                    r#authored_on = Some(map_access.next_value()?);
                                }
                            }
                            Field::AuthoredOnPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#authored_on.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_authoredOn",
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
                                        "authoredOn",
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
                                            "basedOn",
                                            "replaces",
                                            "groupIdentifier",
                                            "status",
                                            "statusReason",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "medium",
                                            "subject",
                                            "about",
                                            "encounter",
                                            "payload",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "authoredOn",
                                            "requester",
                                            "recipient",
                                            "sender",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Requester => {
                                if r#requester.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requester"));
                                }
                                r#requester = Some(map_access.next_value()?);
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
                                        "basedOn",
                                        "replaces",
                                        "groupIdentifier",
                                        "status",
                                        "statusReason",
                                        "category",
                                        "priority",
                                        "doNotPerform",
                                        "medium",
                                        "subject",
                                        "about",
                                        "encounter",
                                        "payload",
                                        "occurrenceDateTime",
                                        "occurrencePeriod",
                                        "authoredOn",
                                        "requester",
                                        "recipient",
                                        "sender",
                                        "reasonCode",
                                        "reasonReference",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CommunicationRequest {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#replaces: r#replaces.unwrap_or(vec![]),
                        r#group_identifier,
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
                        r#do_not_perform,
                        r#medium: r#medium.unwrap_or(vec![]),
                        r#subject,
                        r#about: r#about.unwrap_or(vec![]),
                        r#encounter,
                        r#payload: r#payload.unwrap_or(vec![]),
                        r#occurrence,
                        r#authored_on,
                        r#requester,
                        r#recipient: r#recipient.unwrap_or(vec![]),
                        r#sender,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
