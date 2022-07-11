// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct VerificationResultPrimarySource {
    pub r#validation_date: Option<super::super::types::DateTime>,
    pub r#push_type_available: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#can_push_updates: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#validation_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#who: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#communication_method: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct VerificationResultAttestation {
    pub r#date: Option<super::super::types::Date>,
    pub r#proxy_signature: Option<Box<super::super::types::Signature>>,
    pub r#communication_method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#proxy_identity_certificate: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#who: Option<Box<super::super::types::Reference>>,
    pub r#source_signature: Option<Box<super::super::types::Signature>>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#source_identity_certificate: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct VerificationResultValidator {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identity_certificate: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#attestation_signature: Option<Box<super::super::types::Signature>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#organization: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#last_performed: Option<super::super::types::DateTime>,
    pub r#next_scheduled: Option<super::super::types::Date>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#target: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#validation_process: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#frequency: Option<Box<super::super::types::Timing>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status_date: Option<super::super::types::DateTime>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#need: Option<Box<super::super::types::CodeableConcept>>,
    pub r#failure_action: Option<Box<super::super::types::CodeableConcept>>,
    pub r#primary_source: Vec<VerificationResultPrimarySource>,
    pub r#attestation: Option<VerificationResultAttestation>,
    pub r#validation_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#validator: Vec<VerificationResultValidator>,
    pub r#target_location: Vec<super::super::types::String>,
}
