// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ClaimResponseInsurance {
    pub r#focal: super::super::types::Boolean,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#business_arrangement: Option<super::super::types::String>,
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseError {
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail_sequence: Option<super::super::types::PositiveInt>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#sub_detail_sequence: Option<super::super::types::PositiveInt>,
    pub r#item_sequence: Option<super::super::types::PositiveInt>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponsePayment {
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Box<super::super::types::Money>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum ClaimResponseAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ClaimResponseAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct ClaimResponseAddItemDetailSubDetail {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseAddItemDetail {
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#sub_detail: Vec<ClaimResponseAddItemDetailSubDetail>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseAddItem {
    pub r#location: Option<ClaimResponseAddItemLocation>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    pub r#subdetail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#serviced: Option<ClaimResponseAddItemServiced>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#detail: Vec<ClaimResponseAddItemDetail>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#provider: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseProcessNote {
    pub r#id: Option<std::string::String>,
    pub r#number: Option<super::super::types::PositiveInt>,
    pub r#type: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: super::super::types::String,
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItemAdjudication {
    pub r#value: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<Box<super::super::types::Money>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItemDetailSubDetail {
    pub r#sub_detail_sequence: super::super::types::PositiveInt,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItemDetail {
    pub r#detail_sequence: super::super::types::PositiveInt,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseItem {
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#item_sequence: super::super::types::PositiveInt,
    pub r#detail: Vec<ClaimResponseItemDetail>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponseTotal {
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Box<super::super::types::Money>,
}
#[derive(Debug, Clone)]
pub struct ClaimResponse {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#insurance: Vec<ClaimResponseInsurance>,
    pub r#status: super::super::types::Code,
    pub r#error: Vec<ClaimResponseError>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#payment: Option<ClaimResponsePayment>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#created: super::super::types::DateTime,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#add_item: Vec<ClaimResponseAddItem>,
    pub r#outcome: super::super::types::Code,
    pub r#use: super::super::types::Code,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    pub r#process_note: Vec<ClaimResponseProcessNote>,
    pub r#communication_request: Vec<Box<super::super::types::Reference>>,
    pub r#pre_auth_ref: Option<super::super::types::String>,
    pub r#pre_auth_period: Option<Box<super::super::types::Period>>,
    pub r#payee_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#disposition: Option<super::super::types::String>,
    pub r#item: Vec<ClaimResponseItem>,
    pub r#total: Vec<ClaimResponseTotal>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#form: Option<Box<super::super::types::Attachment>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
