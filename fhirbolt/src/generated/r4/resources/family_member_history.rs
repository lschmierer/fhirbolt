// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryConditionOnset {
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct FamilyMemberHistoryCondition {
    pub r#id: Option<std::string::String>,
    pub r#onset: Option<FamilyMemberHistoryConditionOnset>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contributed_to_death: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryAge {
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryBorn {
    Period(Box<super::super::types::Period>),
    Date(Box<super::super::types::Date>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryDeceased {
    Boolean(Box<super::super::types::Boolean>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Date(Box<super::super::types::Date>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct FamilyMemberHistory {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#estimated_age: Option<super::super::types::Boolean>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#relationship: Box<super::super::types::CodeableConcept>,
    pub r#status: super::super::types::Code,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#condition: Vec<FamilyMemberHistoryCondition>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#age: Option<FamilyMemberHistoryAge>,
    pub r#born: Option<FamilyMemberHistoryBorn>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#name: Option<super::super::types::String>,
    pub r#deceased: Option<FamilyMemberHistoryDeceased>,
    pub r#sex: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#language: Option<super::super::types::Code>,
}
