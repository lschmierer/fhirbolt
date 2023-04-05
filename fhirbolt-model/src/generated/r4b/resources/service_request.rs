// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "An amount of service being requested which can be a quantity ( for example $1,500 home modification), a ratio ( for example, 20 half day visits per month), or a range (2.0 to 1.8 Gy per fraction)."]
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRequestQuantity {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for ServiceRequestQuantity {
    fn default() -> ServiceRequestQuantity {
        ServiceRequestQuantity::Invalid
    }
}
#[doc = "The date/time at which the requested service should occur."]
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ServiceRequestOccurrence {
    fn default() -> ServiceRequestOccurrence {
        ServiceRequestOccurrence::Invalid
    }
}
#[doc = "If a CodeableConcept is present, it indicates the pre-condition for performing the service.  For example \"pain\", \"on flare-up\", etc."]
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRequestAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for ServiceRequestAsNeeded {
    fn default() -> ServiceRequestAsNeeded {
        ServiceRequestAsNeeded::Invalid
    }
}
#[doc = "A record of a request for service such as diagnostic investigations, treatments, or operations to be performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServiceRequest {
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
    #[doc = "Identifiers assigned to this order instance by the orderer and/or the receiver and/or order fulfiller."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this ServiceRequest."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this ServiceRequest."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Plan/proposal/order fulfilled by this request."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The request takes the place of the referenced completed or terminated request(s)."]
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    #[doc = "A shared identifier common to all service requests that were authorized more or less simultaneously by a single author, representing the composite or group identifier."]
    pub r#requisition: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of the order."]
    pub r#status: super::super::types::Code,
    #[doc = "Whether the request is a proposal, plan, an original order or a reflex order."]
    pub r#intent: super::super::types::Code,
    #[doc = "A code that classifies the service for searching, sorting and display purposes (e.g. \"Surgical Procedure\")."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates how quickly the ServiceRequest should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "Set this to true if the record is saying that the service/procedure should NOT be performed."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "A code that identifies a particular service (i.e., procedure, diagnostic investigation, or panel of investigations) that have been requested."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional details and instructions about the how the services are to be delivered.   For example, and order for a urinary catheter may have an order detail for an external or indwelling catheter, or an order for a bandage may require additional instructions specifying how the bandage should be applied."]
    pub r#order_detail: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An amount of service being requested which can be a quantity ( for example $1,500 home modification), a ratio ( for example, 20 half day visits per month), or a range (2.0 to 1.8 Gy per fraction)."]
    pub r#quantity: Option<ServiceRequestQuantity>,
    #[doc = "On whom or what the service is to be performed. This is usually a human patient, but can also be requested on animals, groups of humans or animals, devices such as dialysis machines, or even locations (typically for environmental scans)."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "An encounter that provides additional information about the healthcare context in which this request is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date/time at which the requested service should occur."]
    pub r#occurrence: Option<ServiceRequestOccurrence>,
    #[doc = "If a CodeableConcept is present, it indicates the pre-condition for performing the service.  For example \"pain\", \"on flare-up\", etc."]
    pub r#as_needed: Option<ServiceRequestAsNeeded>,
    #[doc = "When the request transitioned to being actionable."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The individual who initiated the request and has responsibility for its activation."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "Desired type of performer for doing the requested service."]
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The desired performer for doing the requested service.  For example, the surgeon, dermatopathologist, endoscopist, etc."]
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    #[doc = "The preferred location(s) where the procedure should actually happen in coded or free text form. E.g. at home or nursing day care center."]
    pub r#location_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to the the preferred location(s) where the procedure should actually happen. E.g. at home or nursing day care center."]
    pub r#location_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "An explanation or justification for why this service is being requested in coded or textual form.   This is often for billing purposes.  May relate to the resources referred to in `supportingInfo`."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource that provides a justification for why this service is being requested.   May relate to the resources referred to in `supportingInfo`."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be needed for delivering the requested service."]
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    #[doc = "Additional clinical information about the patient or specimen that may influence the services or their interpretations.     This information includes diagnosis, clinical findings and other observations.  In laboratory ordering these are typically referred to as \"ask at order entry questions (AOEs)\".  This includes observations explicitly requested by the producer (filler) to provide context or supporting information needed to complete the order. For example,  reporting the amount of inspired oxygen for blood gas measurements."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "One or more specimens that the laboratory procedure will use."]
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    #[doc = "Anatomic location where the procedure should be performed. This is the target site."]
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Any other notes and comments made about the service request. For example, internal billing notes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Instructions in terms that are understood by the patient or consumer."]
    pub r#patient_instruction: Option<super::super::types::String>,
    #[doc = "Key events in the history of the request."]
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for ServiceRequest {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for ServiceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ServiceRequest")?;
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
            if !self.r#replaces.is_empty() {
                state.serialize_entry("replaces", &self.r#replaces)?;
            }
            if let Some(some) = self.r#requisition.as_ref() {
                state.serialize_entry("requisition", some)?;
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if !self.r#order_detail.is_empty() {
                state.serialize_entry("orderDetail", &self.r#order_detail)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                match some {
                    ServiceRequestQuantity::Quantity(ref value) => {
                        state.serialize_entry("quantityQuantity", value)?;
                    }
                    ServiceRequestQuantity::Ratio(ref value) => {
                        state.serialize_entry("quantityRatio", value)?;
                    }
                    ServiceRequestQuantity::Range(ref value) => {
                        state.serialize_entry("quantityRange", value)?;
                    }
                    ServiceRequestQuantity::Invalid => {
                        return Err(serde::ser::Error::custom("quantity is invalid"))
                    }
                }
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    ServiceRequestOccurrence::DateTime(ref value) => {
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
                    ServiceRequestOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    ServiceRequestOccurrence::Timing(ref value) => {
                        state.serialize_entry("occurrenceTiming", value)?;
                    }
                    ServiceRequestOccurrence::Invalid => {
                        return Err(serde::ser::Error::custom("occurrence is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#as_needed.as_ref() {
                match some {
                    ServiceRequestAsNeeded::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("asNeededBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_asNeededBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("asNeededBoolean", value)?;
                        }
                    }
                    ServiceRequestAsNeeded::CodeableConcept(ref value) => {
                        state.serialize_entry("asNeededCodeableConcept", value)?;
                    }
                    ServiceRequestAsNeeded::Invalid => {
                        return Err(serde::ser::Error::custom("as_needed is invalid"))
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
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if !self.r#location_code.is_empty() {
                state.serialize_entry("locationCode", &self.r#location_code)?;
            }
            if !self.r#location_reference.is_empty() {
                state.serialize_entry("locationReference", &self.r#location_reference)?;
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
            if !self.r#specimen.is_empty() {
                state.serialize_entry("specimen", &self.r#specimen)?;
            }
            if !self.r#body_site.is_empty() {
                state.serialize_entry("bodySite", &self.r#body_site)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#patient_instruction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("patientInstruction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_patientInstruction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#patient_instruction.as_ref() {
                    state.serialize_entry("patientInstruction", some)?;
                }
            }
            if !self.r#relevant_history.is_empty() {
                state.serialize_entry("relevantHistory", &self.r#relevant_history)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ServiceRequest {
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
            #[serde(rename = "replaces")]
            Replaces,
            #[serde(rename = "requisition")]
            Requisition,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "intent")]
            Intent,
            #[serde(rename = "_intent")]
            IntentPrimitiveElement,
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "orderDetail")]
            OrderDetail,
            #[serde(rename = "quantityQuantity")]
            QuantityQuantity,
            #[serde(rename = "quantityRatio")]
            QuantityRatio,
            #[serde(rename = "quantityRange")]
            QuantityRange,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "occurrencePeriod")]
            OccurrencePeriod,
            #[serde(rename = "occurrenceTiming")]
            OccurrenceTiming,
            #[serde(rename = "asNeededBoolean")]
            AsNeededBoolean,
            #[serde(rename = "_asNeededBoolean")]
            AsNeededBooleanPrimitiveElement,
            #[serde(rename = "asNeededCodeableConcept")]
            AsNeededCodeableConcept,
            #[serde(rename = "authoredOn")]
            AuthoredOn,
            #[serde(rename = "_authoredOn")]
            AuthoredOnPrimitiveElement,
            #[serde(rename = "requester")]
            Requester,
            #[serde(rename = "performerType")]
            PerformerType,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "locationCode")]
            LocationCode,
            #[serde(rename = "locationReference")]
            LocationReference,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "insurance")]
            Insurance,
            #[serde(rename = "supportingInfo")]
            SupportingInfo,
            #[serde(rename = "specimen")]
            Specimen,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "patientInstruction")]
            PatientInstruction,
            #[serde(rename = "_patientInstruction")]
            PatientInstructionPrimitiveElement,
            #[serde(rename = "relevantHistory")]
            RelevantHistory,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ServiceRequest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ServiceRequest, V::Error>
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
                let mut r#replaces: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#requisition: Option<Box<super::super::types::Identifier>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#intent: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#order_detail: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#quantity: Option<ServiceRequestQuantity> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#occurrence: Option<ServiceRequestOccurrence> = None;
                let mut r#as_needed: Option<ServiceRequestAsNeeded> = None;
                let mut r#authored_on: Option<super::super::types::DateTime> = None;
                let mut r#requester: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#performer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#location_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#location_reference: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#insurance: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#supporting_info: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#specimen: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#patient_instruction: Option<super::super::types::String> = None;
                let mut r#relevant_history: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ServiceRequest" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ServiceRequest",
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
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
                            Field::Identifier => {
                                if _ctx.from_json {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#identifier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                    let vec =
                                        r#instantiates_canonical.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
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
                                    let vec = r#instantiates_uri.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
                                        ],
                                    ));
                                }
                            }
                            Field::BasedOn => {
                                if _ctx.from_json {
                                    if r#based_on.is_some() {
                                        return Err(serde::de::Error::duplicate_field("basedOn"));
                                    }
                                    r#based_on = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#based_on.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Replaces => {
                                if _ctx.from_json {
                                    if r#replaces.is_some() {
                                        return Err(serde::de::Error::duplicate_field("replaces"));
                                    }
                                    r#replaces = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#replaces.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Requisition => {
                                if r#requisition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requisition"));
                                }
                                r#requisition = Some(map_access.next_value()?);
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
                                        ],
                                    ));
                                }
                            }
                            Field::Intent => {
                                if _ctx.from_json {
                                    let some = r#intent.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("intent"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#intent.is_some() {
                                        return Err(serde::de::Error::duplicate_field("intent"));
                                    }
                                    r#intent = Some(map_access.next_value()?);
                                }
                            }
                            Field::IntentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#intent.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_intent"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "intent",
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
                                        ],
                                    ));
                                }
                            }
                            Field::Category => {
                                if _ctx.from_json {
                                    if r#category.is_some() {
                                        return Err(serde::de::Error::duplicate_field("category"));
                                    }
                                    r#category = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#category.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
                                        ],
                                    ));
                                }
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::OrderDetail => {
                                if _ctx.from_json {
                                    if r#order_detail.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "orderDetail",
                                        ));
                                    }
                                    r#order_detail = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#order_detail.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::QuantityQuantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "quantityQuantity",
                                    ));
                                }
                                r#quantity = Some(ServiceRequestQuantity::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::QuantityRatio => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantityRatio"));
                                }
                                r#quantity =
                                    Some(ServiceRequestQuantity::Ratio(map_access.next_value()?));
                            }
                            Field::QuantityRange => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantityRange"));
                                }
                                r#quantity =
                                    Some(ServiceRequestQuantity::Range(map_access.next_value()?));
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::OccurrenceDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        ServiceRequestOccurrence::DateTime(Default::default()),
                                    );
                                    if let ServiceRequestOccurrence::DateTime(variant) = r#enum {
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
                                    r#occurrence = Some(ServiceRequestOccurrence::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::OccurrenceDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        ServiceRequestOccurrence::DateTime(Default::default()),
                                    );
                                    if let ServiceRequestOccurrence::DateTime(variant) = r#enum {
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
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
                                r#occurrence = Some(ServiceRequestOccurrence::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::OccurrenceTiming => {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceTiming",
                                    ));
                                }
                                r#occurrence = Some(ServiceRequestOccurrence::Timing(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AsNeededBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#as_needed.get_or_insert(
                                        ServiceRequestAsNeeded::Boolean(Default::default()),
                                    );
                                    if let ServiceRequestAsNeeded::Boolean(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "asNeededBoolean",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "asNeeded[x]",
                                        ));
                                    }
                                } else {
                                    if r#as_needed.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "asNeededBoolean",
                                        ));
                                    }
                                    r#as_needed = Some(ServiceRequestAsNeeded::Boolean(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::AsNeededBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#as_needed.get_or_insert(
                                        ServiceRequestAsNeeded::Boolean(Default::default()),
                                    );
                                    if let ServiceRequestAsNeeded::Boolean(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_asNeededBoolean",
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
                                            "_asNeeded[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "asNeededBoolean",
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
                                        ],
                                    ));
                                }
                            }
                            Field::AsNeededCodeableConcept => {
                                if r#as_needed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "asNeededCodeableConcept",
                                    ));
                                }
                                r#as_needed = Some(ServiceRequestAsNeeded::CodeableConcept(
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
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
                            Field::PerformerType => {
                                if r#performer_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performerType"));
                                }
                                r#performer_type = Some(map_access.next_value()?);
                            }
                            Field::Performer => {
                                if _ctx.from_json {
                                    if r#performer.is_some() {
                                        return Err(serde::de::Error::duplicate_field("performer"));
                                    }
                                    r#performer = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#performer.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::LocationCode => {
                                if _ctx.from_json {
                                    if r#location_code.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "locationCode",
                                        ));
                                    }
                                    r#location_code = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#location_code.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::LocationReference => {
                                if _ctx.from_json {
                                    if r#location_reference.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "locationReference",
                                        ));
                                    }
                                    r#location_reference = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#location_reference.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ReasonCode => {
                                if _ctx.from_json {
                                    if r#reason_code.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reasonCode",
                                        ));
                                    }
                                    r#reason_code = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reason_code.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ReasonReference => {
                                if _ctx.from_json {
                                    if r#reason_reference.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reasonReference",
                                        ));
                                    }
                                    r#reason_reference = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reason_reference.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Insurance => {
                                if _ctx.from_json {
                                    if r#insurance.is_some() {
                                        return Err(serde::de::Error::duplicate_field("insurance"));
                                    }
                                    r#insurance = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#insurance.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SupportingInfo => {
                                if _ctx.from_json {
                                    if r#supporting_info.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "supportingInfo",
                                        ));
                                    }
                                    r#supporting_info = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#supporting_info.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Specimen => {
                                if _ctx.from_json {
                                    if r#specimen.is_some() {
                                        return Err(serde::de::Error::duplicate_field("specimen"));
                                    }
                                    r#specimen = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#specimen.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::BodySite => {
                                if _ctx.from_json {
                                    if r#body_site.is_some() {
                                        return Err(serde::de::Error::duplicate_field("bodySite"));
                                    }
                                    r#body_site = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#body_site.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Note => {
                                if _ctx.from_json {
                                    if r#note.is_some() {
                                        return Err(serde::de::Error::duplicate_field("note"));
                                    }
                                    r#note = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#note.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PatientInstruction => {
                                if _ctx.from_json {
                                    let some =
                                        r#patient_instruction.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patientInstruction",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#patient_instruction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patientInstruction",
                                        ));
                                    }
                                    r#patient_instruction = Some(map_access.next_value()?);
                                }
                            }
                            Field::PatientInstructionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#patient_instruction.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patientInstruction",
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
                                        "patientInstruction",
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
                                            "replaces",
                                            "requisition",
                                            "status",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "code",
                                            "orderDetail",
                                            "quantityQuantity",
                                            "quantityRatio",
                                            "quantityRange",
                                            "subject",
                                            "encounter",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "asNeededBoolean",
                                            "asNeededCodeableConcept",
                                            "authoredOn",
                                            "requester",
                                            "performerType",
                                            "performer",
                                            "locationCode",
                                            "locationReference",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "supportingInfo",
                                            "specimen",
                                            "bodySite",
                                            "note",
                                            "patientInstruction",
                                            "relevantHistory",
                                        ],
                                    ));
                                }
                            }
                            Field::RelevantHistory => {
                                if _ctx.from_json {
                                    if r#relevant_history.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "relevantHistory",
                                        ));
                                    }
                                    r#relevant_history = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#relevant_history.get_or_insert(Default::default());
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
                                        "identifier",
                                        "instantiatesCanonical",
                                        "instantiatesUri",
                                        "basedOn",
                                        "replaces",
                                        "requisition",
                                        "status",
                                        "intent",
                                        "category",
                                        "priority",
                                        "doNotPerform",
                                        "code",
                                        "orderDetail",
                                        "quantityQuantity",
                                        "quantityRatio",
                                        "quantityRange",
                                        "subject",
                                        "encounter",
                                        "occurrenceDateTime",
                                        "occurrencePeriod",
                                        "occurrenceTiming",
                                        "asNeededBoolean",
                                        "asNeededCodeableConcept",
                                        "authoredOn",
                                        "requester",
                                        "performerType",
                                        "performer",
                                        "locationCode",
                                        "locationReference",
                                        "reasonCode",
                                        "reasonReference",
                                        "insurance",
                                        "supportingInfo",
                                        "specimen",
                                        "bodySite",
                                        "note",
                                        "patientInstruction",
                                        "relevantHistory",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ServiceRequest {
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
                        r#replaces: r#replaces.unwrap_or(vec![]),
                        r#requisition,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#intent: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#intent.unwrap_or(Default::default())
                        } else {
                            r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#priority,
                        r#do_not_perform,
                        r#code,
                        r#order_detail: r#order_detail.unwrap_or(vec![]),
                        r#quantity,
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#occurrence,
                        r#as_needed,
                        r#authored_on,
                        r#requester,
                        r#performer_type,
                        r#performer: r#performer.unwrap_or(vec![]),
                        r#location_code: r#location_code.unwrap_or(vec![]),
                        r#location_reference: r#location_reference.unwrap_or(vec![]),
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#insurance: r#insurance.unwrap_or(vec![]),
                        r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                        r#specimen: r#specimen.unwrap_or(vec![]),
                        r#body_site: r#body_site.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#patient_instruction,
                        r#relevant_history: r#relevant_history.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
