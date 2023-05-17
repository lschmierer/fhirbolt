// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "Identifies all people and organizations who are expected to be involved in the care team."]
#[derive(Debug, Clone, PartialEq)]
pub struct CareTeamParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates specific responsibility of an individual within the care team, such as \"Primary care physician\", \"Trained social worker counselor\", \"Caregiver\", etc."]
    pub r#role: Vec<super::super::types::CodeableConcept>,
    #[doc = "The specific person or organization who is participating/expected to participate in the care team."]
    pub r#member: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization of the practitioner."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the specific member or organization did (or is intended to) come into effect and end."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for CareTeamParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#role: Default::default(),
            r#member: Default::default(),
            r#on_behalf_of: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "The Care Team includes all the people and organizations who plan to participate in the coordination and delivery of care for a patient."]
#[derive(Debug, Clone, PartialEq)]
pub struct CareTeam {
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
    #[doc = "Business identifiers assigned to this care team by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Indicates the current state of the care team."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Identifies what kind of team.  This is to support differentiation between multiple co-existing teams, such as care plan team, episode of care team, longitudinal care team."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "A label for human use intended to distinguish like teams.  E.g. the \"red\" vs. \"green\" trauma teams."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Identifies the patient or group whose intended care is handled by the team."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The Encounter during which this CareTeam was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the team did (or is intended to) come into effect and end."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Identifies all people and organizations who are expected to be involved in the care team."]
    pub r#participant: Vec<CareTeamParticipant>,
    #[doc = "Describes why the care team exists."]
    pub r#reason_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Condition(s) that this care team addresses."]
    pub r#reason_reference: Vec<super::super::types::Reference>,
    #[doc = "The organization responsible for the care team."]
    pub r#managing_organization: Vec<super::super::types::Reference>,
    #[doc = "A central contact detail for the care team (that applies to all members)."]
    pub r#telecom: Vec<super::super::types::ContactPoint>,
    #[doc = "Comments made about the CareTeam."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for CareTeam {
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
            r#status: Default::default(),
            r#category: Default::default(),
            r#name: Default::default(),
            r#subject: Default::default(),
            r#encounter: Default::default(),
            r#period: Default::default(),
            r#participant: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#managing_organization: Default::default(),
            r#telecom: Default::default(),
            r#note: Default::default(),
        }
    }
}
