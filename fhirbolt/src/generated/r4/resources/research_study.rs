// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ResearchStudyArm {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#name: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ResearchStudyObjective {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ResearchStudy {
    pub r#title: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reason_stopped: Option<Box<super::super::types::CodeableConcept>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#keyword: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#sponsor: Option<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#arm: Vec<ResearchStudyArm>,
    pub r#protocol: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#phase: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#objective: Vec<ResearchStudyObjective>,
    pub r#enrollment: Vec<Box<super::super::types::Reference>>,
    pub r#principal_investigator: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#location: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#site: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#primary_purpose_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#condition: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#focus: Vec<Box<super::super::types::CodeableConcept>>,
}
