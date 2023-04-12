// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The period, timing or frequency upon which the described activity is to occur."]
#[derive(Debug, Clone, PartialEq)]
pub enum CarePlanActivityDetailScheduled {
    Timing(Box<super::super::types::Timing>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for CarePlanActivityDetailScheduled {
    fn default() -> CarePlanActivityDetailScheduled {
        CarePlanActivityDetailScheduled::Invalid
    }
}
#[doc = "Identifies the food, drug or other product to be consumed or supplied in the activity."]
#[derive(Debug, Clone, PartialEq)]
pub enum CarePlanActivityDetailProduct {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for CarePlanActivityDetailProduct {
    fn default() -> CarePlanActivityDetailProduct {
        CarePlanActivityDetailProduct::Invalid
    }
}
#[doc = "A simple summary of a planned activity suitable for a general care plan system (e.g. form driven) that doesn't know about specific resources such as procedure etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CarePlanActivityDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A description of the kind of resource the in-line definition of a care plan activity is representing.  The CarePlan.activity.detail is an in-line definition when a resource is not referenced using CarePlan.activity.reference.  For example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest."]
    pub r#kind: Option<super::super::types::Code>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan activity."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan activity."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Detailed description of the type of planned activity; e.g. what lab test, what procedure, what kind of encounter."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Provides the rationale that drove the inclusion of this particular activity as part of the plan or the reason why the activity was prohibited."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource, such as the health condition(s), whose existence justifies this request and drove the inclusion of this particular activity as part of the plan."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Internal reference that identifies the goals that this activity is intended to contribute towards meeting."]
    pub r#goal: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies what progress is being made for the specific activity."]
    pub r#status: super::super::types::Code,
    #[doc = "Provides reason why the activity isn't yet started, is on hold, was cancelled, etc."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If true, indicates that the described activity is one that must NOT be engaged in when following the plan.  If false, or missing, indicates that the described activity is one that should be engaged in when following the plan."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "The period, timing or frequency upon which the described activity is to occur."]
    pub r#scheduled: Option<CarePlanActivityDetailScheduled>,
    #[doc = "Identifies the facility where the activity will occur; e.g. home, hospital, specific clinic, etc."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies who's expected to be involved in the activity."]
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies the food, drug or other product to be consumed or supplied in the activity."]
    pub r#product: Option<CarePlanActivityDetailProduct>,
    #[doc = "Identifies the quantity expected to be consumed in a given day."]
    pub r#daily_amount: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the quantity expected to be supplied, administered or consumed by the subject."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "This provides a textual description of constraints on the intended activity occurrence, including relation to other activities.  It may also include objectives, pre-conditions and end-conditions.  Finally, it may convey specifics about the activity such as body site, method, route, etc."]
    pub r#description: Option<super::super::types::String>,
}
impl serde::ser::Serialize for CarePlanActivityDetail {
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
                if let Some(some) = self.r#kind.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("kind", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_kind", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#kind.as_ref() {
                    state.serialize_entry("kind", some)?;
                }
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#goal.is_empty() {
                state.serialize_entry("goal", &self.r#goal)?;
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
            if let Some(some) = self.r#scheduled.as_ref() {
                match some {
                    CarePlanActivityDetailScheduled::Timing(ref value) => {
                        state.serialize_entry("scheduledTiming", value)?;
                    }
                    CarePlanActivityDetailScheduled::Period(ref value) => {
                        state.serialize_entry("scheduledPeriod", value)?;
                    }
                    CarePlanActivityDetailScheduled::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("scheduledString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_scheduledString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("scheduledString", value)?;
                        }
                    }
                    CarePlanActivityDetailScheduled::Invalid => {
                        return Err(serde::ser::Error::custom("scheduled is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if let Some(some) = self.r#product.as_ref() {
                match some {
                    CarePlanActivityDetailProduct::CodeableConcept(ref value) => {
                        state.serialize_entry("productCodeableConcept", value)?;
                    }
                    CarePlanActivityDetailProduct::Reference(ref value) => {
                        state.serialize_entry("productReference", value)?;
                    }
                    CarePlanActivityDetailProduct::Invalid => {
                        return Err(serde::ser::Error::custom("product is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#daily_amount.as_ref() {
                state.serialize_entry("dailyAmount", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
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
#[doc = "Identifies a planned action to occur as part of the plan.  For example, a medication to be used, lab tests to perform, self-monitoring, education, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CarePlanActivity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the outcome at the point when the status of the activity is assessed.  For example, the outcome of an education activity could be patient understands (or not)."]
    pub r#outcome_codeable_concept: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Details of the outcome or action resulting from the activity.  The reference to an \"event\" resource, such as Procedure or Encounter or Observation, is the result/outcome of the activity itself.  The activity can be conveyed using CarePlan.activity.detail OR using the CarePlan.activity.reference (a reference to a “request” resource)."]
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Notes about the adherence/status/progress of the activity."]
    pub r#progress: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The details of the proposed activity represented in a specific resource."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
    #[doc = "A simple summary of a planned activity suitable for a general care plan system (e.g. form driven) that doesn't know about specific resources such as procedure etc."]
    pub r#detail: Option<CarePlanActivityDetail>,
}
impl serde::ser::Serialize for CarePlanActivity {
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
            if !self.r#outcome_codeable_concept.is_empty() {
                state
                    .serialize_entry("outcomeCodeableConcept", &self.r#outcome_codeable_concept)?;
            }
            if !self.r#outcome_reference.is_empty() {
                state.serialize_entry("outcomeReference", &self.r#outcome_reference)?;
            }
            if !self.r#progress.is_empty() {
                state.serialize_entry("progress", &self.r#progress)?;
            }
            if let Some(some) = self.r#reference.as_ref() {
                state.serialize_entry("reference", some)?;
            }
            if let Some(some) = self.r#detail.as_ref() {
                state.serialize_entry("detail", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CarePlan {
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
    #[doc = "Business identifiers assigned to this care plan by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A care plan that is fulfilled in whole or in part by this care plan."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "Completed or terminated care plan whose function is taken by this new care plan."]
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger care plan of which this particular care plan is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates whether the plan is currently being acted upon, represents future intentions or is now a historical record."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the care plan and where the care plan fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "Identifies what \"kind\" of plan this is to support differentiation between multiple co-existing plans; e.g. \"Home health\", \"psychiatric\", \"asthma\", \"disease management\", \"wellness plan\", etc."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Human-friendly name for the care plan."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A description of the scope and nature of the plan."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Identifies the patient or group whose intended care is described by the plan."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this CarePlan was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the plan did (or is intended to) come into effect and end."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Represents when this particular CarePlan record was created in the system, which is often a system-generated date."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "When populated, the author is responsible for the care plan.  The care plan is attributed to the author."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the individual(s) or organization who provided the contents of the care plan."]
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies all people and organizations who are expected to be involved in the care envisioned by this plan."]
    pub r#care_team: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies the conditions/problems/concerns/diagnoses/etc. whose management and/or mitigation are handled by this plan."]
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies portions of the patient's record that specifically influenced the formation of the plan.  These might include comorbidities, recent procedures, limitations, recent assessments, etc."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes the intended objective(s) of carrying out the care plan."]
    pub r#goal: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies a planned action to occur as part of the plan.  For example, a medication to be used, lab tests to perform, self-monitoring, education, etc."]
    pub r#activity: Vec<CarePlanActivity>,
    #[doc = "General notes about the care plan not covered elsewhere."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for CarePlan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "CarePlan")?;
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
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
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
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#created.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("created", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_created", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#created.as_ref() {
                    state.serialize_entry("created", some)?;
                }
            }
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            if !self.r#contributor.is_empty() {
                state.serialize_entry("contributor", &self.r#contributor)?;
            }
            if !self.r#care_team.is_empty() {
                state.serialize_entry("careTeam", &self.r#care_team)?;
            }
            if !self.r#addresses.is_empty() {
                state.serialize_entry("addresses", &self.r#addresses)?;
            }
            if !self.r#supporting_info.is_empty() {
                state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
            }
            if !self.r#goal.is_empty() {
                state.serialize_entry("goal", &self.r#goal)?;
            }
            if !self.r#activity.is_empty() {
                state.serialize_entry("activity", &self.r#activity)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
