// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The FHIR query based rules that the server should use to determine when to trigger a notification for this subscription topic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscriptionTopicResourceTriggerQueryCriteria {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The FHIR query based rules are applied to the previous resource state (e.g., state before an update)."]
    pub r#previous: Option<super::super::types::String>,
    #[doc = "For \"create\" interactions, should the \"previous\" criteria count as an automatic pass or an automatic fail."]
    pub r#result_for_create: Option<super::super::types::Code>,
    #[doc = "The FHIR query based rules are applied to the current resource state (e.g., state after an update)."]
    pub r#current: Option<super::super::types::String>,
    #[doc = "For \"delete\" interactions, should the \"current\" criteria count as an automatic pass or an automatic fail."]
    pub r#result_for_delete: Option<super::super::types::Code>,
    #[doc = "If set to true, both current and previous criteria must evaluate true to  trigger a notification for this topic.  Otherwise a notification for this topic will be triggered if either one evaluates to true."]
    pub r#require_both: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for SubscriptionTopicResourceTriggerQueryCriteria {
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
                if let Some(some) = self.r#previous.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("previous", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_previous", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#previous.as_ref() {
                    state.serialize_entry("previous", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#result_for_create.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("resultForCreate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_resultForCreate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#result_for_create.as_ref() {
                    state.serialize_entry("resultForCreate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#current.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("current", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_current", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#current.as_ref() {
                    state.serialize_entry("current", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#result_for_delete.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("resultForDelete", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_resultForDelete", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#result_for_delete.as_ref() {
                    state.serialize_entry("resultForDelete", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#require_both.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requireBoth", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requireBoth", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#require_both.as_ref() {
                    state.serialize_entry("requireBoth", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "A definition of a resource-based event that triggers a notification based on the SubscriptionTopic. The criteria may be just a human readable description and/or a full FHIR search string or FHIRPath expression. Multiple triggers are considered OR joined (e.g., a resource update matching ANY of the definitions will trigger a notification)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscriptionTopicResourceTrigger {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The human readable description of this resource trigger for the SubscriptionTopic -  for example, \"An Encounter enters the 'in-progress' state\"."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "URL of the Resource that is the type used in this resource trigger.  Relative URLs are relative to the StructureDefinition root of the implemented FHIR version (e.g., <http://hl7.org/fhir/StructureDefinition>). For example, \"Patient\" maps to <http://hl7.org/fhir/StructureDefinition/Patient>.  For more information, see <a href=\"elementdefinition-definitions.html#ElementDefinition.type.code\">ElementDefinition.type.code</a>."]
    pub r#resource: super::super::types::Uri,
    #[doc = "The FHIR RESTful interaction which can be used to trigger a notification for the SubscriptionTopic. Multiple values are considered OR joined (e.g., CREATE or UPDATE)."]
    pub r#supported_interaction: Vec<super::super::types::Code>,
    #[doc = "The FHIR query based rules that the server should use to determine when to trigger a notification for this subscription topic."]
    pub r#query_criteria: Option<SubscriptionTopicResourceTriggerQueryCriteria>,
    #[doc = "The FHIRPath based rules that the server should use to determine when to trigger a notification for this topic."]
    pub r#fhir_path_criteria: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubscriptionTopicResourceTrigger {
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
                if let Some(some) = self.r#resource.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("resource", &some)?;
                }
                if self.r#resource.id.is_some() || !self.r#resource.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#resource.id.as_ref(),
                        extension: &self.r#resource.extension,
                    };
                    state.serialize_entry("_resource", &primitive_element)?;
                }
            } else {
                state.serialize_entry("resource", &self.r#resource)?;
            }
            if _ctx.output_json {
                if !self.r#supported_interaction.is_empty() {
                    let values = self
                        .r#supported_interaction
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("supportedInteraction", &values)?;
                    }
                    let requires_elements = self
                        .r#supported_interaction
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#supported_interaction
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
                        state.serialize_entry("_supportedInteraction", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#supported_interaction.is_empty() {
                    state.serialize_entry("supportedInteraction", &self.r#supported_interaction)?;
                }
            }
            if let Some(some) = self.r#query_criteria.as_ref() {
                state.serialize_entry("queryCriteria", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#fhir_path_criteria.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("fhirPathCriteria", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_fhirPathCriteria", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#fhir_path_criteria.as_ref() {
                    state.serialize_entry("fhirPathCriteria", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Event definition which can be used to trigger the SubscriptionTopic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscriptionTopicEventTrigger {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The human readable description of an event to trigger a notification for the SubscriptionTopic - for example, \"Patient Admission, as defined in HL7v2 via message ADT^A01\". Multiple values are considered OR joined (e.g., matching any single event listed)."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A well-defined event which can be used to trigger notifications from the SubscriptionTopic."]
    pub r#event: Box<super::super::types::CodeableConcept>,
    #[doc = "URL of the Resource that is the focus type used in this event trigger.  Relative URLs are relative to the StructureDefinition root of the implemented FHIR version (e.g., <http://hl7.org/fhir/StructureDefinition>). For example, \"Patient\" maps to <http://hl7.org/fhir/StructureDefinition/Patient>.  For more information, see <a href=\"elementdefinition-definitions.html#ElementDefinition.type.code\">ElementDefinition.type.code</a>."]
    pub r#resource: super::super::types::Uri,
}
impl serde::ser::Serialize for SubscriptionTopicEventTrigger {
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
            state.serialize_entry("event", &self.r#event)?;
            if _ctx.output_json {
                if let Some(some) = self.r#resource.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("resource", &some)?;
                }
                if self.r#resource.id.is_some() || !self.r#resource.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#resource.id.as_ref(),
                        extension: &self.r#resource.extension,
                    };
                    state.serialize_entry("_resource", &primitive_element)?;
                }
            } else {
                state.serialize_entry("resource", &self.r#resource)?;
            }
            state.end()
        })
    }
}
#[doc = "List of properties by which Subscriptions on the SubscriptionTopic can be filtered. May be defined Search Parameters (e.g., Encounter.patient) or parameters defined within this SubscriptionTopic context (e.g., hub.event)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscriptionTopicCanFilterBy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of how this filtering parameter is intended to be used."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "URL of the Resource that is the type used in this filter. This is the \"focus\" of the topic (or one of them if there are more than one). It will be the same, a generality, or a specificity of SubscriptionTopic.resourceTrigger.resource or SubscriptionTopic.eventTrigger.resource when they are present."]
    pub r#resource: Option<super::super::types::Uri>,
    #[doc = "Either the canonical URL to a search parameter (like \"<http://hl7.org/fhir/SearchParameter/encounter>-patient\") or topic-defined parameter (like \"hub.event\") which is a label for the filter."]
    pub r#filter_parameter: super::super::types::String,
    #[doc = "Either the canonical URL to a search parameter (like \"<http://hl7.org/fhir/SearchParameter/encounter>-patient\") or the officially-defined URI for a shared filter concept (like \"<http://example.org/concepts/shared>-common-event\")."]
    pub r#filter_definition: Option<super::super::types::Uri>,
    #[doc = "Allowable operators to apply when determining matches (Search Modifiers).  If the filterParameter is a SearchParameter, this list of modifiers SHALL be a strict subset of the modifiers defined on that SearchParameter."]
    pub r#modifier: Vec<super::super::types::Code>,
}
impl serde::ser::Serialize for SubscriptionTopicCanFilterBy {
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
                if let Some(some) = self.r#resource.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("resource", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_resource", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#resource.as_ref() {
                    state.serialize_entry("resource", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#filter_parameter.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("filterParameter", &some)?;
                }
                if self.r#filter_parameter.id.is_some()
                    || !self.r#filter_parameter.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#filter_parameter.id.as_ref(),
                        extension: &self.r#filter_parameter.extension,
                    };
                    state.serialize_entry("_filterParameter", &primitive_element)?;
                }
            } else {
                state.serialize_entry("filterParameter", &self.r#filter_parameter)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#filter_definition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("filterDefinition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_filterDefinition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#filter_definition.as_ref() {
                    state.serialize_entry("filterDefinition", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#modifier.is_empty() {
                    let values = self
                        .r#modifier
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("modifier", &values)?;
                    }
                    let requires_elements = self
                        .r#modifier
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#modifier
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
                        state.serialize_entry("_modifier", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#modifier.is_empty() {
                    state.serialize_entry("modifier", &self.r#modifier)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "List of properties to describe the shape (e.g., resources) included in notifications from this Subscription Topic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscriptionTopicNotificationShape {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "URL of the Resource that is the type used in this shape. This is the \"focus\" of the topic (or one of them if there are more than one) and the root resource for this shape definition. It will be the same, a generality, or a specificity of SubscriptionTopic.resourceTrigger.resource or SubscriptionTopic.eventTrigger.resource when they are present."]
    pub r#resource: super::super::types::Uri,
    #[doc = "Search-style _include directives, rooted in the resource for this shape. Servers SHOULD include resources listed here, if they exist and the user is authorized to receive them.  Clients SHOULD be prepared to receive these additional resources, but SHALL function properly without them."]
    pub r#include: Vec<super::super::types::String>,
    #[doc = "Search-style _revinclude directives, rooted in the resource for this shape. Servers SHOULD include resources listed here, if they exist and the user is authorized to receive them.  Clients SHOULD be prepared to receive these additional resources, but SHALL function properly without them."]
    pub r#rev_include: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for SubscriptionTopicNotificationShape {
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
                if let Some(some) = self.r#resource.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("resource", &some)?;
                }
                if self.r#resource.id.is_some() || !self.r#resource.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#resource.id.as_ref(),
                        extension: &self.r#resource.extension,
                    };
                    state.serialize_entry("_resource", &primitive_element)?;
                }
            } else {
                state.serialize_entry("resource", &self.r#resource)?;
            }
            if _ctx.output_json {
                if !self.r#include.is_empty() {
                    let values = self
                        .r#include
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("include", &values)?;
                    }
                    let requires_elements = self
                        .r#include
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#include
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
                        state.serialize_entry("_include", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#include.is_empty() {
                    state.serialize_entry("include", &self.r#include)?;
                }
            }
            if _ctx.output_json {
                if !self.r#rev_include.is_empty() {
                    let values = self
                        .r#rev_include
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("revInclude", &values)?;
                    }
                    let requires_elements = self
                        .r#rev_include
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#rev_include
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
                        state.serialize_entry("_revInclude", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#rev_include.is_empty() {
                    state.serialize_entry("revInclude", &self.r#rev_include)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Describes a stream of resource state changes identified by trigger criteria and annotated with labels useful to filter projections from this topic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscriptionTopic {
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
    #[doc = "An absolute URI that is used to identify this subscription topic when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this subscription topic is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the subscription topic is stored on different servers."]
    pub r#url: super::super::types::Uri,
    #[doc = "Business identifiers assigned to this subscription topic by the performer and/or other systems.  These identifiers remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the subscription topic when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the Topic author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions are orderable."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the SubscriptionTopic, for example, \"admission\"."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The canonical URL pointing to another FHIR-defined SubscriptionTopic that is adhered to in whole or in part by this SubscriptionTopic."]
    pub r#derived_from: Vec<super::super::types::Canonical>,
    #[doc = "The current state of the SubscriptionTopic."]
    pub r#status: super::super::types::Code,
    #[doc = "A flag to indicate that this TopSubscriptionTopicic is authored for testing purposes (or education/evaluation/marketing), and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "For draft definitions, indicates the date of initial creation.  For active definitions, represents the date of activation.  For withdrawn definitions, indicates the date of withdrawal."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Helps establish the \"authority/credibility\" of the SubscriptionTopic.  May also allow for contact."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the Topic from the consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These terms may be used to assist with indexing and searching of code system definitions."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A jurisdiction in which the Topic is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explains why this Topic is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the SubscriptionTopic and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the SubscriptionTopic."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the asset content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the asset content was last reviewed. Review happens periodically after that, but doesn't change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the SubscriptionTopic content was or is planned to be effective."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "A definition of a resource-based event that triggers a notification based on the SubscriptionTopic. The criteria may be just a human readable description and/or a full FHIR search string or FHIRPath expression. Multiple triggers are considered OR joined (e.g., a resource update matching ANY of the definitions will trigger a notification)."]
    pub r#resource_trigger: Vec<SubscriptionTopicResourceTrigger>,
    #[doc = "Event definition which can be used to trigger the SubscriptionTopic."]
    pub r#event_trigger: Vec<SubscriptionTopicEventTrigger>,
    #[doc = "List of properties by which Subscriptions on the SubscriptionTopic can be filtered. May be defined Search Parameters (e.g., Encounter.patient) or parameters defined within this SubscriptionTopic context (e.g., hub.event)."]
    pub r#can_filter_by: Vec<SubscriptionTopicCanFilterBy>,
    #[doc = "List of properties to describe the shape (e.g., resources) included in notifications from this Subscription Topic."]
    pub r#notification_shape: Vec<SubscriptionTopicNotificationShape>,
}
impl serde::ser::Serialize for SubscriptionTopic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubscriptionTopic")?;
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
                if let Some(some) = self.r#url.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#url.id.as_ref(),
                        extension: &self.r#url.extension,
                    };
                    state.serialize_entry("_url", &primitive_element)?;
                }
            } else {
                state.serialize_entry("url", &self.r#url)?;
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
                if !self.r#derived_from.is_empty() {
                    let values = self
                        .r#derived_from
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("derivedFrom", &values)?;
                    }
                    let requires_elements = self
                        .r#derived_from
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#derived_from
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
                        state.serialize_entry("_derivedFrom", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#derived_from.is_empty() {
                    state.serialize_entry("derivedFrom", &self.r#derived_from)?;
                }
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
            if !self.r#resource_trigger.is_empty() {
                state.serialize_entry("resourceTrigger", &self.r#resource_trigger)?;
            }
            if !self.r#event_trigger.is_empty() {
                state.serialize_entry("eventTrigger", &self.r#event_trigger)?;
            }
            if !self.r#can_filter_by.is_empty() {
                state.serialize_entry("canFilterBy", &self.r#can_filter_by)?;
            }
            if !self.r#notification_shape.is_empty() {
                state.serialize_entry("notificationShape", &self.r#notification_shape)?;
            }
            state.end()
        })
    }
}
