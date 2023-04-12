// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The date or period when the detected issue was initially identified."]
#[derive(Debug, Clone, PartialEq)]
pub enum DetectedIssueIdentified {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for DetectedIssueIdentified {
    fn default() -> DetectedIssueIdentified {
        DetectedIssueIdentified::Invalid
    }
}
#[doc = "Supporting evidence or manifestations that provide the basis for identifying the detected issue such as a GuidanceResponse or MeasureReport."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetectedIssueEvidence {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A manifestation that led to the recording of this detected issue."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Links to resources that constitute evidence for the detected issue such as a GuidanceResponse or MeasureReport."]
    pub r#detail: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for DetectedIssueEvidence {
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
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
#[doc = "Indicates an action that has been taken or is committed to reduce or eliminate the likelihood of the risk identified by the detected issue from manifesting.  Can also reflect an observation of known mitigating factors that may reduce/eliminate the need for any action."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetectedIssueMitigation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the action that was taken or the observation that was made that reduces/eliminates the risk associated with the identified issue."]
    pub r#action: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates when the mitigating action was documented."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Identifies the practitioner who determined the mitigation and takes responsibility for the mitigation step occurring."]
    pub r#author: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for DetectedIssueMitigation {
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
            state.serialize_entry("action", &self.r#action)?;
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
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetectedIssue {
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
    #[doc = "Business identifier associated with the detected issue record."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the status of the detected issue."]
    pub r#status: super::super::types::Code,
    #[doc = "Identifies the general type of issue identified."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the degree of importance associated with the identified issue based on the potential impact on the patient."]
    pub r#severity: Option<super::super::types::Code>,
    #[doc = "Indicates the patient whose record the detected issue is associated with."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "The date or period when the detected issue was initially identified."]
    pub r#identified: Option<DetectedIssueIdentified>,
    #[doc = "Individual or device responsible for the issue being raised.  For example, a decision support application or a pharmacist conducting a medication review."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the resource representing the current activity or proposed activity that is potentially problematic."]
    pub r#implicated: Vec<Box<super::super::types::Reference>>,
    #[doc = "Supporting evidence or manifestations that provide the basis for identifying the detected issue such as a GuidanceResponse or MeasureReport."]
    pub r#evidence: Vec<DetectedIssueEvidence>,
    #[doc = "A textual explanation of the detected issue."]
    pub r#detail: Option<super::super::types::String>,
    #[doc = "The literature, knowledge-base or similar reference that describes the propensity for the detected issue identified."]
    pub r#reference: Option<super::super::types::Uri>,
    #[doc = "Indicates an action that has been taken or is committed to reduce or eliminate the likelihood of the risk identified by the detected issue from manifesting.  Can also reflect an observation of known mitigating factors that may reduce/eliminate the need for any action."]
    pub r#mitigation: Vec<DetectedIssueMitigation>,
}
impl serde::ser::Serialize for DetectedIssue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "DetectedIssue")?;
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#severity.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("severity", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_severity", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#severity.as_ref() {
                    state.serialize_entry("severity", some)?;
                }
            }
            if let Some(some) = self.r#patient.as_ref() {
                state.serialize_entry("patient", some)?;
            }
            if let Some(some) = self.r#identified.as_ref() {
                match some {
                    DetectedIssueIdentified::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("identifiedDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_identifiedDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("identifiedDateTime", value)?;
                        }
                    }
                    DetectedIssueIdentified::Period(ref value) => {
                        state.serialize_entry("identifiedPeriod", value)?;
                    }
                    DetectedIssueIdentified::Invalid => {
                        return Err(serde::ser::Error::custom("identified is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            if !self.r#implicated.is_empty() {
                state.serialize_entry("implicated", &self.r#implicated)?;
            }
            if !self.r#evidence.is_empty() {
                state.serialize_entry("evidence", &self.r#evidence)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#detail.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("detail", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_detail", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#detail.as_ref() {
                    state.serialize_entry("detail", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#reference.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("reference", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_reference", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#reference.as_ref() {
                    state.serialize_entry("reference", some)?;
                }
            }
            if !self.r#mitigation.is_empty() {
                state.serialize_entry("mitigation", &self.r#mitigation)?;
            }
            state.end()
        })
    }
}
