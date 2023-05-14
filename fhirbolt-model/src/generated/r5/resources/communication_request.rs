// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "The communicated content (or for multi-part communications, one portion of the communication)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CommunicationRequestPayloadContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "The time when this communication is to occur."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CommunicationRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Text, attachment(s), or resource(s) to be communicated to the recipient."]
#[derive(Debug, Clone, PartialEq)]
pub struct CommunicationRequestPayload {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The communicated content (or for multi-part communications, one portion of the communication)."]
    pub r#content: CommunicationRequestPayloadContent,
}
#[allow(clippy::derivable_impls)]
impl Default for CommunicationRequestPayload {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#content: Default::default(),
        }
    }
}
#[doc = "A request to convey information; e.g. the CDS system proposes that an alert be sent to a responsible provider, the CDS system proposes that the public health agency be notified about a reportable condition."]
#[derive(Debug, Clone, PartialEq)]
pub struct CommunicationRequest {
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
    #[doc = "Business identifiers assigned to this communication request by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A plan or proposal that is fulfilled in whole or in part by this request."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "Completed or terminated request(s) whose function is taken by this new request."]
    pub r#replaces: Vec<super::super::types::Reference>,
    #[doc = "A shared identifier common to multiple independent Request instances that were activated/authorized more or less simultaneously by a single author.  The presence of the same identifier on each request ties those requests together and may have business ramifications in terms of reporting of results, billing, etc.  E.g. a requisition number shared by a set of lab tests ordered together, or a prescription number shared by all meds ordered at one time."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of the proposal or order."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the CommunicationRequest."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the level of authority/intentionality associated with the CommunicationRequest and where the request fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "The type of message to be sent such as alert, notification, reminder, instruction, etc."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Characterizes how quickly the proposed act must be initiated. Includes concepts such as stat, urgent, routine."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "If true indicates that the CommunicationRequest is asking for the specified action to *not* occur."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "A channel that was used for this communication (e.g. email, fax)."]
    pub r#medium: Vec<super::super::types::CodeableConcept>,
    #[doc = "The patient or group that is the focus of this communication request."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Other resources that pertain to this communication request and to which this communication request should be associated."]
    pub r#about: Vec<super::super::types::Reference>,
    #[doc = "The Encounter during which this CommunicationRequest was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Text, attachment(s), or resource(s) to be communicated to the recipient."]
    pub r#payload: Vec<CommunicationRequestPayload>,
    #[doc = "The time when this communication is to occur."]
    pub r#occurrence: Option<CommunicationRequestOccurrence>,
    #[doc = "For draft requests, indicates the date of initial creation.  For requests with other statuses, indicates the date of activation."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The device, individual, or organization who asks for the information to be shared."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "The entity (e.g. person, organization, clinical information system, device, group, or care team) which is the intended target of the communication."]
    pub r#recipient: Vec<super::super::types::Reference>,
    #[doc = "The entity (e.g. person, organization, clinical information system, or device) which is to be the source of the communication."]
    pub r#information_provider: Vec<super::super::types::Reference>,
    #[doc = "Describes why the request is being made in coded or textual form."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "Comments made about the request by the requester, sender, recipient, subject or other participants."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for CommunicationRequest {
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
            r#replaces: Default::default(),
            r#group_identifier: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#intent: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Default::default(),
            r#priority: Default::default(),
            r#do_not_perform: Default::default(),
            r#medium: Default::default(),
            r#subject: Default::default(),
            r#about: Default::default(),
            r#encounter: Default::default(),
            r#payload: Default::default(),
            r#occurrence: Default::default(),
            r#authored_on: Default::default(),
            r#requester: Default::default(),
            r#recipient: Default::default(),
            r#information_provider: Default::default(),
            r#reason: Default::default(),
            r#note: Default::default(),
        }
    }
}
