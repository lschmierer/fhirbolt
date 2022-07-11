// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum GoalTargetDue {
    Date(Box<super::super::types::Date>),
    Duration(Box<super::super::types::Duration>),
}
#[derive(Debug, Clone)]
pub enum GoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Ratio(Box<super::super::types::Ratio>),
}
#[derive(Debug, Clone)]
pub enum GoalStart {
    Date(Box<super::super::types::Date>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct GoalTarget {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#due: Option<GoalTargetDue>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#detail: Option<GoalTargetDetail>,
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Goal {
    pub r#language: Option<super::super::types::Code>,
    pub r#target: Vec<GoalTarget>,
    pub r#status_reason: Option<super::super::types::String>,
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    pub r#description: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#achievement_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#outcome_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#status_date: Option<super::super::types::Date>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
    pub r#expressed_by: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#start: Option<GoalStart>,
    pub r#lifecycle_status: super::super::types::Code,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
}
