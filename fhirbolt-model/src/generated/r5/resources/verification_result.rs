// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Information about the primary source(s) involved in validation."]
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResultPrimarySource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Reference to the primary source."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of primary source (License Board; Primary Education; Continuing Education; Postal Service; Relationship owner; Registration Authority; legal source; issuing source; authoritative source)."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Method for communicating with the primary source (manual; API; Push)."]
    pub r#communication_method: Vec<super::super::types::CodeableConcept>,
    #[doc = "Status of the validation of the target against the primary source (successful; failed; unknown)."]
    pub r#validation_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the target was validated against the primary source."]
    pub r#validation_date: Option<super::super::types::DateTime>,
    #[doc = "Ability of the primary source to push updates/alerts (yes; no; undetermined)."]
    pub r#can_push_updates: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of alerts/updates the primary source can send (specific requested changes; any changes; as defined by source)."]
    pub r#push_type_available: Vec<super::super::types::CodeableConcept>,
}
impl Default for VerificationResultPrimarySource {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#who: Default::default(),
            r#type: Default::default(),
            r#communication_method: Default::default(),
            r#validation_status: Default::default(),
            r#validation_date: Default::default(),
            r#can_push_updates: Default::default(),
            r#push_type_available: Default::default(),
        }
    }
}
#[doc = "Information about the entity attesting to information."]
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResultAttestation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The individual or organization attesting to information."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "When the who is asserting on behalf of another (organization or individual)."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "The method by which attested information was submitted/retrieved (manual; API; Push)."]
    pub r#communication_method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date the information was attested to."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "A digital identity certificate associated with the attestation source."]
    pub r#source_identity_certificate: Option<super::super::types::String>,
    #[doc = "A digital identity certificate associated with the proxy entity submitting attested information on behalf of the attestation source."]
    pub r#proxy_identity_certificate: Option<super::super::types::String>,
    #[doc = "Signed assertion by the proxy entity indicating that they have the right to submit attested information on behalf of the attestation source."]
    pub r#proxy_signature: Option<Box<super::super::types::Signature>>,
    #[doc = "Signed assertion by the attestation source that they have attested to the information."]
    pub r#source_signature: Option<Box<super::super::types::Signature>>,
}
impl Default for VerificationResultAttestation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#who: Default::default(),
            r#on_behalf_of: Default::default(),
            r#communication_method: Default::default(),
            r#date: Default::default(),
            r#source_identity_certificate: Default::default(),
            r#proxy_identity_certificate: Default::default(),
            r#proxy_signature: Default::default(),
            r#source_signature: Default::default(),
        }
    }
}
#[doc = "Information about the entity validating information."]
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResultValidator {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Reference to the organization validating information."]
    pub r#organization: Box<super::super::types::Reference>,
    #[doc = "A digital identity certificate associated with the validator."]
    pub r#identity_certificate: Option<super::super::types::String>,
    #[doc = "Signed assertion by the validator that they have validated the information."]
    pub r#attestation_signature: Option<Box<super::super::types::Signature>>,
}
impl Default for VerificationResultValidator {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#organization: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#identity_certificate: Default::default(),
            r#attestation_signature: Default::default(),
        }
    }
}
#[doc = "Describes validation requirements, source(s), status and dates for one or more elements."]
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResult {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A resource that was validated."]
    pub r#target: Vec<super::super::types::Reference>,
    #[doc = "The fhirpath location(s) within the resource that was validated."]
    pub r#target_location: Vec<super::super::types::String>,
    #[doc = "The frequency with which the target must be validated (none; initial; periodic)."]
    pub r#need: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The validation status of the target (attested; validated; in process; requires revalidation; validation failed; revalidation failed)."]
    pub r#status: super::super::types::Code,
    #[doc = "When the validation status was updated."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "What the target is validated against (nothing; primary source; multiple sources)."]
    pub r#validation_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The primary process by which the target is validated (edit check; value set; primary source; multiple sources; standalone; in context)."]
    pub r#validation_process: Vec<super::super::types::CodeableConcept>,
    #[doc = "Frequency of revalidation."]
    pub r#frequency: Option<Box<super::super::types::Timing>>,
    #[doc = "The date/time validation was last completed (including failed validations)."]
    pub r#last_performed: Option<super::super::types::DateTime>,
    #[doc = "The date when target is next validated, if appropriate."]
    pub r#next_scheduled: Option<super::super::types::Date>,
    #[doc = "The result if validation fails (fatal; warning; record only; none)."]
    pub r#failure_action: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information about the primary source(s) involved in validation."]
    pub r#primary_source: Vec<VerificationResultPrimarySource>,
    #[doc = "Information about the entity attesting to information."]
    pub r#attestation: Option<VerificationResultAttestation>,
    #[doc = "Information about the entity validating information."]
    pub r#validator: Vec<VerificationResultValidator>,
}
impl Default for VerificationResult {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#target: Default::default(),
            r#target_location: Default::default(),
            r#need: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status_date: Default::default(),
            r#validation_type: Default::default(),
            r#validation_process: Default::default(),
            r#frequency: Default::default(),
            r#last_performed: Default::default(),
            r#next_scheduled: Default::default(),
            r#failure_action: Default::default(),
            r#primary_source: Default::default(),
            r#attestation: Default::default(),
            r#validator: Default::default(),
        }
    }
}
