// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum RequestGroupActionRelatedActionOffset {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
}
#[derive(Debug, Clone)]
pub struct RequestGroupActionRelatedAction {
    pub r#action_id: super::super::types::Id,
    pub r#relationship: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#offset: Option<RequestGroupActionRelatedActionOffset>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum RequestGroupActionTiming {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub struct RequestGroupActionCondition {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#expression: Option<Box<super::super::types::Expression>>,
    pub r#kind: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct RequestGroupAction {
    pub r#participant: Vec<Box<super::super::types::Reference>>,
    pub r#action: Vec<RequestGroupAction>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#related_action: Vec<RequestGroupActionRelatedAction>,
    pub r#required_behavior: Option<super::super::types::Code>,
    pub r#cardinality_behavior: Option<super::super::types::Code>,
    pub r#text_equivalent: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#selection_behavior: Option<super::super::types::Code>,
    pub r#precheck_behavior: Option<super::super::types::Code>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#prefix: Option<super::super::types::String>,
    pub r#grouping_behavior: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#resource: Option<Box<super::super::types::Reference>>,
    pub r#title: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#timing: Option<RequestGroupActionTiming>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#condition: Vec<RequestGroupActionCondition>,
}
#[derive(Debug, Clone)]
pub struct RequestGroup {
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#language: Option<super::super::types::Code>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#intent: super::super::types::Code,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#status: super::super::types::Code,
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#action: Vec<RequestGroupAction>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
}
