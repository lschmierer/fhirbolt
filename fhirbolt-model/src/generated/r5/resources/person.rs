// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Indicates if the individual is deceased or not."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum PersonDeceased {
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
    #[default]
    Invalid,
}
#[doc = "A language which may be used to communicate with the person about his or her health."]
#[derive(Debug, Clone, PartialEq)]
pub struct PersonCommunication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The ISO-639-1 alpha 2 code in lower case for the language, optionally followed by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g. \"en\" for English, or \"en-US\" for American English versus \"en-AU\" for Australian English."]
    pub r#language: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates whether or not the person prefers this language (over other languages he masters up a certain level)."]
    pub r#preferred: Option<super::super::types::Boolean>,
}
impl Default for PersonCommunication {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#language: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#preferred: Default::default(),
        }
    }
}
#[doc = "Link to a resource that concerns the same actual person."]
#[derive(Debug, Clone, PartialEq)]
pub struct PersonLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The resource to which this actual person is associated."]
    pub r#target: Box<super::super::types::Reference>,
    #[doc = "Level of assurance that this link is associated with the target resource."]
    pub r#assurance: Option<super::super::types::Code>,
}
impl Default for PersonLink {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#target: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#assurance: Default::default(),
        }
    }
}
#[doc = "Demographics and administrative information about a person independent of a specific health-related context.\n\nNeed to track persons potentially across multiple roles."]
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifier for a person within a particular scope."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this person's record is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "A name associated with the person."]
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail for the person, e.g. a telephone number or an email address."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Administrative Gender."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The birth date for the person."]
    pub r#birth_date: Option<super::super::types::Date>,
    #[doc = "Indicates if the individual is deceased or not."]
    pub r#deceased: Option<PersonDeceased>,
    #[doc = "One or more addresses for the person."]
    pub r#address: Vec<Box<super::super::types::Address>>,
    #[doc = "This field contains a person's most recent marital (civil) status."]
    pub r#marital_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An image that can be displayed as a thumbnail of the person to enhance the identification of the individual."]
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    #[doc = "A language which may be used to communicate with the person about his or her health."]
    pub r#communication: Vec<PersonCommunication>,
    #[doc = "The organization that is the custodian of the person record."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Link to a resource that concerns the same actual person."]
    pub r#link: Vec<PersonLink>,
}
impl Default for Person {
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
            r#photo: Default::default(),
            r#communication: Default::default(),
            r#managing_organization: Default::default(),
            r#link: Default::default(),
        }
    }
}
