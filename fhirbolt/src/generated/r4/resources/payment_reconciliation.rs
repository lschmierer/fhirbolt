// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct PaymentReconciliationProcessNote {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct PaymentReconciliationDetail {
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#submitter: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#id: Option<std::string::String>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    pub r#response: Option<Box<super::super::types::Reference>>,
    pub r#payee: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#predecessor: Option<Box<super::super::types::Identifier>>,
    pub r#amount: Option<Box<super::super::types::Money>>,
}
#[derive(Debug, Clone)]
pub struct PaymentReconciliation {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#created: super::super::types::DateTime,
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    pub r#payment_issuer: Option<Box<super::super::types::Reference>>,
    pub r#payment_amount: Box<super::super::types::Money>,
    pub r#disposition: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#process_note: Vec<PaymentReconciliationProcessNote>,
    pub r#payment_date: super::super::types::Date,
    pub r#detail: Vec<PaymentReconciliationDetail>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#payment_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#outcome: Option<super::super::types::Code>,
}
