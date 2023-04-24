// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Date vaccine administered or was to be administered."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImmunizationOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Indicates who performed the immunization event."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the type of performance (e.g. ordering provider, administering provider, etc.)."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The practitioner or organization who performed the action."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl Default for ImmunizationPerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "Indicates a patient's eligibility for a funding program."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationProgramEligibility {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates which program the patient had their eligility evaluated for."]
    pub r#program: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates the patient's eligility status for for a specific payment program."]
    pub r#program_status: Box<super::super::types::CodeableConcept>,
}
impl Default for ImmunizationProgramEligibility {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#program: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#program_status: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "Categorical data indicating that an adverse event is associated in time to an immunization."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationReaction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Date of reaction to the immunization."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Details of the reaction."]
    pub r#manifestation: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Self-reported indicator."]
    pub r#reported: Option<super::super::types::Boolean>,
}
impl Default for ImmunizationReaction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#date: Default::default(),
            r#manifestation: Default::default(),
            r#reported: Default::default(),
        }
    }
}
#[doc = "The protocol (set of recommendations) being followed by the provider who administered the dose."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationProtocolApplied {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "One possible path to achieve presumed immunity against a disease - within the context of an authority."]
    pub r#series: Option<super::super::types::String>,
    #[doc = "Indicates the authority who published the protocol (e.g. ACIP) that is being followed."]
    pub r#authority: Option<Box<super::super::types::Reference>>,
    #[doc = "The vaccine preventable disease the dose is being administered against."]
    pub r#target_disease: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Nominal position in a series as intended by the practitioner administering the dose."]
    pub r#dose_number: super::super::types::String,
    #[doc = "The recommended number of doses to achieve immunity as intended by the practitioner administering the dose."]
    pub r#series_doses: Option<super::super::types::String>,
}
impl Default for ImmunizationProtocolApplied {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#series: Default::default(),
            r#authority: Default::default(),
            r#target_disease: Default::default(),
            r#dose_number: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#series_doses: Default::default(),
        }
    }
}
#[doc = "Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party."]
#[derive(Debug, Clone, PartialEq)]
pub struct Immunization {
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
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A unique identifier assigned to this immunization record."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A plan, order or recommendation fulfilled in whole or in part by this immunization."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates the current status of the immunization event."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the reason the immunization event was not performed."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Vaccine that was administered or was to be administered."]
    pub r#vaccine_code: Box<super::super::types::CodeableConcept>,
    #[doc = "An indication of which product was administered to the patient. This is typically a more detailed representation of the concept conveyed by the vaccineCode data element. If a Medication resource is referenced, it may be to a stand-alone resource or a contained resource within the Immunization resource."]
    pub r#administered_product: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Name of vaccine manufacturer."]
    pub r#manufacturer: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Lot number of the  vaccine product."]
    pub r#lot_number: Option<super::super::types::String>,
    #[doc = "Date vaccine batch expires."]
    pub r#expiration_date: Option<super::super::types::Date>,
    #[doc = "The patient who either received or did not receive the immunization."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The visit or admission or other contact between patient and health care provider the immunization was performed as part of."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional information that is relevant to the immunization (e.g. for a vaccine recipient who is pregnant, the gestational age of the fetus). The reason why a vaccine was given (e.g. occupation, underlying medical condition) should be conveyed in Immunization.reason, not as supporting information. The reason why a vaccine was not given (e.g. contraindication) should be conveyed in Immunization.statusReason, not as supporting information."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    #[doc = "Date vaccine administered or was to be administered."]
    pub r#occurrence: ImmunizationOccurrence,
    #[doc = "Indicates whether the data contained in the resource was captured by the individual/organization which was responsible for the administration of the vaccine rather than as 'secondary reported' data documented by a third party. A value of 'true' means this data originated with the individual/organization which was responsible for the administration of the vaccine."]
    pub r#primary_source: Option<super::super::types::Boolean>,
    #[doc = "Typically the source of the data when the report of the immunization event is not based on information from the person who administered the vaccine."]
    pub r#information_source: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The service delivery location where the vaccine administration occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Body site where vaccine was administered."]
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The path by which the vaccine product is taken into the body."]
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quantity of vaccine product that was administered."]
    pub r#dose_quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Indicates who performed the immunization event."]
    pub r#performer: Vec<ImmunizationPerformer>,
    #[doc = "Extra information about the immunization that is not conveyed by the other attributes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Describes why the immunization occurred in coded or textual form, or Indicates another resource (Condition, Observation or DiagnosticReport) whose existence justifies this immunization."]
    pub r#reason: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Indication if a dose is considered to be subpotent. By default, a dose should be considered to be potent."]
    pub r#is_subpotent: Option<super::super::types::Boolean>,
    #[doc = "Reason why a dose is considered to be subpotent."]
    pub r#subpotent_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates a patient's eligibility for a funding program."]
    pub r#program_eligibility: Vec<ImmunizationProgramEligibility>,
    #[doc = "Indicates the source of the vaccine actually administered. This may be different than the patient eligibility (e.g. the patient may be eligible for a publically purchased vaccine but due to inventory issues, vaccine purchased with private funds was actually administered)."]
    pub r#funding_source: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Categorical data indicating that an adverse event is associated in time to an immunization."]
    pub r#reaction: Vec<ImmunizationReaction>,
    #[doc = "The protocol (set of recommendations) being followed by the provider who administered the dose."]
    pub r#protocol_applied: Vec<ImmunizationProtocolApplied>,
}
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
            r#based_on: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status_reason: Default::default(),
            r#vaccine_code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#administered_product: Default::default(),
            r#manufacturer: Default::default(),
            r#lot_number: Default::default(),
            r#expiration_date: Default::default(),
            r#patient: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#encounter: Default::default(),
            r#supporting_information: Default::default(),
            r#occurrence: Default::default(),
            r#primary_source: Default::default(),
            r#information_source: Default::default(),
            r#location: Default::default(),
            r#site: Default::default(),
            r#route: Default::default(),
            r#dose_quantity: Default::default(),
            r#performer: Default::default(),
            r#note: Default::default(),
            r#reason: Default::default(),
            r#is_subpotent: Default::default(),
            r#subpotent_reason: Default::default(),
            r#program_eligibility: Default::default(),
            r#funding_source: Default::default(),
            r#reaction: Default::default(),
            r#protocol_applied: Default::default(),
        }
    }
}
