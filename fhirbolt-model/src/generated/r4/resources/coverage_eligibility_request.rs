// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The date or dates when the enclosed suite of services were performed or completed."]
#[derive(Debug, Clone, PartialEq)]
pub enum CoverageEligibilityRequestServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for CoverageEligibilityRequestServiced {
    fn default() -> CoverageEligibilityRequestServiced {
        CoverageEligibilityRequestServiced::Invalid
    }
}
#[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
#[derive(Debug, Clone, PartialEq)]
pub enum CoverageEligibilityRequestItemDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for CoverageEligibilityRequestItemDiagnosisDiagnosis {
    fn default() -> CoverageEligibilityRequestItemDiagnosisDiagnosis {
        CoverageEligibilityRequestItemDiagnosisDiagnosis::Invalid
    }
}
#[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify supporting information entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
    pub r#information: Box<super::super::types::Reference>,
    #[doc = "The supporting materials are applicable for all detail items, product/servce categories and specific billing codes."]
    pub r#applies_to_all: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for CoverageEligibilityRequestSupportingInfo {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
            }
            state.serialize_entry("information", &self.r#information)?;
            if _ctx.output_json {
                if let Some(some) = self.r#applies_to_all.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("appliesToAll", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_appliesToAll", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#applies_to_all.as_ref() {
                    state.serialize_entry("appliesToAll", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A flag to indicate that this Coverage is to be used for evaluation of this request when set to true."]
    pub r#focal: Option<super::super::types::Boolean>,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "A business agreement number established between the provider and the insurer for special business processing purposes."]
    pub r#business_arrangement: Option<super::super::types::String>,
}
impl serde::ser::Serialize for CoverageEligibilityRequestInsurance {
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
                if let Some(some) = self.r#focal.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("focal", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_focal", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#focal.as_ref() {
                    state.serialize_entry("focal", some)?;
                }
            }
            state.serialize_entry("coverage", &self.r#coverage)?;
            if _ctx.output_json {
                if let Some(some) = self.r#business_arrangement.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("businessArrangement", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_businessArrangement", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#business_arrangement.as_ref() {
                    state.serialize_entry("businessArrangement", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Patient diagnosis for which care is sought."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestItemDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
    pub r#diagnosis: Option<CoverageEligibilityRequestItemDiagnosisDiagnosis>,
}
impl serde::ser::Serialize for CoverageEligibilityRequestItemDiagnosis {
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
            if let Some(some) = self.r#diagnosis.as_ref() {
                match some {
                    CoverageEligibilityRequestItemDiagnosisDiagnosis::CodeableConcept(
                        ref value,
                    ) => {
                        state.serialize_entry("diagnosisCodeableConcept", value)?;
                    }
                    CoverageEligibilityRequestItemDiagnosisDiagnosis::Reference(ref value) => {
                        state.serialize_entry("diagnosisReference", value)?;
                    }
                    CoverageEligibilityRequestItemDiagnosisDiagnosis::Invalid => {
                        return Err(serde::ser::Error::custom("diagnosis is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Service categories or billable services for which benefit details and/or an authorization prior to service delivery may be required by the payor."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Exceptions, special conditions and supporting information applicable for this service or product line."]
    pub r#supporting_info_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The practitioner who is responsible for the product or service to be rendered to the patient."]
    pub r#provider: Option<Box<super::super::types::Reference>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The amount charged to the patient by the provider for a single unit."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "Facility where the services will be provided."]
    pub r#facility: Option<Box<super::super::types::Reference>>,
    #[doc = "Patient diagnosis for which care is sought."]
    pub r#diagnosis: Vec<CoverageEligibilityRequestItemDiagnosis>,
    #[doc = "The plan/proposal/order describing the proposed service in detail."]
    pub r#detail: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CoverageEligibilityRequestItem {
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
                if !self.r#supporting_info_sequence.is_empty() {
                    let values = self
                        .r#supporting_info_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("supportingInfoSequence", &values)?;
                    }
                    let requires_elements = self
                        .r#supporting_info_sequence
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#supporting_info_sequence
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
                        state.serialize_entry("_supportingInfoSequence", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#supporting_info_sequence.is_empty() {
                    state.serialize_entry(
                        "supportingInfoSequence",
                        &self.r#supporting_info_sequence,
                    )?;
                }
            }
            if let Some(some) = self.r#category.as_ref() {
                state.serialize_entry("category", some)?;
            }
            if let Some(some) = self.r#product_or_service.as_ref() {
                state.serialize_entry("productOrService", some)?;
            }
            if !self.r#modifier.is_empty() {
                state.serialize_entry("modifier", &self.r#modifier)?;
            }
            if let Some(some) = self.r#provider.as_ref() {
                state.serialize_entry("provider", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if let Some(some) = self.r#unit_price.as_ref() {
                state.serialize_entry("unitPrice", some)?;
            }
            if let Some(some) = self.r#facility.as_ref() {
                state.serialize_entry("facility", some)?;
            }
            if !self.r#diagnosis.is_empty() {
                state.serialize_entry("diagnosis", &self.r#diagnosis)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
#[doc = "The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding whether the stated coverage is valid and in-force and optionally to provide the insurance details of the policy."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequest {
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
    #[doc = "A unique identifier assigned to this coverage eligiblity request."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "When the requestor expects the processor to complete processing."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to specify whether requesting: prior authorization requirements for some service categories or billing codes; benefits for coverages specified or discovered; discovery and return of coverages for the patient; and/or validation that the specified coverage is in-force at the date/period specified or 'now' if not specified."]
    pub r#purpose: Vec<super::super::types::Code>,
    #[doc = "The party who is the beneficiary of the supplied coverage and for whom eligibility is sought."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date or dates when the enclosed suite of services were performed or completed."]
    pub r#serviced: Option<CoverageEligibilityRequestServiced>,
    #[doc = "The date when this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "Person who created the request."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "The provider which is responsible for the request."]
    pub r#provider: Option<Box<super::super::types::Reference>>,
    #[doc = "The Insurer who issued the coverage in question and is the recipient of the request."]
    pub r#insurer: Box<super::super::types::Reference>,
    #[doc = "Facility where the services are intended to be provided."]
    pub r#facility: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
    pub r#supporting_info: Vec<CoverageEligibilityRequestSupportingInfo>,
    #[doc = "Financial instruments for reimbursement for the health care products and services."]
    pub r#insurance: Vec<CoverageEligibilityRequestInsurance>,
    #[doc = "Service categories or billable services for which benefit details and/or an authorization prior to service delivery may be required by the payor."]
    pub r#item: Vec<CoverageEligibilityRequestItem>,
}
impl serde::ser::Serialize for CoverageEligibilityRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "CoverageEligibilityRequest")?;
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
            if let Some(some) = self.r#priority.as_ref() {
                state.serialize_entry("priority", some)?;
            }
            if _ctx.output_json {
                if !self.r#purpose.is_empty() {
                    let values = self
                        .r#purpose
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("purpose", &values)?;
                    }
                    let requires_elements = self
                        .r#purpose
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#purpose
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
                        state.serialize_entry("_purpose", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#purpose.is_empty() {
                    state.serialize_entry("purpose", &self.r#purpose)?;
                }
            }
            state.serialize_entry("patient", &self.r#patient)?;
            if let Some(some) = self.r#serviced.as_ref() {
                match some {
                    CoverageEligibilityRequestServiced::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("servicedDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_servicedDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("servicedDate", value)?;
                        }
                    }
                    CoverageEligibilityRequestServiced::Period(ref value) => {
                        state.serialize_entry("servicedPeriod", value)?;
                    }
                    CoverageEligibilityRequestServiced::Invalid => {
                        return Err(serde::ser::Error::custom("serviced is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#created.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("created", &some)?;
                }
                if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#created.id.as_ref(),
                        extension: &self.r#created.extension,
                    };
                    state.serialize_entry("_created", &primitive_element)?;
                }
            } else {
                state.serialize_entry("created", &self.r#created)?;
            }
            if let Some(some) = self.r#enterer.as_ref() {
                state.serialize_entry("enterer", some)?;
            }
            if let Some(some) = self.r#provider.as_ref() {
                state.serialize_entry("provider", some)?;
            }
            state.serialize_entry("insurer", &self.r#insurer)?;
            if let Some(some) = self.r#facility.as_ref() {
                state.serialize_entry("facility", some)?;
            }
            if !self.r#supporting_info.is_empty() {
                state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
            }
            if !self.r#insurance.is_empty() {
                state.serialize_entry("insurance", &self.r#insurance)?;
            }
            if !self.r#item.is_empty() {
                state.serialize_entry("item", &self.r#item)?;
            }
            state.end()
        })
    }
}
