// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct PractitionerRoleAvailableTime {
    pub r#days_of_week: Vec<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#available_end_time: Option<super::super::types::Time>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#all_day: Option<super::super::types::Boolean>,
    pub r#available_start_time: Option<super::super::types::Time>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PractitionerRoleNotAvailable {
    pub r#during: Option<Box<super::super::types::Period>>,
    pub r#description: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PractitionerRole {
    pub r#location: Vec<Box<super::super::types::Reference>>,
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#organization: Option<Box<super::super::types::Reference>>,
    pub r#practitioner: Option<Box<super::super::types::Reference>>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#available_time: Vec<PractitionerRoleAvailableTime>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#not_available: Vec<PractitionerRoleNotAvailable>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#availability_exceptions: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
}
