// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct PaymentReconciliationDetail {
    pub r#response: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#date: Option<super::super::types::Date>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#payee: Option<Box<super::super::types::Reference>>,
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#predecessor: Option<Box<super::super::types::Identifier>>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#submitter: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct PaymentReconciliationProcessNote {
    pub r#id: Option<std::string::String>,
    pub r#text: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PaymentReconciliation {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#created: super::super::types::DateTime,
    pub r#outcome: Option<super::super::types::Code>,
    pub r#detail: Vec<PaymentReconciliationDetail>,
    pub r#process_note: Vec<PaymentReconciliationProcessNote>,
    pub r#payment_date: super::super::types::Date,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#payment_amount: Box<super::super::types::Money>,
    pub r#payment_issuer: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#payment_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#status: super::super::types::Code,
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#disposition: Option<super::super::types::String>,
}
