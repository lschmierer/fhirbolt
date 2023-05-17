// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "Details where to send notifications when resources are received that meet the criteria."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionChannel {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of channel to send notifications on."]
    pub r#type: super::super::types::Code,
    #[doc = "The url that describes the actual end-point to send messages to."]
    pub r#endpoint: Option<super::super::types::Url>,
    #[doc = "The mime type to send the payload in - either application/fhir+xml, or application/fhir+json. If the payload is not present, then there is no payload in the notification, just a notification. The mime type \"text/plain\" may also be used for Email and SMS subscriptions."]
    pub r#payload: Option<super::super::types::Code>,
    #[doc = "Additional headers / information to send as part of the notification."]
    pub r#header: Vec<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubscriptionChannel {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#endpoint: Default::default(),
            r#payload: Default::default(),
            r#header: Default::default(),
        }
    }
}
#[doc = "The subscription resource is used to define a push-based subscription from a server to another system. Once a subscription is registered with the server, the server checks every resource that is created or updated, and if the resource matches the given criteria, it sends a message on the defined \"channel\" so that another system can take an appropriate action."]
#[derive(Debug, Clone, PartialEq)]
pub struct Subscription {
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
    #[doc = "The status of the subscription, which marks the server state for managing the subscription."]
    pub r#status: super::super::types::Code,
    #[doc = "Contact details for a human to contact about the subscription. The primary use of this for system administrator troubleshooting."]
    pub r#contact: Vec<super::super::types::ContactPoint>,
    #[doc = "The time for the server to turn the subscription off."]
    pub r#end: Option<super::super::types::Instant>,
    #[doc = "A description of why this subscription is defined."]
    pub r#reason: super::super::types::String,
    #[doc = "The rules that the server should use to determine when to generate notifications for this subscription."]
    pub r#criteria: super::super::types::String,
    #[doc = "A record of the last error that occurred when the server processed a notification."]
    pub r#error: Option<super::super::types::String>,
    #[doc = "Details where to send notifications when resources are received that meet the criteria."]
    pub r#channel: SubscriptionChannel,
}
#[allow(clippy::derivable_impls)]
impl Default for Subscription {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#contact: Default::default(),
            r#end: Default::default(),
            r#reason: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#criteria: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#error: Default::default(),
            r#channel: SubscriptionChannel {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
