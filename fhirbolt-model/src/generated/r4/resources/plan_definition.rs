// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "A code or group definition that describes the intended subject of the plan definition."]
#[derive(Debug, Clone, PartialEq)]
pub enum PlanDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for PlanDefinitionSubject {
    fn default() -> PlanDefinitionSubject {
        PlanDefinitionSubject::Invalid
    }
}
#[doc = "The target value of the measure to be achieved to signify fulfillment of the goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any value at or above the low value."]
#[derive(Debug, Clone, PartialEq)]
pub enum PlanDefinitionGoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for PlanDefinitionGoalTargetDetail {
    fn default() -> PlanDefinitionGoalTargetDetail {
        PlanDefinitionGoalTargetDetail::Invalid
    }
}
#[doc = "A code or group definition that describes the intended subject of the action and its children, if any."]
#[derive(Debug, Clone, PartialEq)]
pub enum PlanDefinitionActionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for PlanDefinitionActionSubject {
    fn default() -> PlanDefinitionActionSubject {
        PlanDefinitionActionSubject::Invalid
    }
}
#[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
#[derive(Debug, Clone, PartialEq)]
pub enum PlanDefinitionActionRelatedActionOffset {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for PlanDefinitionActionRelatedActionOffset {
    fn default() -> PlanDefinitionActionRelatedActionOffset {
        PlanDefinitionActionRelatedActionOffset::Invalid
    }
}
#[doc = "An optional value describing when the action should be performed."]
#[derive(Debug, Clone, PartialEq)]
pub enum PlanDefinitionActionTiming {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for PlanDefinitionActionTiming {
    fn default() -> PlanDefinitionActionTiming {
        PlanDefinitionActionTiming::Invalid
    }
}
#[doc = "A reference to an ActivityDefinition that describes the action to be taken in detail, or a PlanDefinition that describes a series of actions to be taken."]
#[derive(Debug, Clone, PartialEq)]
pub enum PlanDefinitionActionDefinition {
    Canonical(Box<super::super::types::Canonical>),
    Uri(Box<super::super::types::Uri>),
    Invalid,
}
impl Default for PlanDefinitionActionDefinition {
    fn default() -> PlanDefinitionActionDefinition {
        PlanDefinitionActionDefinition::Invalid
    }
}
#[doc = "Indicates what should be done and within what timeframe."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinitionGoalTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The parameter whose value is to be tracked, e.g. body weight, blood pressure, or hemoglobin A1c level."]
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target value of the measure to be achieved to signify fulfillment of the goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any value at or above the low value."]
    pub r#detail: Option<PlanDefinitionGoalTargetDetail>,
    #[doc = "Indicates the timeframe after the start of the goal in which the goal should be met."]
    pub r#due: Option<Box<super::super::types::Duration>>,
}
impl serde::ser::Serialize for PlanDefinitionGoalTarget {
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
                    PlanDefinitionGoalTargetDetail::Quantity(ref value) => {
                        state.serialize_entry("detailQuantity", value)?;
                    }
                    PlanDefinitionGoalTargetDetail::Range(ref value) => {
                        state.serialize_entry("detailRange", value)?;
                    }
                    PlanDefinitionGoalTargetDetail::CodeableConcept(ref value) => {
                        state.serialize_entry("detailCodeableConcept", value)?;
                    }
                    PlanDefinitionGoalTargetDetail::Invalid => {
                        return Err(serde::ser::Error::custom("detail is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#due.as_ref() {
                state.serialize_entry("due", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Goals that describe what the activities within the plan are intended to achieve. For example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinitionGoal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates a category the goal falls within."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Human-readable and/or coded description of a specific desired objective of care, such as \"control blood pressure\" or \"negotiate an obstacle course\" or \"dance with child at wedding\"."]
    pub r#description: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the expected level of importance associated with reaching/sustaining the defined goal."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The event after which the goal should begin being pursued."]
    pub r#start: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies problems, conditions, issues, or concerns the goal is intended to address."]
    pub r#addresses: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Didactic or other informational resources associated with the goal that provide further supporting information about the goal. Information resources can include inline text commentary and links to web resources."]
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "Indicates what should be done and within what timeframe."]
    pub r#target: Vec<PlanDefinitionGoalTarget>,
}
impl serde::ser::Serialize for PlanDefinitionGoal {
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
            if let Some(some) = self.r#category.as_ref() {
                state.serialize_entry("category", some)?;
            }
            state.serialize_entry("description", &self.r#description)?;
            if let Some(some) = self.r#priority.as_ref() {
                state.serialize_entry("priority", some)?;
            }
            if let Some(some) = self.r#start.as_ref() {
                state.serialize_entry("start", some)?;
            }
            if !self.r#addresses.is_empty() {
                state.serialize_entry("addresses", &self.r#addresses)?;
            }
            if !self.r#documentation.is_empty() {
                state.serialize_entry("documentation", &self.r#documentation)?;
            }
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            state.end()
        })
    }
}
#[doc = "An expression that describes applicability criteria or start/stop conditions for the action."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinitionActionCondition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of condition."]
    pub r#kind: super::super::types::Code,
    #[doc = "An expression that returns true or false, indicating whether the condition is satisfied."]
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
impl serde::ser::Serialize for PlanDefinitionActionCondition {
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
                if let Some(some) = self.r#kind.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("kind", &some)?;
                }
                if self.r#kind.id.is_some() || !self.r#kind.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#kind.id.as_ref(),
                        extension: &self.r#kind.extension,
                    };
                    state.serialize_entry("_kind", &primitive_element)?;
                }
            } else {
                state.serialize_entry("kind", &self.r#kind)?;
            }
            if let Some(some) = self.r#expression.as_ref() {
                state.serialize_entry("expression", some)?;
            }
            state.end()
        })
    }
}
#[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinitionActionRelatedAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The element id of the related action."]
    pub r#action_id: super::super::types::Id,
    #[doc = "The relationship of this action to the related action."]
    pub r#relationship: super::super::types::Code,
    #[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
    pub r#offset: Option<PlanDefinitionActionRelatedActionOffset>,
}
impl serde::ser::Serialize for PlanDefinitionActionRelatedAction {
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
                if let Some(some) = self.r#action_id.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("actionId", &some)?;
                }
                if self.r#action_id.id.is_some() || !self.r#action_id.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#action_id.id.as_ref(),
                        extension: &self.r#action_id.extension,
                    };
                    state.serialize_entry("_actionId", &primitive_element)?;
                }
            } else {
                state.serialize_entry("actionId", &self.r#action_id)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#relationship.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("relationship", &some)?;
                }
                if self.r#relationship.id.is_some() || !self.r#relationship.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#relationship.id.as_ref(),
                        extension: &self.r#relationship.extension,
                    };
                    state.serialize_entry("_relationship", &primitive_element)?;
                }
            } else {
                state.serialize_entry("relationship", &self.r#relationship)?;
            }
            if let Some(some) = self.r#offset.as_ref() {
                match some {
                    PlanDefinitionActionRelatedActionOffset::Duration(ref value) => {
                        state.serialize_entry("offsetDuration", value)?;
                    }
                    PlanDefinitionActionRelatedActionOffset::Range(ref value) => {
                        state.serialize_entry("offsetRange", value)?;
                    }
                    PlanDefinitionActionRelatedActionOffset::Invalid => {
                        return Err(serde::ser::Error::custom("offset is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Indicates who should participate in performing the action described."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinitionActionParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of participant in the action."]
    pub r#type: super::super::types::Code,
    #[doc = "The role the participant should play in performing the described action."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for PlanDefinitionActionParticipant {
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
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Customizations that should be applied to the statically defined resource. For example, if the dosage of a medication must be computed based on the patient's weight, a customization would be used to specify an expression that calculated the weight, and the path on the resource that would contain the result."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinitionActionDynamicValue {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The path to the element to be customized. This is the path on the resource that will hold the result of the calculation defined by the expression. The specified path SHALL be a FHIRPath resolveable on the specified target type of the ActivityDefinition, and SHALL consist only of identifiers, constant indexers, and a restricted subset of functions. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details)."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "An expression specifying the value of the customized element."]
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
impl serde::ser::Serialize for PlanDefinitionActionDynamicValue {
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
            if let Some(some) = self.r#expression.as_ref() {
                state.serialize_entry("expression", some)?;
            }
            state.end()
        })
    }
}
#[doc = "An action or group of actions to be taken as part of the plan."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinitionAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A user-visible prefix for the action."]
    pub r#prefix: Option<super::super::types::String>,
    #[doc = "The title of the action displayed to a user."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A brief description of the action used to provide a summary to display to the user."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A text equivalent of the action to be performed. This provides a human-interpretable description of the action when the definition is consumed by a system that might not be capable of interpreting it dynamically."]
    pub r#text_equivalent: Option<super::super::types::String>,
    #[doc = "Indicates how quickly the action should be addressed with respect to other actions."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A code that provides meaning for the action or action group. For example, a section may have a LOINC code for the section of a documentation template."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A description of why this action is necessary or appropriate."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Didactic or other informational resources associated with the action that can be provided to the CDS recipient. Information resources can include inline text commentary and links to web resources."]
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "Identifies goals that this action supports. The reference must be to a goal element defined within this plan definition."]
    pub r#goal_id: Vec<super::super::types::Id>,
    #[doc = "A code or group definition that describes the intended subject of the action and its children, if any."]
    pub r#subject: Option<PlanDefinitionActionSubject>,
    #[doc = "A description of when the action should be triggered."]
    pub r#trigger: Vec<Box<super::super::types::TriggerDefinition>>,
    #[doc = "An expression that describes applicability criteria or start/stop conditions for the action."]
    pub r#condition: Vec<PlanDefinitionActionCondition>,
    #[doc = "Defines input data requirements for the action."]
    pub r#input: Vec<Box<super::super::types::DataRequirement>>,
    #[doc = "Defines the outputs of the action, if any."]
    pub r#output: Vec<Box<super::super::types::DataRequirement>>,
    #[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
    pub r#related_action: Vec<PlanDefinitionActionRelatedAction>,
    #[doc = "An optional value describing when the action should be performed."]
    pub r#timing: Option<PlanDefinitionActionTiming>,
    #[doc = "Indicates who should participate in performing the action described."]
    pub r#participant: Vec<PlanDefinitionActionParticipant>,
    #[doc = "The type of action to perform (create, update, remove)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Defines the grouping behavior for the action and its children."]
    pub r#grouping_behavior: Option<super::super::types::Code>,
    #[doc = "Defines the selection behavior for the action and its children."]
    pub r#selection_behavior: Option<super::super::types::Code>,
    #[doc = "Defines the required behavior for the action."]
    pub r#required_behavior: Option<super::super::types::Code>,
    #[doc = "Defines whether the action should usually be preselected."]
    pub r#precheck_behavior: Option<super::super::types::Code>,
    #[doc = "Defines whether the action can be selected multiple times."]
    pub r#cardinality_behavior: Option<super::super::types::Code>,
    #[doc = "A reference to an ActivityDefinition that describes the action to be taken in detail, or a PlanDefinition that describes a series of actions to be taken."]
    pub r#definition: Option<PlanDefinitionActionDefinition>,
    #[doc = "A reference to a StructureMap resource that defines a transform that can be executed to produce the intent resource using the ActivityDefinition instance as the input."]
    pub r#transform: Option<super::super::types::Canonical>,
    #[doc = "Customizations that should be applied to the statically defined resource. For example, if the dosage of a medication must be computed based on the patient's weight, a customization would be used to specify an expression that calculated the weight, and the path on the resource that would contain the result."]
    pub r#dynamic_value: Vec<PlanDefinitionActionDynamicValue>,
    #[doc = "Sub actions that are contained within the action. The behavior of this action determines the functionality of the sub-actions. For example, a selection behavior of at-most-one indicates that of the sub-actions, at most one may be chosen as part of realizing the action definition."]
    pub r#action: Vec<PlanDefinitionAction>,
}
impl serde::ser::Serialize for PlanDefinitionAction {
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
                if let Some(some) = self.r#prefix.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("prefix", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_prefix", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#prefix.as_ref() {
                    state.serialize_entry("prefix", some)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#text_equivalent.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("textEquivalent", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_textEquivalent", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text_equivalent.as_ref() {
                    state.serialize_entry("textEquivalent", some)?;
                }
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
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if !self.r#reason.is_empty() {
                state.serialize_entry("reason", &self.r#reason)?;
            }
            if !self.r#documentation.is_empty() {
                state.serialize_entry("documentation", &self.r#documentation)?;
            }
            if _ctx.output_json {
                if !self.r#goal_id.is_empty() {
                    let values = self
                        .r#goal_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("goalId", &values)?;
                    }
                    let requires_elements = self
                        .r#goal_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#goal_id
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
                        state.serialize_entry("_goalId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#goal_id.is_empty() {
                    state.serialize_entry("goalId", &self.r#goal_id)?;
                }
            }
            if let Some(some) = self.r#subject.as_ref() {
                match some {
                    PlanDefinitionActionSubject::CodeableConcept(ref value) => {
                        state.serialize_entry("subjectCodeableConcept", value)?;
                    }
                    PlanDefinitionActionSubject::Reference(ref value) => {
                        state.serialize_entry("subjectReference", value)?;
                    }
                    PlanDefinitionActionSubject::Invalid => {
                        return Err(serde::ser::Error::custom("subject is invalid"))
                    }
                }
            }
            if !self.r#trigger.is_empty() {
                state.serialize_entry("trigger", &self.r#trigger)?;
            }
            if !self.r#condition.is_empty() {
                state.serialize_entry("condition", &self.r#condition)?;
            }
            if !self.r#input.is_empty() {
                state.serialize_entry("input", &self.r#input)?;
            }
            if !self.r#output.is_empty() {
                state.serialize_entry("output", &self.r#output)?;
            }
            if !self.r#related_action.is_empty() {
                state.serialize_entry("relatedAction", &self.r#related_action)?;
            }
            if let Some(some) = self.r#timing.as_ref() {
                match some {
                    PlanDefinitionActionTiming::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timingDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timingDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timingDateTime", value)?;
                        }
                    }
                    PlanDefinitionActionTiming::Age(ref value) => {
                        state.serialize_entry("timingAge", value)?;
                    }
                    PlanDefinitionActionTiming::Period(ref value) => {
                        state.serialize_entry("timingPeriod", value)?;
                    }
                    PlanDefinitionActionTiming::Duration(ref value) => {
                        state.serialize_entry("timingDuration", value)?;
                    }
                    PlanDefinitionActionTiming::Range(ref value) => {
                        state.serialize_entry("timingRange", value)?;
                    }
                    PlanDefinitionActionTiming::Timing(ref value) => {
                        state.serialize_entry("timingTiming", value)?;
                    }
                    PlanDefinitionActionTiming::Invalid => {
                        return Err(serde::ser::Error::custom("timing is invalid"))
                    }
                }
            }
            if !self.r#participant.is_empty() {
                state.serialize_entry("participant", &self.r#participant)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#grouping_behavior.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("groupingBehavior", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_groupingBehavior", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#grouping_behavior.as_ref() {
                    state.serialize_entry("groupingBehavior", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#selection_behavior.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("selectionBehavior", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_selectionBehavior", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#selection_behavior.as_ref() {
                    state.serialize_entry("selectionBehavior", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#required_behavior.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requiredBehavior", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requiredBehavior", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#required_behavior.as_ref() {
                    state.serialize_entry("requiredBehavior", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#precheck_behavior.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("precheckBehavior", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_precheckBehavior", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#precheck_behavior.as_ref() {
                    state.serialize_entry("precheckBehavior", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#cardinality_behavior.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("cardinalityBehavior", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_cardinalityBehavior", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#cardinality_behavior.as_ref() {
                    state.serialize_entry("cardinalityBehavior", some)?;
                }
            }
            if let Some(some) = self.r#definition.as_ref() {
                match some {
                    PlanDefinitionActionDefinition::Canonical(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("definitionCanonical", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_definitionCanonical", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("definitionCanonical", value)?;
                        }
                    }
                    PlanDefinitionActionDefinition::Uri(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("definitionUri", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_definitionUri", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("definitionUri", value)?;
                        }
                    }
                    PlanDefinitionActionDefinition::Invalid => {
                        return Err(serde::ser::Error::custom("definition is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#transform.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("transform", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_transform", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#transform.as_ref() {
                    state.serialize_entry("transform", some)?;
                }
            }
            if !self.r#dynamic_value.is_empty() {
                state.serialize_entry("dynamicValue", &self.r#dynamic_value)?;
            }
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            state.end()
        })
    }
}
#[doc = "This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact. The resource is general enough to support the description of a broad range of clinical artifacts such as clinical decision support rules, order sets and protocols."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanDefinition {
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
    #[doc = "An absolute URI that is used to identify this plan definition when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this plan definition is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the plan definition is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this plan definition when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the plan definition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the plan definition author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the plan definition. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the plan definition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the plan definition giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "A high-level category for the plan definition that distinguishes the kinds of systems that would be interested in the plan definition."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of this plan definition. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this plan definition is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "A code or group definition that describes the intended subject of the plan definition."]
    pub r#subject: Option<PlanDefinitionSubject>,
    #[doc = "The date  (and optionally time) when the plan definition was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the plan definition changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the plan definition."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the plan definition from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate plan definition instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the plan definition is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this plan definition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A detailed description of how the plan definition is used from a clinical perspective."]
    pub r#usage: Option<super::super::types::String>,
    #[doc = "A copyright statement relating to the plan definition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the plan definition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the plan definition content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the plan definition. Topics provide a high-level categorization of the definition that can be useful for filtering and searching."]
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Related artifacts such as additional documentation, justification, or bibliographic references."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "A reference to a Library resource containing any formal logic used by the plan definition."]
    pub r#library: Vec<super::super::types::Canonical>,
    #[doc = "Goals that describe what the activities within the plan are intended to achieve. For example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc."]
    pub r#goal: Vec<PlanDefinitionGoal>,
    #[doc = "An action or group of actions to be taken as part of the plan."]
    pub r#action: Vec<PlanDefinitionAction>,
}
impl serde::ser::Serialize for PlanDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "PlanDefinition")?;
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
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
                if let Some(some) = self.r#subtitle.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subtitle", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subtitle", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subtitle.as_ref() {
                    state.serialize_entry("subtitle", some)?;
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
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
            if let Some(some) = self.r#subject.as_ref() {
                match some {
                    PlanDefinitionSubject::CodeableConcept(ref value) => {
                        state.serialize_entry("subjectCodeableConcept", value)?;
                    }
                    PlanDefinitionSubject::Reference(ref value) => {
                        state.serialize_entry("subjectReference", value)?;
                    }
                    PlanDefinitionSubject::Invalid => {
                        return Err(serde::ser::Error::custom("subject is invalid"))
                    }
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
                if let Some(some) = self.r#usage.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("usage", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_usage", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#usage.as_ref() {
                    state.serialize_entry("usage", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#approval_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("approvalDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_approvalDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#approval_date.as_ref() {
                    state.serialize_entry("approvalDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastReviewDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastReviewDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    state.serialize_entry("lastReviewDate", some)?;
                }
            }
            if let Some(some) = self.r#effective_period.as_ref() {
                state.serialize_entry("effectivePeriod", some)?;
            }
            if !self.r#topic.is_empty() {
                state.serialize_entry("topic", &self.r#topic)?;
            }
            if !self.r#author.is_empty() {
                state.serialize_entry("author", &self.r#author)?;
            }
            if !self.r#editor.is_empty() {
                state.serialize_entry("editor", &self.r#editor)?;
            }
            if !self.r#reviewer.is_empty() {
                state.serialize_entry("reviewer", &self.r#reviewer)?;
            }
            if !self.r#endorser.is_empty() {
                state.serialize_entry("endorser", &self.r#endorser)?;
            }
            if !self.r#related_artifact.is_empty() {
                state.serialize_entry("relatedArtifact", &self.r#related_artifact)?;
            }
            if _ctx.output_json {
                if !self.r#library.is_empty() {
                    let values = self
                        .r#library
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("library", &values)?;
                    }
                    let requires_elements = self
                        .r#library
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#library
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
                        state.serialize_entry("_library", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#library.is_empty() {
                    state.serialize_entry("library", &self.r#library)?;
                }
            }
            if !self.r#goal.is_empty() {
                state.serialize_entry("goal", &self.r#goal)?;
            }
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            state.end()
        })
    }
}
