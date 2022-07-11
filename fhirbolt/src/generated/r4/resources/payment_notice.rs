// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct PaymentNotice {
    pub r#payee: Option<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#amount: Box<super::super::types::Money>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#created: super::super::types::DateTime,
    pub r#payment_date: Option<super::super::types::Date>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#response: Option<Box<super::super::types::Reference>>,
    pub r#recipient: Box<super::super::types::Reference>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#payment_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#payment: Box<super::super::types::Reference>,
}
