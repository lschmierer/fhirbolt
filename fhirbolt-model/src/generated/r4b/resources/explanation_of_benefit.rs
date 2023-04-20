// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "The date when or period to which this information refers."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitSupportingInfoTiming {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ExplanationOfBenefitSupportingInfoTiming {
    fn default() -> ExplanationOfBenefitSupportingInfoTiming {
        ExplanationOfBenefitSupportingInfoTiming::Invalid
    }
}
#[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitSupportingInfoValue {
    Boolean(Box<super::super::types::Boolean>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitSupportingInfoValue {
    fn default() -> ExplanationOfBenefitSupportingInfoValue {
        ExplanationOfBenefitSupportingInfoValue::Invalid
    }
}
#[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitDiagnosisDiagnosis {
    fn default() -> ExplanationOfBenefitDiagnosisDiagnosis {
        ExplanationOfBenefitDiagnosisDiagnosis::Invalid
    }
}
#[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitProcedureProcedure {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitProcedureProcedure {
    fn default() -> ExplanationOfBenefitProcedureProcedure {
        ExplanationOfBenefitProcedureProcedure::Invalid
    }
}
#[doc = "The physical location of the accident event."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitAccidentLocation {
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitAccidentLocation {
    fn default() -> ExplanationOfBenefitAccidentLocation {
        ExplanationOfBenefitAccidentLocation::Invalid
    }
}
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ExplanationOfBenefitItemServiced {
    fn default() -> ExplanationOfBenefitItemServiced {
        ExplanationOfBenefitItemServiced::Invalid
    }
}
#[doc = "Where the product or service was provided."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitItemLocation {
    fn default() -> ExplanationOfBenefitItemLocation {
        ExplanationOfBenefitItemLocation::Invalid
    }
}
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ExplanationOfBenefitAddItemServiced {
    fn default() -> ExplanationOfBenefitAddItemServiced {
        ExplanationOfBenefitAddItemServiced::Invalid
    }
}
#[doc = "Where the product or service was provided."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitAddItemLocation {
    fn default() -> ExplanationOfBenefitAddItemLocation {
        ExplanationOfBenefitAddItemLocation::Invalid
    }
}
#[doc = "The quantity of the benefit which is permitted under the coverage."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
    Invalid,
}
impl Default for ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    fn default() -> ExplanationOfBenefitBenefitBalanceFinancialAllowed {
        ExplanationOfBenefitBenefitBalanceFinancialAllowed::Invalid
    }
}
#[doc = "The quantity of the benefit which have been consumed to date."]
#[derive(Debug, Clone, PartialEq)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Money(Box<super::super::types::Money>),
    Invalid,
}
impl Default for ExplanationOfBenefitBenefitBalanceFinancialUsed {
    fn default() -> ExplanationOfBenefitBenefitBalanceFinancialUsed {
        ExplanationOfBenefitBenefitBalanceFinancialUsed::Invalid
    }
}
#[doc = "Other claims which are related to this claim such as prior submissions or claims for related services or for the same event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitRelated {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to a related claim."]
    pub r#claim: Option<Box<super::super::types::Reference>>,
    #[doc = "A code to convey how the claims are related."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An alternate organizational reference to the case or file to which this particular claim pertains."]
    pub r#reference: Option<Box<super::super::types::Identifier>>,
}
#[doc = "The party to be reimbursed for cost of the products and services according to the terms of the policy."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitPayee {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of Party to be reimbursed: Subscriber, provider, other."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reference to the individual or organization to whom any payment will be made."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
#[doc = "The members of the team who provided the products and services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitCareTeam {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
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
#[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
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
#[doc = "Information about diagnoses relevant to the claim items."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify diagnosis entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
    pub r#diagnosis: ExplanationOfBenefitDiagnosisDiagnosis,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indication of whether the diagnosis was present on admission to a facility."]
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A package billing code or bundle code used to group products and services to a particular health condition (such as heart attack) which is based on a predetermined grouping code system."]
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Procedures performed on the patient relevant to the billing items with the claim."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitProcedure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify procedure entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date and optionally time the procedure was performed."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
    pub r#procedure: ExplanationOfBenefitProcedureProcedure,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
}
#[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A flag to indicate that this Coverage is to be used for adjudication of this claim when set to true."]
    pub r#focal: super::super::types::Boolean,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "Reference numbers previously provided by the insurer to the provider to be quoted on subsequent claims containing services or products related to the prior authorization."]
    pub r#pre_auth_ref: Vec<super::super::types::String>,
}
#[doc = "Details of a accident which resulted in injuries which required the products and services listed in the claim."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAccident {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Date of an accident event  related to the products and services contained in the claim."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "The type or context of the accident event for the purposes of selection of potential insurance coverages and determination of coordination between insurers."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The physical location of the accident event."]
    pub r#location: Option<ExplanationOfBenefitAccidentLocation>,
}
#[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItemAdjudication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include: the value submitted, maximum values or percentages allowed or payable under the plan, amounts that the patient is responsible for in-aggregate or pertaining to this item, amounts paid by other coverages, and the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "A code supporting the understanding of the adjudication result and explaining variance from expected amount."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Monetary amount associated with the category."]
    pub r#amount: Option<Box<super::super::types::Money>>,
    #[doc = "A non-monetary value associated with the category. Mutually exclusive to the amount element above."]
    pub r#value: Option<super::super::types::Decimal>,
}
#[doc = "Third-tier of goods and services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
#[doc = "Second-tier of goods and services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Third-tier of goods and services."]
    pub r#sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
}
#[doc = "A claim line. Either a simple (a product or service) or a 'group' of details which can also be a simple items or groups of sub-details."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
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
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
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
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "Physical service site on the patient (limb, tooth, etc.)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A region or surface of the bodySite, e.g. limb region or tooth surface(s)."]
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A billed item may include goods or services provided in multiple encounters."]
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Second-tier of goods and services."]
    pub r#detail: Vec<ExplanationOfBenefitItemDetail>,
}
#[doc = "The third-tier service adjudications for payor added services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
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
#[doc = "The second-tier service adjudications for payor added services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAddItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
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
#[doc = "The first-tier service adjudications for payor added product or service lines."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitAddItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Claim items which this service line is intended to replace."]
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the details within the claim item which this line is intended to replace."]
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the sub-details woithin the details within the claim item which this line is intended to replace."]
    pub r#sub_detail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The providers who are authorized for the services rendered to the patient."]
    pub r#provider: Vec<Box<super::super::types::Reference>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
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
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "The second-tier service adjudications for payor added services."]
    pub r#detail: Vec<ExplanationOfBenefitAddItemDetail>,
}
#[doc = "Categorized monetary totals for the adjudication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitTotal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include: the value submitted, maximum values or percentages allowed or payable under the plan, amounts that the patient is responsible for in aggregate or pertaining to this item, amounts paid by other coverages, and the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "Monetary total amount associated with the category."]
    pub r#amount: Box<super::super::types::Money>,
}
#[doc = "Payment details for the adjudication of the claim."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitPayment {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
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
#[doc = "A note that describes or explains adjudication results in a human readable form."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitProcessNote {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify a note entry."]
    pub r#number: Option<super::super::types::PositiveInt>,
    #[doc = "The business purpose of the note text."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The explanation or description associated with the processing."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "A code to define the language used in the text of the note."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Benefits Used to date."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Classification of benefit being provided."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The quantity of the benefit which is permitted under the coverage."]
    pub r#allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,
    #[doc = "The quantity of the benefit which have been consumed to date."]
    pub r#used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
}
#[doc = "Balance by Benefit Category."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefitBenefitBalance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
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
#[doc = "This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplanationOfBenefit {
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
    #[doc = "A unique identifier assigned to this explanation of benefit."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
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
    pub r#pre_auth_ref_period: Vec<Box<super::super::types::Period>>,
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
