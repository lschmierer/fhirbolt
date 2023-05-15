// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "A date or period in the past or future indicating when the event occurred or is expectd to occur."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageEligibilityResponseEventWhen {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "The date or dates when the enclosed suite of services were performed or completed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageEligibilityResponseServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "The quantity of the benefit which is permitted under the coverage."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
    #[default]
    Invalid,
}
#[doc = "The quantity of the benefit which have been consumed to date."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
    #[default]
    Invalid,
}
#[doc = "Information code for an event with a corresponding date or period."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseEvent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A coded event such as when a service is expected or a card printed."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A date or period in the past or future indicating when the event occurred or is expectd to occur."]
    pub r#when: CoverageEligibilityResponseEventWhen,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageEligibilityResponseEvent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#when: Default::default(),
        }
    }
}
#[doc = "Benefits used to date."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Classification of benefit being provided."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The quantity of the benefit which is permitted under the coverage."]
    pub r#allowed: Option<CoverageEligibilityResponseInsuranceItemBenefitAllowed>,
    #[doc = "The quantity of the benefit which have been consumed to date."]
    pub r#used: Option<CoverageEligibilityResponseInsuranceItemBenefitUsed>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageEligibilityResponseInsuranceItemBenefit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#allowed: Default::default(),
            r#used: Default::default(),
        }
    }
}
#[doc = "Benefits and optionally current balances, and authorization details by category or service."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseInsuranceItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "The practitioner who is eligible for the provision of the product or service."]
    pub r#provider: Option<Box<super::super::types::Reference>>,
    #[doc = "True if the indicated class of service is excluded from the plan, missing or False indicates the product or service is included in the coverage."]
    pub r#excluded: Option<super::super::types::Boolean>,
    #[doc = "A short name or tag for the benefit."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A richer description of the benefit or services covered."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Is a flag to indicate whether the benefits refer to in-network providers or out-of-network providers."]
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates if the benefits apply to an individual or to the family."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The term or period of the values such as 'maximum lifetime benefit' or 'maximum annual visits'."]
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Benefits used to date."]
    pub r#benefit: Vec<CoverageEligibilityResponseInsuranceItemBenefit>,
    #[doc = "A boolean flag indicating whether a preauthorization is required prior to actual service delivery."]
    pub r#authorization_required: Option<super::super::types::Boolean>,
    #[doc = "Codes or comments regarding information or actions associated with the preauthorization."]
    pub r#authorization_supporting: Vec<super::super::types::CodeableConcept>,
    #[doc = "A web location for obtaining requirements or descriptive information regarding the preauthorization."]
    pub r#authorization_url: Option<super::super::types::Uri>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageEligibilityResponseInsuranceItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Default::default(),
            r#product_or_service: Default::default(),
            r#modifier: Default::default(),
            r#provider: Default::default(),
            r#excluded: Default::default(),
            r#name: Default::default(),
            r#description: Default::default(),
            r#network: Default::default(),
            r#unit: Default::default(),
            r#term: Default::default(),
            r#benefit: Default::default(),
            r#authorization_required: Default::default(),
            r#authorization_supporting: Default::default(),
            r#authorization_url: Default::default(),
        }
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "Flag indicating if the coverage provided is inforce currently if no service date(s) specified or for the whole duration of the service dates."]
    pub r#inforce: Option<super::super::types::Boolean>,
    #[doc = "The term of the benefits documented in this response."]
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    #[doc = "Benefits and optionally current balances, and authorization details by category or service."]
    pub r#item: Vec<CoverageEligibilityResponseInsuranceItem>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageEligibilityResponseInsurance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#coverage: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#inforce: Default::default(),
            r#benefit_period: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "Errors encountered during the processing of the request."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponseError {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An error code,from a specified code system, which details why the eligibility check could not be performed."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names, repetition indicators and the default child accessor that identifies one of the elements in the resource that caused this issue to be raised."]
    pub r#expression: Vec<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageEligibilityResponseError {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#expression: Default::default(),
        }
    }
}
#[doc = "This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageEligibilityResponse {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
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
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique identifier assigned to this coverage eligiblity request."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "Code to specify whether requesting: prior authorization requirements for some service categories or billing codes; benefits for coverages specified or discovered; discovery and return of coverages for the patient; and/or validation that the specified coverage is in-force at the date/period specified or 'now' if not specified."]
    pub r#purpose: Vec<super::super::types::Code>,
    #[doc = "The party who is the beneficiary of the supplied coverage and for whom eligibility is sought."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "Information code for an event with a corresponding date or period."]
    pub r#event: Vec<CoverageEligibilityResponseEvent>,
    #[doc = "The date or dates when the enclosed suite of services were performed or completed."]
    pub r#serviced: Option<CoverageEligibilityResponseServiced>,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "The provider which is responsible for the request."]
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    #[doc = "Reference to the original request resource."]
    pub r#request: Box<super::super::types::Reference>,
    #[doc = "The outcome of the request processing."]
    pub r#outcome: super::super::types::Code,
    #[doc = "A human readable description of the status of the adjudication."]
    pub r#disposition: Option<super::super::types::String>,
    #[doc = "The Insurer who issued the coverage in question and is the author of the response."]
    pub r#insurer: Box<super::super::types::Reference>,
    #[doc = "Financial instruments for reimbursement for the health care products and services."]
    pub r#insurance: Vec<CoverageEligibilityResponseInsurance>,
    #[doc = "A reference from the Insurer to which these services pertain to be used on further communication and as proof that the request occurred."]
    pub r#pre_auth_ref: Option<super::super::types::String>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Errors encountered during the processing of the request."]
    pub r#error: Vec<CoverageEligibilityResponseError>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageEligibilityResponse {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#purpose: Default::default(),
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#event: Default::default(),
            r#serviced: Default::default(),
            r#created: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#requestor: Default::default(),
            r#request: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#outcome: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#disposition: Default::default(),
            r#insurer: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#insurance: Default::default(),
            r#pre_auth_ref: Default::default(),
            r#form: Default::default(),
            r#error: Default::default(),
        }
    }
}
