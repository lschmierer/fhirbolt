// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A language which may be used to communicate with the related person about the patient's health."]
#[derive(Debug, Clone, PartialEq)]
pub struct RelatedPersonCommunication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The ISO-639-1 alpha 2 code in lower case for the language, optionally followed by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g. \"en\" for English, or \"en-US\" for American English versus \"en-AU\" for Australian English."]
    pub r#language: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates whether or not the related person prefers this language (over other languages he or she masters up a certain level)."]
    pub r#preferred: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for RelatedPersonCommunication {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#language: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#preferred: Default::default(),
        }
    }
}
#[doc = "Information about a person that is involved in a patient's health or the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.\n\nNeed to track persons related to the patient or the healthcare process."]
#[derive(Debug, Clone, PartialEq)]
pub struct RelatedPerson {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
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
    #[doc = "Identifier for a person within a particular scope."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Whether this related person record is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The patient this person is related to."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The nature of the relationship between the related person and the patient."]
    pub r#relationship: Vec<super::super::types::CodeableConcept>,
    #[doc = "A name associated with the person."]
    pub r#name: Vec<super::super::types::HumanName>,
    #[doc = "A contact detail for the person, e.g. a telephone number or an email address."]
    pub r#telecom: Vec<super::super::types::ContactPoint>,
    #[doc = "Administrative Gender - the gender that the person is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The date on which the related person was born."]
    pub r#birth_date: Option<super::super::types::Date>,
    #[doc = "Address where the related person can be contacted or visited."]
    pub r#address: Vec<super::super::types::Address>,
    #[doc = "Image of the person."]
    pub r#photo: Vec<super::super::types::Attachment>,
    #[doc = "The period of time during which this relationship is or was active. If there are no dates defined, then the interval is unknown."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "A language which may be used to communicate with the related person about the patient's health."]
    pub r#communication: Vec<RelatedPersonCommunication>,
}
#[allow(clippy::derivable_impls)]
impl Default for RelatedPerson {
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
            r#active: Default::default(),
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#relationship: Default::default(),
            r#name: Default::default(),
            r#telecom: Default::default(),
            r#gender: Default::default(),
            r#birth_date: Default::default(),
            r#address: Default::default(),
            r#photo: Default::default(),
            r#period: Default::default(),
            r#communication: Default::default(),
        }
    }
}
