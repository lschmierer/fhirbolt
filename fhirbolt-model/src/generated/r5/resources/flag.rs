// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Prospective warnings of potential issues when providing care to the patient."]
#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
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
    #[doc = "Business identifiers assigned to this flag by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Supports basic workflow."]
    pub r#status: super::super::types::Code,
    #[doc = "Allows a flag to be divided into different categories like clinical, administrative etc. Intended to be used as a means of filtering which flags are displayed to particular user or in a given context."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The coded value or textual component of the flag to display to the user."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The patient, related person, location, group, organization, or practitioner etc. this is about record this flag is associated with."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The period of time from the activation of the flag to inactivation of the flag. If the flag is active, the end of the period should be unspecified."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "This alert is only relevant during the encounter."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The person, organization or device that created the flag."]
    pub r#author: Option<Box<super::super::types::Reference>>,
}
impl Default for Flag {
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
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#category: Default::default(),
            r#code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#subject: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: Default::default(),
            r#encounter: Default::default(),
            r#author: Default::default(),
        }
    }
}
