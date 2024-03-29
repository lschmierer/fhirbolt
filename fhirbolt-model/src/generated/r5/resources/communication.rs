// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A communicated content (or for multi-part communications, one portion of the communication)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CommunicationPayloadContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Text, attachment(s), or resource(s) that was communicated to the recipient."]
#[derive(Debug, Clone, PartialEq)]
pub struct CommunicationPayload {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A communicated content (or for multi-part communications, one portion of the communication)."]
    pub r#content: CommunicationPayloadContent,
}
#[allow(clippy::derivable_impls)]
impl Default for CommunicationPayload {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#content: Default::default(),
        }
    }
}
#[doc = "A clinical or business level record of information being transmitted or shared; e.g. an alert that was sent to a responsible provider, a public health agency communication to a provider/reporter in response to a case report for a reportable condition."]
#[derive(Debug, Clone, PartialEq)]
pub struct Communication {
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
    #[doc = "Business identifiers assigned to this communication by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Communication."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Communication."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "An order, proposal or plan fulfilled in whole or in part by this Communication."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event (e.g. Communication, Procedure) of which this particular communication is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "Prior communication that this communication is in response to."]
    pub r#in_response_to: Vec<super::super::types::Reference>,
    #[doc = "The status of the transmission."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the Communication."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of message conveyed such as alert, notification, reminder, instruction, etc."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Characterizes how quickly the planned or in progress communication must be addressed. Includes concepts such as stat, urgent, routine."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A channel that was used for this communication (e.g. email, fax)."]
    pub r#medium: Vec<super::super::types::CodeableConcept>,
    #[doc = "The patient or group that was the focus of this communication."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Description of the purpose/content, similar to a subject line in an email."]
    pub r#topic: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Other resources that pertain to this communication and to which this communication should be associated."]
    pub r#about: Vec<super::super::types::Reference>,
    #[doc = "The Encounter during which this Communication was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The time when this communication was sent."]
    pub r#sent: Option<super::super::types::DateTime>,
    #[doc = "The time when this communication arrived at the destination."]
    pub r#received: Option<super::super::types::DateTime>,
    #[doc = "The entity (e.g. person, organization, clinical information system, care team or device) which is the target of the communication."]
    pub r#recipient: Vec<super::super::types::Reference>,
    #[doc = "The entity (e.g. person, organization, clinical information system, or device) which is the source of the communication."]
    pub r#sender: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason or justification for the communication."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "Text, attachment(s), or resource(s) that was communicated to the recipient."]
    pub r#payload: Vec<CommunicationPayload>,
    #[doc = "Additional notes or commentary about the communication by the sender, receiver or other interested parties."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for Communication {
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
            r#instantiates_canonical: Default::default(),
            r#instantiates_uri: Default::default(),
            r#based_on: Default::default(),
            r#part_of: Default::default(),
            r#in_response_to: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#category: Default::default(),
            r#priority: Default::default(),
            r#medium: Default::default(),
            r#subject: Default::default(),
            r#topic: Default::default(),
            r#about: Default::default(),
            r#encounter: Default::default(),
            r#sent: Default::default(),
            r#received: Default::default(),
            r#recipient: Default::default(),
            r#sender: Default::default(),
            r#reason: Default::default(),
            r#payload: Default::default(),
            r#note: Default::default(),
        }
    }
}
