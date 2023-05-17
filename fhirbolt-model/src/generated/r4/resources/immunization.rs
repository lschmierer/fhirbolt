// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Date vaccine administered or was to be administered."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImmunizationOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Nominal position in a series."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImmunizationProtocolAppliedDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "The recommended number of doses to achieve immunity."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImmunizationProtocolAppliedSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Indicates who performed the immunization event."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes the type of performance (e.g. ordering provider, administering provider, etc.)."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The practitioner or organization who performed the action."]
    pub r#actor: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImmunizationPerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Educational material presented to the patient (or guardian) at the time of vaccine administration."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationEducation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier of the material presented to the patient."]
    pub r#document_type: Option<super::super::types::String>,
    #[doc = "Reference pointer to the educational material given to the patient if the information was on line."]
    pub r#reference: Option<super::super::types::Uri>,
    #[doc = "Date the educational material was published."]
    pub r#publication_date: Option<super::super::types::DateTime>,
    #[doc = "Date the educational material was given to the patient."]
    pub r#presentation_date: Option<super::super::types::DateTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImmunizationEducation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#document_type: Default::default(),
            r#reference: Default::default(),
            r#publication_date: Default::default(),
            r#presentation_date: Default::default(),
        }
    }
}
#[doc = "Categorical data indicating that an adverse event is associated in time to an immunization."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationReaction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Date of reaction to the immunization."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Details of the reaction."]
    pub r#detail: Option<Box<super::super::types::Reference>>,
    #[doc = "Self-reported indicator."]
    pub r#reported: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImmunizationReaction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#date: Default::default(),
            r#detail: Default::default(),
            r#reported: Default::default(),
        }
    }
}
#[doc = "The protocol (set of recommendations) being followed by the provider who administered the dose."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationProtocolApplied {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "One possible path to achieve presumed immunity against a disease - within the context of an authority."]
    pub r#series: Option<super::super::types::String>,
    #[doc = "Indicates the authority who published the protocol (e.g. ACIP) that is being followed."]
    pub r#authority: Option<Box<super::super::types::Reference>>,
    #[doc = "The vaccine preventable disease the dose is being administered against."]
    pub r#target_disease: Vec<super::super::types::CodeableConcept>,
    #[doc = "Nominal position in a series."]
    pub r#dose_number: ImmunizationProtocolAppliedDoseNumber,
    #[doc = "The recommended number of doses to achieve immunity."]
    pub r#series_doses: Option<ImmunizationProtocolAppliedSeriesDoses>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImmunizationProtocolApplied {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#series: Default::default(),
            r#authority: Default::default(),
            r#target_disease: Default::default(),
            r#dose_number: Default::default(),
            r#series_doses: Default::default(),
        }
    }
}
#[doc = "Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party."]
#[derive(Debug, Clone, PartialEq)]
pub struct Immunization {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique identifier assigned to this immunization record."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Indicates the current status of the immunization event."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the reason the immunization event was not performed."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Vaccine that was administered or was to be administered."]
    pub r#vaccine_code: Box<super::super::types::CodeableConcept>,
    #[doc = "The patient who either received or did not receive the immunization."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The visit or admission or other contact between patient and health care provider the immunization was performed as part of."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Date vaccine administered or was to be administered."]
    pub r#occurrence: ImmunizationOccurrence,
    #[doc = "The date the occurrence of the immunization was first captured in the record - potentially significantly after the occurrence of the event."]
    pub r#recorded: Option<super::super::types::DateTime>,
    #[doc = "An indication that the content of the record is based on information from the person who administered the vaccine. This reflects the context under which the data was originally recorded."]
    pub r#primary_source: Option<super::super::types::Boolean>,
    #[doc = "The source of the data when the report of the immunization event is not based on information from the person who administered the vaccine."]
    pub r#report_origin: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The service delivery location where the vaccine administration occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Name of vaccine manufacturer."]
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    #[doc = "Lot number of the  vaccine product."]
    pub r#lot_number: Option<super::super::types::String>,
    #[doc = "Date vaccine batch expires."]
    pub r#expiration_date: Option<super::super::types::Date>,
    #[doc = "Body site where vaccine was administered."]
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The path by which the vaccine product is taken into the body."]
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quantity of vaccine product that was administered."]
    pub r#dose_quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Indicates who performed the immunization event."]
    pub r#performer: Vec<ImmunizationPerformer>,
    #[doc = "Extra information about the immunization that is not conveyed by the other attributes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Reasons why the vaccine was administered."]
    pub r#reason_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Condition, Observation or DiagnosticReport that supports why the immunization was administered."]
    pub r#reason_reference: Vec<super::super::types::Reference>,
    #[doc = "Indication if a dose is considered to be subpotent. By default, a dose should be considered to be potent."]
    pub r#is_subpotent: Option<super::super::types::Boolean>,
    #[doc = "Reason why a dose is considered to be subpotent."]
    pub r#subpotent_reason: Vec<super::super::types::CodeableConcept>,
    #[doc = "Educational material presented to the patient (or guardian) at the time of vaccine administration."]
    pub r#education: Vec<ImmunizationEducation>,
    #[doc = "Indicates a patient's eligibility for a funding program."]
    pub r#program_eligibility: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates the source of the vaccine actually administered. This may be different than the patient eligibility (e.g. the patient may be eligible for a publically purchased vaccine but due to inventory issues, vaccine purchased with private funds was actually administered)."]
    pub r#funding_source: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Categorical data indicating that an adverse event is associated in time to an immunization."]
    pub r#reaction: Vec<ImmunizationReaction>,
    #[doc = "The protocol (set of recommendations) being followed by the provider who administered the dose."]
    pub r#protocol_applied: Vec<ImmunizationProtocolApplied>,
}
#[allow(clippy::derivable_impls)]
impl Default for Immunization {
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
            r#identifier: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#vaccine_code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#recorded: Default::default(),
            r#primary_source: Default::default(),
            r#report_origin: Default::default(),
            r#location: Default::default(),
            r#manufacturer: Default::default(),
            r#lot_number: Default::default(),
            r#expiration_date: Default::default(),
            r#site: Default::default(),
            r#route: Default::default(),
            r#dose_quantity: Default::default(),
            r#performer: Default::default(),
            r#note: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#is_subpotent: Default::default(),
            r#subpotent_reason: Default::default(),
            r#education: Default::default(),
            r#program_eligibility: Default::default(),
            r#funding_source: Default::default(),
            r#reaction: Default::default(),
            r#protocol_applied: Default::default(),
        }
    }
}
