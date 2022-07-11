// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct PractitionerRoleAvailableTime {
    pub r#all_day: Option<super::super::types::Boolean>,
    pub r#available_end_time: Option<super::super::types::Time>,
    pub r#available_start_time: Option<super::super::types::Time>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#days_of_week: Vec<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct PractitionerRoleNotAvailable {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: super::super::types::String,
    pub r#during: Option<Box<super::super::types::Period>>,
}
#[derive(Debug, Clone)]
pub struct PractitionerRole {
    pub r#active: Option<super::super::types::Boolean>,
    pub r#available_time: Vec<PractitionerRoleAvailableTime>,
    pub r#practitioner: Option<Box<super::super::types::Reference>>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#organization: Option<Box<super::super::types::Reference>>,
    pub r#not_available: Vec<PractitionerRoleNotAvailable>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#availability_exceptions: Option<super::super::types::String>,
    pub r#location: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
