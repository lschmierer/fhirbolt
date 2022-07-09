// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ClaimSupportingInfoTiming {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum ClaimSupportingInfoValue {
    Boolean(Box<super::super::types::Boolean>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ClaimSupportingInfo {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#timing: Option<ClaimSupportingInfoTiming>,
    pub r#value: Option<ClaimSupportingInfoValue>,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub enum ClaimProcedureProcedure {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ClaimProcedure {
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#procedure: ClaimProcedureProcedure,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#id: Option<std::string::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimCareTeam {
    pub r#provider: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#qualification: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#responsible: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub enum ClaimAccidentLocation {
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ClaimAccident {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#date: super::super::types::Date,
    pub r#location: Option<ClaimAccidentLocation>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ClaimRelated {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Option<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    pub r#claim: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub enum ClaimDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ClaimDiagnosis {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#diagnosis: ClaimDiagnosisDiagnosis,
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#id: Option<std::string::String>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ClaimPayee {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#party: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimInsurance {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#focal: super::super::types::Boolean,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#business_arrangement: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    pub r#sequence: super::super::types::PositiveInt,
}
#[derive(Debug, Clone)]
pub enum ClaimItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct ClaimItemDetailSubDetail {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ClaimItemDetail {
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#sub_detail: Vec<ClaimItemDetailSubDetail>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum ClaimItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ClaimItem {
    pub r#serviced: Option<ClaimItemServiced>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#procedure_sequence: Vec<super::super::types::PositiveInt>,
    pub r#detail: Vec<ClaimItemDetail>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#diagnosis_sequence: Vec<super::super::types::PositiveInt>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#care_team_sequence: Vec<super::super::types::PositiveInt>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#information_sequence: Vec<super::super::types::PositiveInt>,
    pub r#location: Option<ClaimItemLocation>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Claim {
    pub r#language: Option<super::super::types::Code>,
    pub r#prescription: Option<Box<super::super::types::Reference>>,
    pub r#use: super::super::types::Code,
    pub r#supporting_info: Vec<ClaimSupportingInfo>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#referral: Option<Box<super::super::types::Reference>>,
    pub r#procedure: Vec<ClaimProcedure>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#provider: Box<super::super::types::Reference>,
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    pub r#care_team: Vec<ClaimCareTeam>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#priority: Box<super::super::types::CodeableConcept>,
    pub r#accident: Option<ClaimAccident>,
    pub r#related: Vec<ClaimRelated>,
    pub r#insurer: Option<Box<super::super::types::Reference>>,
    pub r#created: super::super::types::DateTime,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
    pub r#diagnosis: Vec<ClaimDiagnosis>,
    pub r#payee: Option<ClaimPayee>,
    pub r#insurance: Vec<ClaimInsurance>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#item: Vec<ClaimItem>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#original_prescription: Option<Box<super::super::types::Reference>>,
    pub r#total: Option<Box<super::super::types::Money>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#billable_period: Option<Box<super::super::types::Period>>,
}
