// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Indicates if the individual is deceased or not."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum PatientDeceased {
    Boolean(super::super::types::Boolean),
    DateTime(super::super::types::DateTime),
    #[default]
    Invalid,
}
#[doc = "Indicates whether the patient is part of a multiple (boolean) or indicates the actual birth order (integer)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum PatientMultipleBirth {
    Boolean(super::super::types::Boolean),
    Integer(super::super::types::Integer),
    #[default]
    Invalid,
}
#[doc = "A contact party (e.g. guardian, partner, friend) for the patient."]
#[derive(Debug, Clone, PartialEq)]
pub struct PatientContact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The nature of the relationship between the patient and the contact person."]
    pub r#relationship: Vec<super::super::types::CodeableConcept>,
    #[doc = "A name associated with the contact person."]
    pub r#name: Option<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail for the person, e.g. a telephone number or an email address."]
    pub r#telecom: Vec<super::super::types::ContactPoint>,
    #[doc = "Address for the contact person."]
    pub r#address: Option<Box<super::super::types::Address>>,
    #[doc = "Administrative Gender - the gender that the contact person is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "Organization on behalf of which the contact is acting or for which the contact is working."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The period during which this contact person or organization is valid to be contacted relating to this patient."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for PatientContact {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#relationship: Default::default(),
            r#name: Default::default(),
            r#telecom: Default::default(),
            r#address: Default::default(),
            r#gender: Default::default(),
            r#organization: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "A language which may be used to communicate with the patient about his or her health."]
#[derive(Debug, Clone, PartialEq)]
pub struct PatientCommunication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The ISO-639-1 alpha 2 code in lower case for the language, optionally followed by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g. \"en\" for English, or \"en-US\" for American English versus \"en-EN\" for England English."]
    pub r#language: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates whether or not the patient prefers this language (over other languages he masters up a certain level)."]
    pub r#preferred: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for PatientCommunication {
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
#[doc = "Link to another patient resource that concerns the same actual patient."]
#[derive(Debug, Clone, PartialEq)]
pub struct PatientLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The other patient resource that the link refers to."]
    pub r#other: Box<super::super::types::Reference>,
    #[doc = "The type of link between this patient resource and another patient resource."]
    pub r#type: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for PatientLink {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#other: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Demographics and other administrative information about an individual or animal receiving care or other health-related services.\n\nTracking patient is the center of the healthcare process."]
#[derive(Debug, Clone, PartialEq)]
pub struct Patient {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier for this patient."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Whether this patient record is in active use. \nMany systems use this property to mark as non-current patients, such as those that have not been seen for a period of time based on an organization's business rules.\n\nIt is often used to filter patient lists to exclude inactive patients\n\nDeceased patients may also be marked as inactive for the same reasons, but may be active for some time after death."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "A name associated with the individual."]
    pub r#name: Vec<super::super::types::HumanName>,
    #[doc = "A contact detail (e.g. a telephone number or an email address) by which the individual may be contacted."]
    pub r#telecom: Vec<super::super::types::ContactPoint>,
    #[doc = "Administrative Gender - the gender that the patient is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The date of birth for the individual."]
    pub r#birth_date: Option<super::super::types::Date>,
    #[doc = "Indicates if the individual is deceased or not."]
    pub r#deceased: Option<PatientDeceased>,
    #[doc = "An address for the individual."]
    pub r#address: Vec<super::super::types::Address>,
    #[doc = "This field contains a patient's most recent marital (civil) status."]
    pub r#marital_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates whether the patient is part of a multiple (boolean) or indicates the actual birth order (integer)."]
    pub r#multiple_birth: Option<PatientMultipleBirth>,
    #[doc = "Image of the patient."]
    pub r#photo: Vec<super::super::types::Attachment>,
    #[doc = "A contact party (e.g. guardian, partner, friend) for the patient."]
    pub r#contact: Vec<PatientContact>,
    #[doc = "A language which may be used to communicate with the patient about his or her health."]
    pub r#communication: Vec<PatientCommunication>,
    #[doc = "Patient's nominated care provider."]
    pub r#general_practitioner: Vec<super::super::types::Reference>,
    #[doc = "Organization that is the custodian of the patient record."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Link to another patient resource that concerns the same actual patient."]
    pub r#link: Vec<PatientLink>,
}
#[allow(clippy::derivable_impls)]
impl Default for Patient {
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
            r#name: Default::default(),
            r#telecom: Default::default(),
            r#gender: Default::default(),
            r#birth_date: Default::default(),
            r#deceased: Default::default(),
            r#address: Default::default(),
            r#marital_status: Default::default(),
            r#multiple_birth: Default::default(),
            r#photo: Default::default(),
            r#contact: Default::default(),
            r#communication: Default::default(),
            r#general_practitioner: Default::default(),
            r#managing_organization: Default::default(),
            r#link: Default::default(),
        }
    }
}
