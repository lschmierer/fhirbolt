// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The details of the device to be used."]
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceRequestCode {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for DeviceRequestCode {
    fn default() -> DeviceRequestCode {
        DeviceRequestCode::Invalid
    }
}
#[doc = "The value of the device detail."]
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceRequestParameterValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Boolean(Box<super::super::types::Boolean>),
    Invalid,
}
impl Default for DeviceRequestParameterValue {
    fn default() -> DeviceRequestParameterValue {
        DeviceRequestParameterValue::Invalid
    }
}
#[doc = "The timing schedule for the use of the device. The Schedule data type allows many different expressions, for example. \"Every 8 hours\"; \"Three times a day\"; \"1/2 an hour before breakfast for 10 days from 23-Dec 2011:\"; \"15 Oct 2013, 17 Oct 2013 and 1 Nov 2013\"."]
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for DeviceRequestOccurrence {
    fn default() -> DeviceRequestOccurrence {
        DeviceRequestOccurrence::Invalid
    }
}
#[doc = "Specific parameters for the ordered item.  For example, the prism value for lenses."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceRequestParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code or string that identifies the device detail being asserted."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The value of the device detail."]
    pub r#value: Option<DeviceRequestParameterValue>,
}
impl serde::ser::Serialize for DeviceRequestParameter {
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    DeviceRequestParameterValue::CodeableConcept(ref value) => {
                        state.serialize_entry("valueCodeableConcept", value)?;
                    }
                    DeviceRequestParameterValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    DeviceRequestParameterValue::Range(ref value) => {
                        state.serialize_entry("valueRange", value)?;
                    }
                    DeviceRequestParameterValue::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueBoolean", value)?;
                        }
                    }
                    DeviceRequestParameterValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Represents a request for a patient to employ a medical device. The device may be an implantable device, or an external assistive device, such as a walker."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceRequest {
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
    #[doc = "Identifiers assigned to this order by the orderer or by the receiver."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this DeviceRequest."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this DeviceRequest."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Plan/proposal/order fulfilled by this request."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The request takes the place of the referenced completed or terminated request(s)."]
    pub r#prior_request: Vec<Box<super::super::types::Reference>>,
    #[doc = "Composite request this is part of."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of the request."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Whether the request is a proposal, plan, an original order or a reflex order."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates how quickly the {{title}} should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "The details of the device to be used."]
    pub r#code: DeviceRequestCode,
    #[doc = "Specific parameters for the ordered item.  For example, the prism value for lenses."]
    pub r#parameter: Vec<DeviceRequestParameter>,
    #[doc = "The patient who will use the device."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "An encounter that provides additional context in which this request is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The timing schedule for the use of the device. The Schedule data type allows many different expressions, for example. \"Every 8 hours\"; \"Three times a day\"; \"1/2 an hour before breakfast for 10 days from 23-Dec 2011:\"; \"15 Oct 2013, 17 Oct 2013 and 1 Nov 2013\"."]
    pub r#occurrence: Option<DeviceRequestOccurrence>,
    #[doc = "When the request transitioned to being actionable."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The individual who initiated the request and has responsibility for its activation."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "Desired type of performer for doing the diagnostic testing."]
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The desired performer for doing the diagnostic testing."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "Reason or justification for the use of this device."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reason or justification for the use of this device."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be required for delivering the requested service."]
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    #[doc = "Additional clinical information about the patient that may influence the request fulfilment.  For example, this may include where on the subject's body the device will be used (i.e. the target site)."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details about this request that were not represented at all or sufficiently in one of the attributes provided in a class. These may include for example a comment, an instruction, or a note associated with the statement."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Key events in the history of the request."]
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for DeviceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "DeviceRequest")?;
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
            if !self.r#prior_request.is_empty() {
                state.serialize_entry("priorRequest", &self.r#prior_request)?;
            }
            if let Some(some) = self.r#group_identifier.as_ref() {
                state.serialize_entry("groupIdentifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#intent.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("intent", &some)?;
                }
                if self.r#intent.id.is_some() || !self.r#intent.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#intent.id.as_ref(),
                        extension: &self.r#intent.extension,
                    };
                    state.serialize_entry("_intent", &primitive_element)?;
                }
            } else {
                state.serialize_entry("intent", &self.r#intent)?;
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
            match self.r#code {
                DeviceRequestCode::Reference(ref value) => {
                    state.serialize_entry("codeReference", value)?;
                }
                DeviceRequestCode::CodeableConcept(ref value) => {
                    state.serialize_entry("codeCodeableConcept", value)?;
                }
                DeviceRequestCode::Invalid => {
                    return Err(serde::ser::Error::custom("code is a required field"))
                }
            }
            if !self.r#parameter.is_empty() {
                state.serialize_entry("parameter", &self.r#parameter)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    DeviceRequestOccurrence::DateTime(ref value) => {
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
                    DeviceRequestOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    DeviceRequestOccurrence::Timing(ref value) => {
                        state.serialize_entry("occurrenceTiming", value)?;
                    }
                    DeviceRequestOccurrence::Invalid => {
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
            if let Some(some) = self.r#performer_type.as_ref() {
                state.serialize_entry("performerType", some)?;
            }
            if let Some(some) = self.r#performer.as_ref() {
                state.serialize_entry("performer", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#insurance.is_empty() {
                state.serialize_entry("insurance", &self.r#insurance)?;
            }
            if !self.r#supporting_info.is_empty() {
                state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#relevant_history.is_empty() {
                state.serialize_entry("relevantHistory", &self.r#relevant_history)?;
            }
            state.end()
        })
    }
}
