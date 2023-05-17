// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "The period during which the activity occurred."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ProvenanceOccurred {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    #[default]
    Invalid,
}
#[doc = "An actor taking a role in an activity  for which it can be assigned some degree of responsibility for the activity taking place."]
#[derive(Debug, Clone, PartialEq)]
pub struct ProvenanceAgent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The participation the agent had with respect to the activity."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The function of the agent with respect to the activity. The security role enabling the agent with respect to the activity."]
    pub r#role: Vec<super::super::types::CodeableConcept>,
    #[doc = "The individual, device or organization that participated in the event."]
    pub r#who: Box<super::super::types::Reference>,
    #[doc = "The individual, device, or organization for whom the change was made."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ProvenanceAgent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#role: Default::default(),
            r#who: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#on_behalf_of: Default::default(),
        }
    }
}
#[doc = "An entity used in this activity."]
#[derive(Debug, Clone, PartialEq)]
pub struct ProvenanceEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "How the entity was used during the activity."]
    pub r#role: super::super::types::Code,
    #[doc = "Identity of the  Entity used. May be a logical or physical uri and maybe absolute or relative."]
    pub r#what: Box<super::super::types::Reference>,
    #[doc = "The entity is attributed to an agent to express the agent's responsibility for that entity, possibly along with other agents. This description can be understood as shorthand for saying that the agent was responsible for the activity which generated the entity."]
    pub r#agent: Vec<ProvenanceAgent>,
}
#[allow(clippy::derivable_impls)]
impl Default for ProvenanceEntity {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#role: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#what: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#agent: Default::default(),
        }
    }
}
#[doc = "Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for assessing authenticity, enabling trust, and allowing reproducibility. Provenance assertions are a form of contextual metadata and can themselves become important records with their own provenance. Provenance statement indicates clinical significance in terms of confidence in authenticity, reliability, and trustworthiness, integrity, and stage in lifecycle (e.g. Document Completion - has the artifact been legally authenticated), all of which may impact security, privacy, and trust policies."]
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
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
    #[doc = "The Reference(s) that were generated or updated by  the activity described in this resource. A provenance can point to more than one target if multiple resources were created/updated by the same activity."]
    pub r#target: Vec<super::super::types::Reference>,
    #[doc = "The period during which the activity occurred."]
    pub r#occurred: Option<ProvenanceOccurred>,
    #[doc = "The instant of time at which the activity was recorded."]
    pub r#recorded: super::super::types::Instant,
    #[doc = "Policy or plan the activity was defined by. Typically, a single activity may have multiple applicable policy documents, such as patient consent, guarantor funding, etc."]
    pub r#policy: Vec<super::super::types::Uri>,
    #[doc = "Where the activity occurred, if relevant."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason that the activity was taking place."]
    pub r#reason: Vec<super::super::types::CodeableConcept>,
    #[doc = "An activity is something that occurs over a period of time and acts upon or with entities; it may include consuming, processing, transforming, modifying, relocating, using, or generating entities."]
    pub r#activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An actor taking a role in an activity  for which it can be assigned some degree of responsibility for the activity taking place."]
    pub r#agent: Vec<ProvenanceAgent>,
    #[doc = "An entity used in this activity."]
    pub r#entity: Vec<ProvenanceEntity>,
    #[doc = "A digital signature on the target Reference(s). The signer should match a Provenance.agent. The purpose of the signature is indicated."]
    pub r#signature: Vec<super::super::types::Signature>,
}
#[allow(clippy::derivable_impls)]
impl Default for Provenance {
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
            r#occurred: Default::default(),
            r#recorded: super::super::types::Instant {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#policy: Default::default(),
            r#location: Default::default(),
            r#reason: Default::default(),
            r#activity: Default::default(),
            r#agent: Default::default(),
            r#entity: Default::default(),
            r#signature: Default::default(),
        }
    }
}
