// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitAccidentLocation {
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitAccident {
    pub r#location: Option<ExplanationOfBenefitAccidentLocation>,
    pub r#id: Option<std::string::String>,
    pub r#date: Option<super::super::types::Date>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitPayment {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitProcessNote {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#text: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#number: Option<super::super::types::PositiveInt>,
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Money(Box<super::super::types::Money>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,
    pub r#used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitBenefitBalance {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#excluded: Option<super::super::types::Boolean>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    pub r#financial: Vec<ExplanationOfBenefitBenefitBalanceFinancial>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitRelated {
    pub r#reference: Option<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#claim: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitDiagnosis {
    pub r#diagnosis: ExplanationOfBenefitDiagnosisDiagnosis,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitItemDetail {
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#id: Option<std::string::String>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#factor: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitItemAdjudication {
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitItem {
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#serviced: Option<ExplanationOfBenefitItemServiced>,
    pub r#detail: Vec<ExplanationOfBenefitItemDetail>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<ExplanationOfBenefitItemLocation>,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#diagnosis_sequence: Vec<super::super::types::PositiveInt>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#information_sequence: Vec<super::super::types::PositiveInt>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#care_team_sequence: Vec<super::super::types::PositiveInt>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#procedure_sequence: Vec<super::super::types::PositiveInt>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitCareTeam {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#qualification: Option<Box<super::super::types::CodeableConcept>>,
    pub r#responsible: Option<super::super::types::Boolean>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#provider: Box<super::super::types::Reference>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
    pub r#id: Option<std::string::String>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitAddItemDetail {
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#sub_detail: Vec<ExplanationOfBenefitAddItemDetailSubDetail>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitAddItem {
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail: Vec<ExplanationOfBenefitAddItemDetail>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#sub_detail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#provider: Vec<Box<super::super::types::Reference>>,
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#serviced: Option<ExplanationOfBenefitAddItemServiced>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#location: Option<ExplanationOfBenefitAddItemLocation>,
    pub r#factor: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitPayee {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#party: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitSupportingInfoTiming {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitSupportingInfoValue {
    Boolean(Box<super::super::types::Boolean>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitSupportingInfo {
    pub r#reason: Option<Box<super::super::types::Coding>>,
    pub r#timing: Option<ExplanationOfBenefitSupportingInfoTiming>,
    pub r#value: Option<ExplanationOfBenefitSupportingInfoValue>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitProcedureProcedure {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitProcedure {
    pub r#id: Option<std::string::String>,
    pub r#procedure: ExplanationOfBenefitProcedureProcedure,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitInsurance {
    pub r#focal: super::super::types::Boolean,
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefitTotal {
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#amount: Box<super::super::types::Money>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ExplanationOfBenefit {
    pub r#accident: Option<ExplanationOfBenefitAccident>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#payment: Option<ExplanationOfBenefitPayment>,
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    pub r#precedence: Option<super::super::types::PositiveInt>,
    pub r#use: super::super::types::Code,
    pub r#provider: Box<super::super::types::Reference>,
    pub r#funds_reserve_requested: Option<Box<super::super::types::CodeableConcept>>,
    pub r#outcome: super::super::types::Code,
    pub r#created: super::super::types::DateTime,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#referral: Option<Box<super::super::types::Reference>>,
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#process_note: Vec<ExplanationOfBenefitProcessNote>,
    pub r#benefit_balance: Vec<ExplanationOfBenefitBenefitBalance>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#related: Vec<ExplanationOfBenefitRelated>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#diagnosis: Vec<ExplanationOfBenefitDiagnosis>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    pub r#item: Vec<ExplanationOfBenefitItem>,
    pub r#care_team: Vec<ExplanationOfBenefitCareTeam>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#add_item: Vec<ExplanationOfBenefitAddItem>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#payee: Option<ExplanationOfBenefitPayee>,
    pub r#claim: Option<Box<super::super::types::Reference>>,
    pub r#disposition: Option<super::super::types::String>,
    pub r#supporting_info: Vec<ExplanationOfBenefitSupportingInfo>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#procedure: Vec<ExplanationOfBenefitProcedure>,
    pub r#insurance: Vec<ExplanationOfBenefitInsurance>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#prescription: Option<Box<super::super::types::Reference>>,
    pub r#pre_auth_ref_period: Vec<Box<super::super::types::Period>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#form: Option<Box<super::super::types::Attachment>>,
    pub r#billable_period: Option<Box<super::super::types::Period>>,
    pub r#original_prescription: Option<Box<super::super::types::Reference>>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#total: Vec<ExplanationOfBenefitTotal>,
}
