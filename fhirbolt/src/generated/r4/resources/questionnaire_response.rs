// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
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
    pub r#item: Vec<QuestionnaireResponseItem>,
    pub r#value: Option<QuestionnaireResponseItemAnswerValue>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct QuestionnaireResponseItem {
    pub r#definition: Option<super::super::types::Uri>,
    pub r#item: Vec<QuestionnaireResponseItem>,
    pub r#id: Option<std::string::String>,
    pub r#link_id: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#answer: Vec<QuestionnaireResponseItemAnswer>,
}
#[derive(Debug, Clone)]
pub struct QuestionnaireResponse {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#item: Vec<QuestionnaireResponseItem>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#authored: Option<super::super::types::DateTime>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#questionnaire: Option<super::super::types::Canonical>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
}
