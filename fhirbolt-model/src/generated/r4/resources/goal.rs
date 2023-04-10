// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "The date or event after which the goal should begin being pursued."]
#[derive(Debug, Clone, PartialEq)]
pub enum GoalStart {
    Date(Box<super::super::types::Date>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for GoalStart {
    fn default() -> GoalStart {
        GoalStart::Invalid
    }
}
#[doc = "The target value of the focus to be achieved to signify the fulfillment of the goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any focus value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any focus value at or above the low value."]
#[derive(Debug, Clone, PartialEq)]
pub enum GoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Ratio(Box<super::super::types::Ratio>),
    Invalid,
}
impl Default for GoalTargetDetail {
    fn default() -> GoalTargetDetail {
        GoalTargetDetail::Invalid
    }
}
#[doc = "Indicates either the date or the duration after start by which the goal should be met."]
#[derive(Debug, Clone, PartialEq)]
pub enum GoalTargetDue {
    Date(Box<super::super::types::Date>),
    Duration(Box<super::super::types::Duration>),
    Invalid,
}
impl Default for GoalTargetDue {
    fn default() -> GoalTargetDue {
        GoalTargetDue::Invalid
    }
}
#[doc = "Indicates what should be done by when."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GoalTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The parameter whose value is being tracked, e.g. body weight, blood pressure, or hemoglobin A1c level."]
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target value of the focus to be achieved to signify the fulfillment of the goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any focus value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any focus value at or above the low value."]
    pub r#detail: Option<GoalTargetDetail>,
    #[doc = "Indicates either the date or the duration after start by which the goal should be met."]
    pub r#due: Option<GoalTargetDue>,
}
impl serde::ser::Serialize for GoalTarget {
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
            if let Some(some) = self.r#measure.as_ref() {
                state.serialize_entry("measure", some)?;
            }
            if let Some(some) = self.r#detail.as_ref() {
                match some {
                    GoalTargetDetail::Quantity(ref value) => {
                        state.serialize_entry("detailQuantity", value)?;
                    }
                    GoalTargetDetail::Range(ref value) => {
                        state.serialize_entry("detailRange", value)?;
                    }
                    GoalTargetDetail::CodeableConcept(ref value) => {
                        state.serialize_entry("detailCodeableConcept", value)?;
                    }
                    GoalTargetDetail::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("detailString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_detailString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("detailString", value)?;
                        }
                    }
                    GoalTargetDetail::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("detailBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_detailBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("detailBoolean", value)?;
                        }
                    }
                    GoalTargetDetail::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("detailInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_detailInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("detailInteger", value)?;
                        }
                    }
                    GoalTargetDetail::Ratio(ref value) => {
                        state.serialize_entry("detailRatio", value)?;
                    }
                    GoalTargetDetail::Invalid => {
                        return Err(serde::ser::Error::custom("detail is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#due.as_ref() {
                match some {
                    GoalTargetDue::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("dueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_dueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("dueDate", value)?;
                        }
                    }
                    GoalTargetDue::Duration(ref value) => {
                        state.serialize_entry("dueDuration", value)?;
                    }
                    GoalTargetDue::Invalid => {
                        return Err(serde::ser::Error::custom("due is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Goal {
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
    #[doc = "Business identifiers assigned to this goal by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The state of the goal throughout its lifecycle."]
    pub r#lifecycle_status: super::super::types::Code,
    #[doc = "Describes the progression, or lack thereof, towards the goal against the target."]
    pub r#achievement_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates a category the goal falls within."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the mutually agreed level of importance associated with reaching/sustaining the goal."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Human-readable and/or coded description of a specific desired objective of care, such as \"control blood pressure\" or \"negotiate an obstacle course\" or \"dance with child at wedding\"."]
    pub r#description: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the patient, group or organization for whom the goal is being established."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The date or event after which the goal should begin being pursued."]
    pub r#start: Option<GoalStart>,
    #[doc = "Indicates what should be done by when."]
    pub r#target: Vec<GoalTarget>,
    #[doc = "Identifies when the current status.  I.e. When initially created, when achieved, when cancelled, etc."]
    pub r#status_date: Option<super::super::types::Date>,
    #[doc = "Captures the reason for the current status."]
    pub r#status_reason: Option<super::super::types::String>,
    #[doc = "Indicates whose goal this is - patient goal, practitioner goal, etc."]
    pub r#expressed_by: Option<Box<super::super::types::Reference>>,
    #[doc = "The identified conditions and other health record elements that are intended to be addressed by the goal."]
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    #[doc = "Any comments related to the goal."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Identifies the change (or lack of change) at the point when the status of the goal is assessed."]
    pub r#outcome_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Details of what's changed (or not changed)."]
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for Goal {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for Goal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Goal")?;
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
                if let Some(some) = self.r#lifecycle_status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lifecycleStatus", &some)?;
                }
                if self.r#lifecycle_status.id.is_some()
                    || !self.r#lifecycle_status.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#lifecycle_status.id.as_ref(),
                        extension: &self.r#lifecycle_status.extension,
                    };
                    state.serialize_entry("_lifecycleStatus", &primitive_element)?;
                }
            } else {
                state.serialize_entry("lifecycleStatus", &self.r#lifecycle_status)?;
            }
            if let Some(some) = self.r#achievement_status.as_ref() {
                state.serialize_entry("achievementStatus", some)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#priority.as_ref() {
                state.serialize_entry("priority", some)?;
            }
            state.serialize_entry("description", &self.r#description)?;
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#start.as_ref() {
                match some {
                    GoalStart::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("startDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_startDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("startDate", value)?;
                        }
                    }
                    GoalStart::CodeableConcept(ref value) => {
                        state.serialize_entry("startCodeableConcept", value)?;
                    }
                    GoalStart::Invalid => {
                        return Err(serde::ser::Error::custom("start is invalid"))
                    }
                }
            }
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("statusDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_statusDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status_date.as_ref() {
                    state.serialize_entry("statusDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status_reason.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("statusReason", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_statusReason", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status_reason.as_ref() {
                    state.serialize_entry("statusReason", some)?;
                }
            }
            if let Some(some) = self.r#expressed_by.as_ref() {
                state.serialize_entry("expressedBy", some)?;
            }
            if !self.r#addresses.is_empty() {
                state.serialize_entry("addresses", &self.r#addresses)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#outcome_code.is_empty() {
                state.serialize_entry("outcomeCode", &self.r#outcome_code)?;
            }
            if !self.r#outcome_reference.is_empty() {
                state.serialize_entry("outcomeReference", &self.r#outcome_reference)?;
            }
            state.end()
        })
    }
}
