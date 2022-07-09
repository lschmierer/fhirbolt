// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum QuestionnaireItemInitialValue {
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
pub struct QuestionnaireItemInitial {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: QuestionnaireItemInitialValue,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum QuestionnaireItemAnswerOptionValue {
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct QuestionnaireItemAnswerOption {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: QuestionnaireItemAnswerOptionValue,
    pub r#initial_selected: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub enum QuestionnaireItemEnableWhenAnswer {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct QuestionnaireItemEnableWhen {
    pub r#id: Option<std::string::String>,
    pub r#answer: QuestionnaireItemEnableWhenAnswer,
    pub r#question: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#operator: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct QuestionnaireItem {
    pub r#required: Option<super::super::types::Boolean>,
    pub r#max_length: Option<super::super::types::Integer>,
    pub r#initial: Vec<QuestionnaireItemInitial>,
    pub r#answer_option: Vec<QuestionnaireItemAnswerOption>,
    pub r#item: Vec<QuestionnaireItem>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#definition: Option<super::super::types::Uri>,
    pub r#text: Option<super::super::types::String>,
    pub r#type: super::super::types::Code,
    pub r#code: Vec<Box<super::super::types::Coding>>,
    pub r#link_id: super::super::types::String,
    pub r#answer_value_set: Option<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#enable_when: Vec<QuestionnaireItemEnableWhen>,
    pub r#prefix: Option<super::super::types::String>,
    pub r#repeats: Option<super::super::types::Boolean>,
    pub r#enable_behavior: Option<super::super::types::Code>,
    pub r#read_only: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct Questionnaire {
    pub r#language: Option<super::super::types::Code>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#name: Option<super::super::types::String>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#version: Option<super::super::types::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#title: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#subject_type: Vec<super::super::types::Code>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#item: Vec<QuestionnaireItem>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#code: Vec<Box<super::super::types::Coding>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#derived_from: Vec<super::super::types::Canonical>,
    pub r#purpose: Option<super::super::types::Markdown>,
}
