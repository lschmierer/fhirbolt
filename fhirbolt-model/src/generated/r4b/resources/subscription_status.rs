// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "Detailed information about events relevant to this subscription notification."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionStatusNotificationEvent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The sequential number of this event in this subscription context. Note that this value is a 64-bit integer value, encoded as a string."]
    pub r#event_number: super::super::types::String,
    #[doc = "The actual time this event occured on the server."]
    pub r#timestamp: Option<super::super::types::Instant>,
    #[doc = "The focus of this event. While this will usually be a reference to the focus resource of the event, it MAY contain a reference to a non-FHIR object."]
    pub r#focus: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional context information for this event. Generally, this will contain references to additional resources included with the event (e.g., the Patient relevant to an Encounter), however it MAY refer to non-FHIR objects."]
    pub r#additional_context: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubscriptionStatusNotificationEvent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#event_number: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#timestamp: Default::default(),
            r#focus: Default::default(),
            r#additional_context: Default::default(),
        }
    }
}
#[doc = "The SubscriptionStatus resource describes the state of a Subscription during notifications."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionStatus {
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
    #[doc = "The status of the subscription, which marks the server state for managing the subscription."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The type of event being conveyed with this notificaiton."]
    pub r#type: super::super::types::Code,
    #[doc = "The total number of actual events which have been generated since the Subscription was created (inclusive of this notification) - regardless of how many have been successfully communicated.  This number is NOT incremented for handshake and heartbeat notifications."]
    pub r#events_since_subscription_start: Option<super::super::types::String>,
    #[doc = "Detailed information about events relevant to this subscription notification."]
    pub r#notification_event: Vec<SubscriptionStatusNotificationEvent>,
    #[doc = "The reference to the Subscription which generated this notification."]
    pub r#subscription: Box<super::super::types::Reference>,
    #[doc = "The reference to the SubscriptionTopic for the Subscription which generated this notification."]
    pub r#topic: Option<super::super::types::Canonical>,
    #[doc = "A record of errors that occurred when the server processed a notification."]
    pub r#error: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubscriptionStatus {
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
            r#status: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#events_since_subscription_start: Default::default(),
            r#notification_event: Default::default(),
            r#subscription: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#topic: Default::default(),
            r#error: Default::default(),
        }
    }
}
