// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum QuestionnaireResponseItemAnswerValue {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Attachment(Box<super::super::types::Attachment>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct QuestionnaireResponseItemAnswer {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item: Vec<QuestionnaireResponseItem>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<QuestionnaireResponseItemAnswerValue>,
}
#[derive(Debug, Clone)]
pub struct QuestionnaireResponseItem {
    pub r#answer: Vec<QuestionnaireResponseItemAnswer>,
    pub r#text: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#link_id: super::super::types::String,
    pub r#definition: Option<super::super::types::Uri>,
    pub r#item: Vec<QuestionnaireResponseItem>,
}
#[derive(Debug, Clone)]
pub struct QuestionnaireResponse {
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#authored: Option<super::super::types::DateTime>,
    pub r#item: Vec<QuestionnaireResponseItem>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#questionnaire: Option<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
}
