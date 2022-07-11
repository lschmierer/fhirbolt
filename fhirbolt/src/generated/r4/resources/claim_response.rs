// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ClaimResponseAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum ClaimResponseAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ClaimResponseError {
    pub r#detail_sequence: Option<super::super::types::PositiveInt>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_sequence: Option<super::super::types::PositiveInt>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sub_detail_sequence: Option<super::super::types::PositiveInt>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponsePayment {
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#amount: Box<super::super::types::Money>,
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseAddItemDetailSubDetail {
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#factor: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseAddItemDetail {
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#sub_detail: Vec<ClaimResponseAddItemDetailSubDetail>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseAddItem {
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subdetail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#detail: Vec<ClaimResponseAddItemDetail>,
    pub r#provider: Vec<Box<super::super::types::Reference>>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#serviced: Option<ClaimResponseAddItemServiced>,
    pub r#location: Option<ClaimResponseAddItemLocation>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseProcessNote {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: super::super::types::String,
    pub r#number: Option<super::super::types::PositiveInt>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItemDetailSubDetail {
    pub r#sub_detail_sequence: super::super::types::PositiveInt,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItemDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#detail_sequence: super::super::types::PositiveInt,
    pub r#sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItemAdjudication {
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#value: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItem {
    pub r#detail: Vec<ClaimResponseItemDetail>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_sequence: super::super::types::PositiveInt,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseInsurance {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#focal: super::super::types::Boolean,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
    pub r#business_arrangement: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseTotal {
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#amount: Box<super::super::types::Money>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponse {
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#use: super::super::types::Code,
    pub r#error: Vec<ClaimResponseError>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#payee_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#payment: Option<ClaimResponsePayment>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#form: Option<Box<super::super::types::Attachment>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#disposition: Option<super::super::types::String>,
    pub r#communication_request: Vec<Box<super::super::types::Reference>>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#pre_auth_ref: Option<super::super::types::String>,
    pub r#add_item: Vec<ClaimResponseAddItem>,
    pub r#process_note: Vec<ClaimResponseProcessNote>,
    pub r#created: super::super::types::DateTime,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#outcome: super::super::types::Code,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#pre_auth_period: Option<Box<super::super::types::Period>>,
    pub r#item: Vec<ClaimResponseItem>,
    pub r#insurance: Vec<ClaimResponseInsurance>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#total: Vec<ClaimResponseTotal>,
    pub r#status: super::super::types::Code,
}
