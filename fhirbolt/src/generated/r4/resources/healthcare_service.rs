// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct HealthcareServiceEligibility {
    pub r#comment: Option<super::super::types::Markdown>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct HealthcareServiceAvailableTime {
    pub r#id: Option<std::string::String>,
    pub r#days_of_week: Vec<super::super::types::Code>,
    pub r#all_day: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#available_start_time: Option<super::super::types::Time>,
    pub r#available_end_time: Option<super::super::types::Time>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct HealthcareServiceNotAvailable {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: super::super::types::String,
    pub r#during: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct HealthcareService {
    pub r#photo: Option<Box<super::super::types::Attachment>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#eligibility: Vec<HealthcareServiceEligibility>,
    pub r#available_time: Vec<HealthcareServiceAvailableTime>,
    pub r#appointment_required: Option<super::super::types::Boolean>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extra_details: Option<super::super::types::Markdown>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#program: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#comment: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#provided_by: Option<Box<super::super::types::Reference>>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#referral_method: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#availability_exceptions: Option<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    pub r#not_available: Vec<HealthcareServiceNotAvailable>,
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#service_provision_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Vec<Box<super::super::types::Reference>>,
}
