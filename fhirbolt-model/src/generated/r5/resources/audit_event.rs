// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "The time or period during which the activity occurred."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AuditEventOccurred {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    #[default]
    Invalid,
}
#[doc = "When the event utilizes a network there should be an agent describing the local system, and an agent describing remote system, with the network interface details."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AuditEventAgentNetwork {
    Reference(Box<super::super::types::Reference>),
    Uri(Box<super::super::types::Uri>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "The  value of the extra detail."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AuditEventEntityDetailValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    #[default]
    Invalid,
}
#[doc = "Indicates whether the event succeeded or failed. A free text descripiton can be given in outcome.text."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventOutcome {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates whether the event succeeded or failed."]
    pub r#code: Box<super::super::types::Coding>,
    #[doc = "Additional details about the error. This may be a text description of the error or a system code that identifies the error."]
    pub r#detail: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for AuditEventOutcome {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#detail: Default::default(),
        }
    }
}
#[doc = "An actor taking an active role in the event or activity that is logged."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventAgent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The Functional Role of the user when performing the event."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural roles of the agent indicating the agent's competency. The security role enabling the agent with respect to the activity."]
    pub r#role: Vec<super::super::types::CodeableConcept>,
    #[doc = "Reference to who this agent is that was involved in the event."]
    pub r#who: Box<super::super::types::Reference>,
    #[doc = "Indicator that the user is or is not the requestor, or initiator, for the event being audited."]
    pub r#requestor: Option<super::super::types::Boolean>,
    #[doc = "Where the agent location is known, the agent location when the event occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Where the policy(ies) are known that authorized the agent participation in the event. Typically, a single activity may have multiple applicable policies, such as patient consent, guarantor funding, etc. The policy would also indicate the security token used."]
    pub r#policy: Vec<super::super::types::Uri>,
    #[doc = "When the event utilizes a network there should be an agent describing the local system, and an agent describing remote system, with the network interface details."]
    pub r#network: Option<AuditEventAgentNetwork>,
    #[doc = "The authorization (e.g., PurposeOfUse) that was used during the event being recorded."]
    pub r#authorization: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for AuditEventAgent {
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
            r#requestor: Default::default(),
            r#location: Default::default(),
            r#policy: Default::default(),
            r#network: Default::default(),
            r#authorization: Default::default(),
        }
    }
}
#[doc = "The actor that is reporting the event."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Logical source location within the healthcare enterprise network.  For example, a hospital or other provider location within a multi-entity provider group."]
    pub r#site: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifier of the source where the event was detected."]
    pub r#observer: Box<super::super::types::Reference>,
    #[doc = "Code specifying the type of source where event originated."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for AuditEventSource {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#site: Default::default(),
            r#observer: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#type: Default::default(),
        }
    }
}
#[doc = "Tagged value pairs for conveying additional information about the entity."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventEntityDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of extra detail provided in the value."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The  value of the extra detail."]
    pub r#value: AuditEventEntityDetailValue,
}
#[allow(clippy::derivable_impls)]
impl Default for AuditEventEntityDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
        }
    }
}
#[doc = "Specific instances of data or objects that have been accessed."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies a specific instance of the entity. The reference should be version specific. This is allowed to be a Parameters resource."]
    pub r#what: Option<Box<super::super::types::Reference>>,
    #[doc = "Code representing the role the entity played in the event being audited."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Security labels for the identified entity."]
    pub r#security_label: Vec<super::super::types::CodeableConcept>,
    #[doc = "The query parameters for a query-type entities."]
    pub r#query: Option<super::super::types::Base64Binary>,
    #[doc = "Tagged value pairs for conveying additional information about the entity."]
    pub r#detail: Vec<AuditEventEntityDetail>,
    #[doc = "The entity is attributed to an agent to express the agent's responsibility for that entity in the activity. This is most used to indicate when persistence media (the entity) are used by an agent. For example when importing data from a device, the device would be described in an entity, and the user importing data from that media would be indicated as the entity.agent."]
    pub r#agent: Vec<AuditEventAgent>,
}
#[allow(clippy::derivable_impls)]
impl Default for AuditEventEntity {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#what: Default::default(),
            r#role: Default::default(),
            r#security_label: Default::default(),
            r#query: Default::default(),
            r#detail: Default::default(),
            r#agent: Default::default(),
        }
    }
}
#[doc = "A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEvent {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Classification of the type of event."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Describes what happened. The most specific code for the event."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicator for type of action performed during the event that generated the audit."]
    pub r#action: Option<super::super::types::Code>,
    #[doc = "Indicates and enables segmentation of various severity including debugging from critical."]
    pub r#severity: Option<super::super::types::Code>,
    #[doc = "The time or period during which the activity occurred."]
    pub r#occurred: Option<AuditEventOccurred>,
    #[doc = "The time when the event was recorded."]
    pub r#recorded: super::super::types::Instant,
    #[doc = "Indicates whether the event succeeded or failed. A free text descripiton can be given in outcome.text."]
    pub r#outcome: Option<AuditEventOutcome>,
    #[doc = "The authorization (e.g., PurposeOfUse) that was used during the event being recorded."]
    pub r#authorization: Vec<super::super::types::CodeableConcept>,
    #[doc = "Allows tracing of authorizatino for the events and tracking whether proposals/recommendations were acted upon."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The patient element is available to enable deterministic tracking of activities that involve the patient as the subject of the data used in an activity."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "This will typically be the encounter the event occurred, but some events may be initiated prior to or after the official completion of an encounter but still be tied to the context of the encounter (e.g. pre-admission lab tests)."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "An actor taking an active role in the event or activity that is logged."]
    pub r#agent: Vec<AuditEventAgent>,
    #[doc = "The actor that is reporting the event."]
    pub r#source: AuditEventSource,
    #[doc = "Specific instances of data or objects that have been accessed."]
    pub r#entity: Vec<AuditEventEntity>,
}
#[allow(clippy::derivable_impls)]
impl Default for AuditEvent {
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
            r#category: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#action: Default::default(),
            r#severity: Default::default(),
            r#occurred: Default::default(),
            r#recorded: super::super::types::Instant {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#outcome: Default::default(),
            r#authorization: Default::default(),
            r#based_on: Default::default(),
            r#patient: Default::default(),
            r#encounter: Default::default(),
            r#agent: Default::default(),
            r#source: AuditEventSource {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#entity: Default::default(),
        }
    }
}
