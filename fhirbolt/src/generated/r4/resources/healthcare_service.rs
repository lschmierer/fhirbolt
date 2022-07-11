// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct HealthcareServiceNotAvailable {
    pub r#during: Option<Box<super::super::types::Period>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: super::super::types::String,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct HealthcareServiceEligibility {
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#comment: Option<super::super::types::Markdown>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct HealthcareServiceAvailableTime {
    pub r#available_end_time: Option<super::super::types::Time>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#days_of_week: Vec<super::super::types::Code>,
    pub r#all_day: Option<super::super::types::Boolean>,
    pub r#available_start_time: Option<super::super::types::Time>,
}
#[derive(Debug, Clone)]
pub struct HealthcareService {
    pub r#referral_method: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#availability_exceptions: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#location: Vec<Box<super::super::types::Reference>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#program: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#photo: Option<Box<super::super::types::Attachment>>,
    pub r#not_available: Vec<HealthcareServiceNotAvailable>,
    pub r#name: Option<super::super::types::String>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#eligibility: Vec<HealthcareServiceEligibility>,
    pub r#provided_by: Option<Box<super::super::types::Reference>>,
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#available_time: Vec<HealthcareServiceAvailableTime>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#service_provision_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#appointment_required: Option<super::super::types::Boolean>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extra_details: Option<super::super::types::Markdown>,
}
