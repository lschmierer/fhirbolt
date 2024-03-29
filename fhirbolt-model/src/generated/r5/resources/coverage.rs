// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The amount due from the patient for the cost category."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CoverageCostToBeneficiaryValue {
    Quantity(Box<super::super::types::Quantity>),
    Money(Box<super::super::types::Money>),
    #[default]
    Invalid,
}
#[doc = "Link to the paying party and optionally what specifically they will be responsible to pay."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoveragePaymentBy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The list of parties providing non-insurance payment for the treatment costs."]
    pub r#party: Box<super::super::types::Reference>,
    #[doc = " Description of the financial responsibility."]
    pub r#responsibility: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoveragePaymentBy {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#party: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#responsibility: Default::default(),
        }
    }
}
#[doc = "A suite of underwriter specific classifiers."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageClass {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of classification for which an insurer-specific class label or number and optional name is provided.  For example, type may be used to identify a class of coverage or employer group, policy, or plan."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The alphanumeric identifier associated with the insurer issued label."]
    pub r#value: Box<super::super::types::Identifier>,
    #[doc = "A short description for the class."]
    pub r#name: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageClass {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Box::new(super::super::types::Identifier {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#name: Default::default(),
        }
    }
}
#[doc = "A suite of codes indicating exceptions or reductions to patient costs and their effective periods."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageCostToBeneficiaryException {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The code for the specific exception."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The timeframe the exception is in force."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageCostToBeneficiaryException {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#period: Default::default(),
        }
    }
}
#[doc = "A suite of codes indicating the cost category and associated amount which have been detailed in the policy and may have been  included on the health card."]
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageCostToBeneficiary {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The category of patient centric costs associated with treatment."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Is a flag to indicate whether the benefits refer to in-network providers or out-of-network providers."]
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates if the benefits apply to an individual or to the family."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The term or period of the values such as 'maximum lifetime benefit' or 'maximum annual visits'."]
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The amount due from the patient for the cost category."]
    pub r#value: Option<CoverageCostToBeneficiaryValue>,
    #[doc = "A suite of codes indicating exceptions or reductions to patient costs and their effective periods."]
    pub r#exception: Vec<CoverageCostToBeneficiaryException>,
}
#[allow(clippy::derivable_impls)]
impl Default for CoverageCostToBeneficiary {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#category: Default::default(),
            r#network: Default::default(),
            r#unit: Default::default(),
            r#term: Default::default(),
            r#value: Default::default(),
            r#exception: Default::default(),
        }
    }
}
#[doc = "Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.\n\nCoverage provides a link between covered parties (patients) and the payors of their healthcare costs (both insurance and self-pay)."]
#[derive(Debug, Clone, PartialEq)]
pub struct Coverage {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
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
    #[doc = "The identifier of the coverage as issued by the insurer."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The nature of the coverage be it insurance, or cash payment such as self-pay."]
    pub r#kind: super::super::types::Code,
    #[doc = "Link to the paying party and optionally what specifically they will be responsible to pay."]
    pub r#payment_by: Vec<CoveragePaymentBy>,
    #[doc = "The type of coverage: social program, medical plan, accident coverage (workers compensation, auto), group health or payment by an individual or organization."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The party who 'owns' the insurance policy."]
    pub r#policy_holder: Option<Box<super::super::types::Reference>>,
    #[doc = "The party who has signed-up for or 'owns' the contractual relationship to the policy or to whom the benefit of the policy for services rendered to them or their family is due."]
    pub r#subscriber: Option<Box<super::super::types::Reference>>,
    #[doc = "The insurer assigned ID for the Subscriber."]
    pub r#subscriber_id: Vec<super::super::types::Identifier>,
    #[doc = "The party who benefits from the insurance coverage; the patient when products and/or services are provided."]
    pub r#beneficiary: Box<super::super::types::Reference>,
    #[doc = "A designator for a dependent under the coverage."]
    pub r#dependent: Option<super::super::types::String>,
    #[doc = "The relationship of beneficiary (patient) to the subscriber."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Time period during which the coverage is in force. A missing start date indicates the start date isn't known, a missing end date means the coverage is continuing to be in force."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The program or plan underwriter, payor, insurance company."]
    pub r#insurer: Option<Box<super::super::types::Reference>>,
    #[doc = "A suite of underwriter specific classifiers."]
    pub r#class: Vec<CoverageClass>,
    #[doc = "The order of applicability of this coverage relative to other coverages which are currently in force. Note, there may be gaps in the numbering and this does not imply primary, secondary etc. as the specific positioning of coverages depends upon the episode of care. For example; a patient might have (0) auto insurance (1) their own health insurance and (2) spouse's health insurance. When claiming for treatments which were not the result of an auto accident then only coverages (1) and (2) above would be applicatble and would apply in the order specified in parenthesis."]
    pub r#order: Option<super::super::types::PositiveInt>,
    #[doc = "The insurer-specific identifier for the insurer-defined network of providers to which the beneficiary may seek treatment which will be covered at the 'in-network' rate, otherwise 'out of network' terms and conditions apply."]
    pub r#network: Option<super::super::types::String>,
    #[doc = "A suite of codes indicating the cost category and associated amount which have been detailed in the policy and may have been  included on the health card."]
    pub r#cost_to_beneficiary: Vec<CoverageCostToBeneficiary>,
    #[doc = "When 'subrogation=true' this insurance instance has been included not for adjudication but to provide insurers with the details to recover costs."]
    pub r#subrogation: Option<super::super::types::Boolean>,
    #[doc = "The policy(s) which constitute this insurance coverage."]
    pub r#contract: Vec<super::super::types::Reference>,
    #[doc = "The insurance plan details, benefits and costs, which constitute this insurance coverage."]
    pub r#insurance_plan: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Coverage {
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
            r#kind: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#payment_by: Default::default(),
            r#type: Default::default(),
            r#policy_holder: Default::default(),
            r#subscriber: Default::default(),
            r#subscriber_id: Default::default(),
            r#beneficiary: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#dependent: Default::default(),
            r#relationship: Default::default(),
            r#period: Default::default(),
            r#insurer: Default::default(),
            r#class: Default::default(),
            r#order: Default::default(),
            r#network: Default::default(),
            r#cost_to_beneficiary: Default::default(),
            r#subrogation: Default::default(),
            r#contract: Default::default(),
            r#insurance_plan: Default::default(),
        }
    }
}
