// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The  value of the extra detail."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AuditEventEntityDetailValue {
    String(Box<super::super::types::String>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    #[default]
    Invalid,
}
#[doc = "Logical network location for application activity, if the activity has a network location."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventAgentNetwork {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier for the network access point of the user device for the audit event."]
    pub r#address: Option<super::super::types::String>,
    #[doc = "An identifier for the type of network access point that originated the audit event."]
    pub r#type: Option<super::super::types::Code>,
}
impl Default for AuditEventAgentNetwork {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#address: Default::default(),
            r#type: Default::default(),
        }
    }
}
#[doc = "An actor taking an active role in the event or activity that is logged."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventAgent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Specification of the participation type the user plays when performing the event."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The security role that the user was acting under, that come from local codes defined by the access control security system (e.g. RBAC, ABAC) used in the local context."]
    pub r#role: Vec<super::super::types::CodeableConcept>,
    #[doc = "Reference to who this agent is that was involved in the event."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "Alternative agent Identifier. For a human, this should be a user identifier text string from authentication system. This identifier would be one known to a common authentication system (e.g. single sign-on), if available."]
    pub r#alt_id: Option<super::super::types::String>,
    #[doc = "Human-meaningful name for the agent."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Indicator that the user is or is not the requestor, or initiator, for the event being audited."]
    pub r#requestor: super::super::types::Boolean,
    #[doc = "Where the event occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The policy or plan that authorized the activity being recorded. Typically, a single activity may have multiple applicable policies, such as patient consent, guarantor funding, etc. The policy would also indicate the security token used."]
    pub r#policy: Vec<super::super::types::Uri>,
    #[doc = "Type of media involved. Used when the event is about exporting/importing onto media."]
    pub r#media: Option<Box<super::super::types::Coding>>,
    #[doc = "Logical network location for application activity, if the activity has a network location."]
    pub r#network: Option<AuditEventAgentNetwork>,
    #[doc = "The reason (purpose of use), specific to this agent, that was used during the event being recorded."]
    pub r#purpose_of_use: Vec<super::super::types::CodeableConcept>,
}
impl Default for AuditEventAgent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#role: Default::default(),
            r#who: Default::default(),
            r#alt_id: Default::default(),
            r#name: Default::default(),
            r#requestor: {
                let mut default: super::super::types::Boolean = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#location: Default::default(),
            r#policy: Default::default(),
            r#media: Default::default(),
            r#network: Default::default(),
            r#purpose_of_use: Default::default(),
        }
    }
}
#[doc = "The system that is reporting the event."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Logical source location within the healthcare enterprise network.  For example, a hospital or other provider location within a multi-entity provider group."]
    pub r#site: Option<super::super::types::String>,
    #[doc = "Identifier of the source where the event was detected."]
    pub r#observer: Box<super::super::types::Reference>,
    #[doc = "Code specifying the type of source where event originated."]
    pub r#type: Vec<super::super::types::Coding>,
}
impl Default for AuditEventSource {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#site: Default::default(),
            r#observer: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#type: Default::default(),
        }
    }
}
#[doc = "Tagged value pairs for conveying additional information about the entity."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventEntityDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of extra detail provided in the value."]
    pub r#type: super::super::types::String,
    #[doc = "The  value of the extra detail."]
    pub r#value: AuditEventEntityDetailValue,
}
impl Default for AuditEventEntityDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#value: Default::default(),
        }
    }
}
#[doc = "Specific instances of data or objects that have been accessed."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies a specific instance of the entity. The reference should be version specific."]
    pub r#what: Option<Box<super::super::types::Reference>>,
    #[doc = "The type of the object that was involved in this audit event."]
    pub r#type: Option<Box<super::super::types::Coding>>,
    #[doc = "Code representing the role the entity played in the event being audited."]
    pub r#role: Option<Box<super::super::types::Coding>>,
    #[doc = "Identifier for the data life-cycle stage for the entity."]
    pub r#lifecycle: Option<Box<super::super::types::Coding>>,
    #[doc = "Security labels for the identified entity."]
    pub r#security_label: Vec<super::super::types::Coding>,
    #[doc = "A name of the entity in the audit event."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Text that describes the entity in more detail."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The query parameters for a query-type entities."]
    pub r#query: Option<super::super::types::Base64Binary>,
    #[doc = "Tagged value pairs for conveying additional information about the entity."]
    pub r#detail: Vec<AuditEventEntityDetail>,
}
impl Default for AuditEventEntity {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#what: Default::default(),
            r#type: Default::default(),
            r#role: Default::default(),
            r#lifecycle: Default::default(),
            r#security_label: Default::default(),
            r#name: Default::default(),
            r#description: Default::default(),
            r#query: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "A record of an event made for purposes of maintaining a security log. Typical uses include detection of intrusion attempts and monitoring for inappropriate usage."]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEvent {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier for a family of the event.  For example, a menu item, program, rule, policy, function code, application name or URL. It identifies the performed function."]
    pub r#type: Box<super::super::types::Coding>,
    #[doc = "Identifier for the category of event."]
    pub r#subtype: Vec<super::super::types::Coding>,
    #[doc = "Indicator for type of action performed during the event that generated the audit."]
    pub r#action: Option<super::super::types::Code>,
    #[doc = "The period during which the activity occurred."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The time when the event was recorded."]
    pub r#recorded: super::super::types::Instant,
    #[doc = "Indicates whether the event succeeded or failed."]
    pub r#outcome: Option<super::super::types::Code>,
    #[doc = "A free text description of the outcome of the event."]
    pub r#outcome_desc: Option<super::super::types::String>,
    #[doc = "The purposeOfUse (reason) that was used during the event being recorded."]
    pub r#purpose_of_event: Vec<super::super::types::CodeableConcept>,
    #[doc = "An actor taking an active role in the event or activity that is logged."]
    pub r#agent: Vec<AuditEventAgent>,
    #[doc = "The system that is reporting the event."]
    pub r#source: AuditEventSource,
    #[doc = "Specific instances of data or objects that have been accessed."]
    pub r#entity: Vec<AuditEventEntity>,
}
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
            r#type: {
                let mut default: Box<super::super::types::Coding> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#subtype: Default::default(),
            r#action: Default::default(),
            r#period: Default::default(),
            r#recorded: {
                let mut default: super::super::types::Instant = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#outcome: Default::default(),
            r#outcome_desc: Default::default(),
            r#purpose_of_event: Default::default(),
            r#agent: Default::default(),
            r#source: {
                let mut default: AuditEventSource = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#entity: Default::default(),
        }
    }
}
