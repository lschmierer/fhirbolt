// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "A date or period in the past or future indicating when the event occurred or is expectd to occur."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageEligibilityRequestEventWhen {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "The date or dates when the enclosed suite of services were performed or completed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageEligibilityRequestServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageEligibilityRequestItemDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Information code for an event with a corresponding date or period."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestEvent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A coded event such as when a service is expected or a card printed."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A date or period in the past or future indicating when the event occurred or is expectd to occur."]
    pub r#when: CoverageEligibilityRequestEventWhen,
}
impl Default for CoverageEligibilityRequestEvent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#when: Default::default(),
        }
    }
}
#[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify supporting information entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
    pub r#information: Box<super::super::types::Reference>,
    #[doc = "The supporting materials are applicable for all detail items, product/servce categories and specific billing codes."]
    pub r#applies_to_all: Option<super::super::types::Boolean>,
}
impl Default for CoverageEligibilityRequestSupportingInfo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: {
                let mut default: super::super::types::PositiveInt = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#information: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#applies_to_all: Default::default(),
        }
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A flag to indicate that this Coverage is to be used for evaluation of this request when set to true."]
    pub r#focal: Option<super::super::types::Boolean>,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "A business agreement number established between the provider and the insurer for special business processing purposes."]
    pub r#business_arrangement: Option<super::super::types::String>,
}
impl Default for CoverageEligibilityRequestInsurance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#focal: Default::default(),
            r#coverage: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#business_arrangement: Default::default(),
        }
    }
}
#[doc = "Patient diagnosis for which care is sought."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestItemDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
    pub r#diagnosis: Option<CoverageEligibilityRequestItemDiagnosisDiagnosis>,
}
impl Default for CoverageEligibilityRequestItemDiagnosis {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#diagnosis: Default::default(),
        }
    }
}
#[doc = "Service categories or billable services for which benefit details and/or an authorization prior to service delivery may be required by the payor."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityRequestItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
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
impl Default for CoverageEligibilityRequestItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#supporting_info_sequence: Default::default(),
            r#category: Default::default(),
            r#product_or_service: Default::default(),
            r#modifier: Default::default(),
            r#provider: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#facility: Default::default(),
            r#diagnosis: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding whether the stated coverage is valid and in-force and optionally to provide the insurance details of the policy."]
#[derive(Debug, Clone, PartialEq)]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
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
    #[doc = "Information code for an event with a corresponding date or period."]
    pub r#event: Vec<CoverageEligibilityRequestEvent>,
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
impl Default for CoverageEligibilityRequest {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#priority: Default::default(),
            r#purpose: Default::default(),
            r#patient: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#event: Default::default(),
            r#serviced: Default::default(),
            r#created: {
                let mut default: super::super::types::DateTime = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#enterer: Default::default(),
            r#provider: Default::default(),
            r#insurer: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#facility: Default::default(),
            r#supporting_info: Default::default(),
            r#insurance: Default::default(),
            r#item: Default::default(),
        }
    }
}
