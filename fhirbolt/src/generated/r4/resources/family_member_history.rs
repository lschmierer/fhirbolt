// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum FamilyMemberHistoryConditionOnset {
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Period(Box<super::super::types::Period>),
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
pub struct FamilyMemberHistoryCondition {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contributed_to_death: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#onset: Option<FamilyMemberHistoryConditionOnset>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
#[derive(Debug, Clone)]
pub struct FamilyMemberHistory {
    pub r#estimated_age: Option<super::super::types::Boolean>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#status: super::super::types::Code,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#condition: Vec<FamilyMemberHistoryCondition>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#sex: Option<Box<super::super::types::CodeableConcept>>,
    pub r#deceased: Option<FamilyMemberHistoryDeceased>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#relationship: Box<super::super::types::CodeableConcept>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#age: Option<FamilyMemberHistoryAge>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#born: Option<FamilyMemberHistoryBorn>,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
}
