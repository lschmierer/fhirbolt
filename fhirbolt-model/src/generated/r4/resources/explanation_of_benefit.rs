// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The date when or period to which this information refers."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitSupportingInfoTiming {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitSupportingInfoValue {
    Boolean(Box<super::super::types::Boolean>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitProcedureProcedure {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The physical location of the accident event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitAccidentLocation {
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Where the product or service was provided."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Where the product or service was provided."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The quantity of the benefit which is permitted under the coverage."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
    #[default]
    Invalid,
}
#[doc = "The quantity of the benefit which have been consumed to date."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Money(Box<super::super::types::Money>),
    #[default]
    Invalid,
}
#[doc = "Other claims which are related to this claim such as prior submissions or claims for related services or for the same event."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitRelated {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Reference to a related claim."]
    pub r#claim: Option<Box<super::super::types::Reference>>,
    #[doc = "A code to convey how the claims are related."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An alternate organizational reference to the case or file to which this particular claim pertains."]
    pub r#reference: Option<Box<super::super::types::Identifier>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitRelated {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#claim: Default::default(),
            r#relationship: Default::default(),
            r#reference: Default::default(),
        }
    }
}
#[doc = "The party to be reimbursed for cost of the products and services according to the terms of the policy."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitPayee {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Type of Party to be reimbursed: Subscriber, provider, other."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reference to the individual or organization to whom any payment will be made."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitPayee {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#party: Default::default(),
        }
    }
}
#[doc = "The members of the team who provided the products and services."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitCareTeam {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A number to uniquely identify care team entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "Member of the team who provided the product or service."]
    pub r#provider: Box<super::super::types::Reference>,
    #[doc = "The party who is billing and/or responsible for the claimed products or services."]
    pub r#responsible: Option<super::super::types::Boolean>,
    #[doc = "The lead, assisting or supervising practitioner and their discipline if a multidisciplinary team."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The qualification of the practitioner which is applicable for this service."]
    pub r#qualification: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitCareTeam {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#provider: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#responsible: Default::default(),
            r#role: Default::default(),
            r#qualification: Default::default(),
        }
    }
}
#[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A number to uniquely identify supporting information entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The general class of the information supplied: information; exception; accident, employment; onset, etc."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "System and code pertaining to the specific information regarding special conditions relating to the setting, treatment or patient  for which care is sought."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date when or period to which this information refers."]
    pub r#timing: Option<ExplanationOfBenefitSupportingInfoTiming>,
    #[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
    pub r#value: Option<ExplanationOfBenefitSupportingInfoValue>,
    #[doc = "Provides the reason in the situation where a reason code is required in addition to the content."]
    pub r#reason: Option<Box<super::super::types::Coding>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitSupportingInfo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#code: Default::default(),
            r#timing: Default::default(),
            r#value: Default::default(),
            r#reason: Default::default(),
        }
    }
}
#[doc = "Information about diagnoses relevant to the claim items."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A number to uniquely identify diagnosis entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
    pub r#diagnosis: ExplanationOfBenefitDiagnosisDiagnosis,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indication of whether the diagnosis was present on admission to a facility."]
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A package billing code or bundle code used to group products and services to a particular health condition (such as heart attack) which is based on a predetermined grouping code system."]
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitDiagnosis {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#diagnosis: Default::default(),
            r#type: Default::default(),
            r#on_admission: Default::default(),
            r#package_code: Default::default(),
        }
    }
}
#[doc = "Procedures performed on the patient relevant to the billing items with the claim."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitProcedure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A number to uniquely identify procedure entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Date and optionally time the procedure was performed."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
    pub r#procedure: ExplanationOfBenefitProcedureProcedure,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitProcedure {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Default::default(),
            r#date: Default::default(),
            r#procedure: Default::default(),
            r#udi: Default::default(),
        }
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A flag to indicate that this Coverage is to be used for adjudication of this claim when set to true."]
    pub r#focal: super::super::types::Boolean,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "Reference numbers previously provided by the insurer to the provider to be quoted on subsequent claims containing services or products related to the prior authorization."]
    pub r#pre_auth_ref: Vec<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitInsurance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#focal: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#coverage: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#pre_auth_ref: Default::default(),
        }
    }
}
#[doc = "Details of a accident which resulted in injuries which required the products and services listed in the claim."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAccident {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Date of an accident event  related to the products and services contained in the claim."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "The type or context of the accident event for the purposes of selection of potential insurance coverages and determination of coordination between insurers."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The physical location of the accident event."]
    pub r#location: Option<ExplanationOfBenefitAccidentLocation>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitAccident {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#date: Default::default(),
            r#type: Default::default(),
            r#location: Default::default(),
        }
    }
}
#[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItemAdjudication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include: the value submitted, maximum values or percentages allowed or payable under the plan, amounts that the patient is responsible for in-aggregate or pertaining to this item, amounts paid by other coverages, and the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "A code supporting the understanding of the adjudication result and explaining variance from expected amount."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Monetary amount associated with the category."]
    pub r#amount: Option<Box<super::super::types::Money>>,
    #[doc = "A non-monetary value associated with the category. Mutually exclusive to the amount element above."]
    pub r#value: Option<super::super::types::Decimal>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitItemAdjudication {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#reason: Default::default(),
            r#amount: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "Third-tier of goods and services."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<super::super::types::Reference>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitItemDetailSubDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#revenue: Default::default(),
            r#category: Default::default(),
            r#product_or_service: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#modifier: Default::default(),
            r#program_code: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#factor: Default::default(),
            r#net: Default::default(),
            r#udi: Default::default(),
            r#note_number: Default::default(),
            r#adjudication: Default::default(),
        }
    }
}
#[doc = "Second-tier of goods and services."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<super::super::types::Reference>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Third-tier of goods and services."]
    pub r#sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitItemDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#revenue: Default::default(),
            r#category: Default::default(),
            r#product_or_service: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#modifier: Default::default(),
            r#program_code: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#factor: Default::default(),
            r#net: Default::default(),
            r#udi: Default::default(),
            r#note_number: Default::default(),
            r#adjudication: Default::default(),
            r#sub_detail: Default::default(),
        }
    }
}
#[doc = "A claim line. Either a simple (a product or service) or a 'group' of details which can also be a simple items or groups of sub-details."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A number to uniquely identify item entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "Care team members related to this service or product."]
    pub r#care_team_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Diagnoses applicable for this service or product."]
    pub r#diagnosis_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Procedures applicable for this service or product."]
    pub r#procedure_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Exceptions, special conditions and supporting information applicable for this service or product."]
    pub r#information_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "The date or dates when the service or product was supplied, performed or completed."]
    pub r#serviced: Option<ExplanationOfBenefitItemServiced>,
    #[doc = "Where the product or service was provided."]
    pub r#location: Option<ExplanationOfBenefitItemLocation>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<super::super::types::Reference>,
    #[doc = "Physical service site on the patient (limb, tooth, etc.)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A region or surface of the bodySite, e.g. limb region or tooth surface(s)."]
    pub r#sub_site: Vec<super::super::types::CodeableConcept>,
    #[doc = "A billed item may include goods or services provided in multiple encounters."]
    pub r#encounter: Vec<super::super::types::Reference>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Second-tier of goods and services."]
    pub r#detail: Vec<ExplanationOfBenefitItemDetail>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#care_team_sequence: Default::default(),
            r#diagnosis_sequence: Default::default(),
            r#procedure_sequence: Default::default(),
            r#information_sequence: Default::default(),
            r#revenue: Default::default(),
            r#category: Default::default(),
            r#product_or_service: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#modifier: Default::default(),
            r#program_code: Default::default(),
            r#serviced: Default::default(),
            r#location: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#factor: Default::default(),
            r#net: Default::default(),
            r#udi: Default::default(),
            r#body_site: Default::default(),
            r#sub_site: Default::default(),
            r#encounter: Default::default(),
            r#note_number: Default::default(),
            r#adjudication: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "The third-tier service adjudications for payor added services."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitAddItemDetailSubDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#product_or_service: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#modifier: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#factor: Default::default(),
            r#net: Default::default(),
            r#note_number: Default::default(),
            r#adjudication: Default::default(),
        }
    }
}
#[doc = "The second-tier service adjudications for payor added services."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAddItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "The third-tier service adjudications for payor added services."]
    pub r#sub_detail: Vec<ExplanationOfBenefitAddItemDetailSubDetail>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitAddItemDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#product_or_service: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#modifier: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#factor: Default::default(),
            r#net: Default::default(),
            r#note_number: Default::default(),
            r#adjudication: Default::default(),
            r#sub_detail: Default::default(),
        }
    }
}
#[doc = "The first-tier service adjudications for payor added product or service lines."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAddItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Claim items which this service line is intended to replace."]
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the details within the claim item which this line is intended to replace."]
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the sub-details woithin the details within the claim item which this line is intended to replace."]
    pub r#sub_detail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The providers who are authorized for the services rendered to the patient."]
    pub r#provider: Vec<super::super::types::Reference>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "The date or dates when the service or product was supplied, performed or completed."]
    pub r#serviced: Option<ExplanationOfBenefitAddItemServiced>,
    #[doc = "Where the product or service was provided."]
    pub r#location: Option<ExplanationOfBenefitAddItemLocation>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Physical service site on the patient (limb, tooth, etc.)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A region or surface of the bodySite, e.g. limb region or tooth surface(s)."]
    pub r#sub_site: Vec<super::super::types::CodeableConcept>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "The second-tier service adjudications for payor added services."]
    pub r#detail: Vec<ExplanationOfBenefitAddItemDetail>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitAddItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item_sequence: Default::default(),
            r#detail_sequence: Default::default(),
            r#sub_detail_sequence: Default::default(),
            r#provider: Default::default(),
            r#product_or_service: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#modifier: Default::default(),
            r#program_code: Default::default(),
            r#serviced: Default::default(),
            r#location: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#factor: Default::default(),
            r#net: Default::default(),
            r#body_site: Default::default(),
            r#sub_site: Default::default(),
            r#note_number: Default::default(),
            r#adjudication: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "Categorized monetary totals for the adjudication."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitTotal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include: the value submitted, maximum values or percentages allowed or payable under the plan, amounts that the patient is responsible for in aggregate or pertaining to this item, amounts paid by other coverages, and the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "Monetary total amount associated with the category."]
    pub r#amount: Box<super::super::types::Money>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitTotal {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#amount: Box::new(super::super::types::Money {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Payment details for the adjudication of the claim."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitPayment {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Whether this represents partial or complete payment of the benefits payable."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Total amount of all adjustments to this payment included in this transaction which are not related to this claim's adjudication."]
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    #[doc = "Reason for the payment adjustment."]
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Estimated date the payment will be issued or the actual issue date of payment."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "Benefits payable less any payment adjustment."]
    pub r#amount: Option<Box<super::super::types::Money>>,
    #[doc = "Issuer's unique identifier for the payment instrument."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitPayment {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#adjustment: Default::default(),
            r#adjustment_reason: Default::default(),
            r#date: Default::default(),
            r#amount: Default::default(),
            r#identifier: Default::default(),
        }
    }
}
#[doc = "A note that describes or explains adjudication results in a human readable form."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitProcessNote {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A number to uniquely identify a note entry."]
    pub r#number: Option<super::super::types::PositiveInt>,
    #[doc = "The business purpose of the note text."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The explanation or description associated with the processing."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "A code to define the language used in the text of the note."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitProcessNote {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#number: Default::default(),
            r#type: Default::default(),
            r#text: Default::default(),
            r#language: Default::default(),
        }
    }
}
#[doc = "Benefits Used to date."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Classification of benefit being provided."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The quantity of the benefit which is permitted under the coverage."]
    pub r#allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,
    #[doc = "The quantity of the benefit which have been consumed to date."]
    pub r#used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitBenefitBalanceFinancial {
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
#[doc = "Balance by Benefit Category."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitBenefitBalance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Box<super::super::types::CodeableConcept>,
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
    #[doc = "Benefits Used to date."]
    pub r#financial: Vec<ExplanationOfBenefitBenefitBalanceFinancial>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefitBenefitBalance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#excluded: Default::default(),
            r#name: Default::default(),
            r#description: Default::default(),
            r#network: Default::default(),
            r#unit: Default::default(),
            r#term: Default::default(),
            r#financial: Default::default(),
        }
    }
}
#[doc = "This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefit {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique identifier assigned to this explanation of benefit."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The category of claim, e.g. oral, pharmacy, vision, institutional, professional."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A finer grained suite of claim type codes which may convey additional information such as Inpatient vs Outpatient and/or a specialty service."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate whether the nature of the request is: to request adjudication of products and services previously rendered; or requesting authorization and adjudication for provision in the future; or requesting the non-binding adjudication of the listed products and services which could be provided in the future."]
    pub r#use: super::super::types::Code,
    #[doc = "The party to whom the professional services and/or products have been supplied or are being considered and for whom actual for forecast reimbursement is sought."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The period for which charges are being submitted."]
    pub r#billable_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "Individual who created the claim, predetermination or preauthorization."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "The party responsible for authorization, adjudication and reimbursement."]
    pub r#insurer: Box<super::super::types::Reference>,
    #[doc = "The provider which is responsible for the claim, predetermination or preauthorization."]
    pub r#provider: Box<super::super::types::Reference>,
    #[doc = "The provider-required urgency of processing the request. Typical values include: stat, routine deferred."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate whether and for whom funds are to be reserved for future claims."]
    pub r#funds_reserve_requested: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code, used only on a response to a preauthorization, to indicate whether the benefits payable have been reserved and for whom."]
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Other claims which are related to this claim such as prior submissions or claims for related services or for the same event."]
    pub r#related: Vec<ExplanationOfBenefitRelated>,
    #[doc = "Prescription to support the dispensing of pharmacy, device or vision products."]
    pub r#prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "Original prescription which has been superseded by this prescription to support the dispensing of pharmacy services, medications or products."]
    pub r#original_prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "The party to be reimbursed for cost of the products and services according to the terms of the policy."]
    pub r#payee: Option<ExplanationOfBenefitPayee>,
    #[doc = "A reference to a referral resource."]
    pub r#referral: Option<Box<super::super::types::Reference>>,
    #[doc = "Facility where the services were provided."]
    pub r#facility: Option<Box<super::super::types::Reference>>,
    #[doc = "The business identifier for the instance of the adjudication request: claim predetermination or preauthorization."]
    pub r#claim: Option<Box<super::super::types::Reference>>,
    #[doc = "The business identifier for the instance of the adjudication response: claim, predetermination or preauthorization response."]
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
    #[doc = "The outcome of the claim, predetermination, or preauthorization processing."]
    pub r#outcome: super::super::types::Code,
    #[doc = "A human readable description of the status of the adjudication."]
    pub r#disposition: Option<super::super::types::String>,
    #[doc = "Reference from the Insurer which is used in later communications which refers to this adjudication."]
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    #[doc = "The timeframe during which the supplied preauthorization reference may be quoted on claims to obtain the adjudication as provided."]
    pub r#pre_auth_ref_period: Vec<super::super::types::Period>,
    #[doc = "The members of the team who provided the products and services."]
    pub r#care_team: Vec<ExplanationOfBenefitCareTeam>,
    #[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
    pub r#supporting_info: Vec<ExplanationOfBenefitSupportingInfo>,
    #[doc = "Information about diagnoses relevant to the claim items."]
    pub r#diagnosis: Vec<ExplanationOfBenefitDiagnosis>,
    #[doc = "Procedures performed on the patient relevant to the billing items with the claim."]
    pub r#procedure: Vec<ExplanationOfBenefitProcedure>,
    #[doc = "This indicates the relative order of a series of EOBs related to different coverages for the same suite of services."]
    pub r#precedence: Option<super::super::types::PositiveInt>,
    #[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
    pub r#insurance: Vec<ExplanationOfBenefitInsurance>,
    #[doc = "Details of a accident which resulted in injuries which required the products and services listed in the claim."]
    pub r#accident: Option<ExplanationOfBenefitAccident>,
    #[doc = "A claim line. Either a simple (a product or service) or a 'group' of details which can also be a simple items or groups of sub-details."]
    pub r#item: Vec<ExplanationOfBenefitItem>,
    #[doc = "The first-tier service adjudications for payor added product or service lines."]
    pub r#add_item: Vec<ExplanationOfBenefitAddItem>,
    #[doc = "The adjudication results which are presented at the header level rather than at the line-item or add-item levels."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Categorized monetary totals for the adjudication."]
    pub r#total: Vec<ExplanationOfBenefitTotal>,
    #[doc = "Payment details for the adjudication of the claim."]
    pub r#payment: Option<ExplanationOfBenefitPayment>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The actual form, by reference or inclusion, for printing the content or an EOB."]
    pub r#form: Option<Box<super::super::types::Attachment>>,
    #[doc = "A note that describes or explains adjudication results in a human readable form."]
    pub r#process_note: Vec<ExplanationOfBenefitProcessNote>,
    #[doc = "The term of the benefits documented in this response."]
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    #[doc = "Balance by Benefit Category."]
    pub r#benefit_balance: Vec<ExplanationOfBenefitBenefitBalance>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExplanationOfBenefit {
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
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#sub_type: Default::default(),
            r#use: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#billable_period: Default::default(),
            r#created: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#enterer: Default::default(),
            r#insurer: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#provider: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#priority: Default::default(),
            r#funds_reserve_requested: Default::default(),
            r#funds_reserve: Default::default(),
            r#related: Default::default(),
            r#prescription: Default::default(),
            r#original_prescription: Default::default(),
            r#payee: Default::default(),
            r#referral: Default::default(),
            r#facility: Default::default(),
            r#claim: Default::default(),
            r#claim_response: Default::default(),
            r#outcome: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#disposition: Default::default(),
            r#pre_auth_ref: Default::default(),
            r#pre_auth_ref_period: Default::default(),
            r#care_team: Default::default(),
            r#supporting_info: Default::default(),
            r#diagnosis: Default::default(),
            r#procedure: Default::default(),
            r#precedence: Default::default(),
            r#insurance: Default::default(),
            r#accident: Default::default(),
            r#item: Default::default(),
            r#add_item: Default::default(),
            r#adjudication: Default::default(),
            r#total: Default::default(),
            r#payment: Default::default(),
            r#form_code: Default::default(),
            r#form: Default::default(),
            r#process_note: Default::default(),
            r#benefit_period: Default::default(),
            r#benefit_balance: Default::default(),
        }
    }
}
