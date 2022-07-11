// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum InvoiceLineItemChargeItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct InvoiceLineItemPriceComponent {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#amount: Option<Box<super::super::types::Money>>,
}
#[derive(Debug, Clone)]
pub struct InvoiceLineItem {
    pub r#price_component: Vec<InvoiceLineItemPriceComponent>,
    pub r#sequence: Option<super::super::types::PositiveInt>,
    pub r#charge_item: InvoiceLineItemChargeItem,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct InvoiceParticipant {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Invoice {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#total_net: Option<Box<super::super::types::Money>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#status: super::super::types::Code,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#account: Option<Box<super::super::types::Reference>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#line_item: Vec<InvoiceLineItem>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#total_gross: Option<Box<super::super::types::Money>>,
    pub r#payment_terms: Option<super::super::types::Markdown>,
    pub r#issuer: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#cancelled_reason: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#participant: Vec<InvoiceParticipant>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    pub r#total_price_component: Vec<InvoiceLineItemPriceComponent>,
}
