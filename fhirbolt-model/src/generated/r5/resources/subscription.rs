// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "The filter properties to be applied to narrow the subscription topic stream.  When multiple filters are applied, evaluates to true if all the conditions applicable to that resource are met; otherwise it returns false (i.e., logical AND)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionFilterBy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A resource listed in the `SubscriptionTopic` this `Subscription` references (`SubscriptionTopic.canFilterBy.resource`). This element can be used to differentiate filters for topics that include more than one resource type."]
    pub r#resource_type: Option<super::super::types::Uri>,
    #[doc = "The filter as defined in the `SubscriptionTopic.canFilterBy.filterParameter` element."]
    pub r#filter_parameter: super::super::types::String,
    #[doc = "Comparator applied to this filter parameter."]
    pub r#comparator: Option<super::super::types::Code>,
    #[doc = "Modifier applied to this filter parameter."]
    pub r#modifier: Option<super::super::types::Code>,
    #[doc = "The literal value or resource path as is legal in search - for example, `Patient/123` or `le1950`."]
    pub r#value: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for SubscriptionFilterBy {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#resource_type: Default::default(),
            r#filter_parameter: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#comparator: Default::default(),
            r#modifier: Default::default(),
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Channel-dependent information to send as part of the notification (e.g., HTTP Headers)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Parameter name for information passed to the channel for notifications, for example in the case of a REST hook wanting to pass through an authorization header, the name would be Authorization."]
    pub r#name: super::super::types::String,
    #[doc = "Parameter value for information passed to the channel for notifications, for example in the case of a REST hook wanting to pass through an authorization header, the value would be `Bearer 0193...`."]
    pub r#value: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for SubscriptionParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The subscription resource describes a particular client's request to be notified about a SubscriptionTopic."]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A formal identifier that is used to identify this code system when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A natural language name identifying the subscription."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The status of the subscription, which marks the server state for managing the subscription."]
    pub r#status: super::super::types::Code,
    #[doc = "The reference to the subscription topic to be notified about."]
    pub r#topic: super::super::types::Canonical,
    #[doc = "Contact details for a human to contact about the subscription. The primary use of this for system administrator troubleshooting."]
    pub r#contact: Vec<super::super::types::ContactPoint>,
    #[doc = "The time for the server to turn the subscription off."]
    pub r#end: Option<super::super::types::Instant>,
    #[doc = "Entity with authorization to make subsequent revisions to the Subscription and also determines what data the subscription is authorized to disclose."]
    pub r#managing_entity: Option<Box<super::super::types::Reference>>,
    #[doc = "A description of why this subscription is defined."]
    pub r#reason: Option<super::super::types::String>,
    #[doc = "The filter properties to be applied to narrow the subscription topic stream.  When multiple filters are applied, evaluates to true if all the conditions applicable to that resource are met; otherwise it returns false (i.e., logical AND)."]
    pub r#filter_by: Vec<SubscriptionFilterBy>,
    #[doc = "The type of channel to send notifications on."]
    pub r#channel_type: Box<super::super::types::Coding>,
    #[doc = "The url that describes the actual end-point to send notifications to."]
    pub r#endpoint: Option<super::super::types::Url>,
    #[doc = "Channel-dependent information to send as part of the notification (e.g., HTTP Headers)."]
    pub r#parameter: Vec<SubscriptionParameter>,
    #[doc = "If present, a 'heartbeat' notification (keep-alive) is sent via this channel with an interval period equal to this elements integer value in seconds.  If not present, a heartbeat notification is not sent."]
    pub r#heartbeat_period: Option<super::super::types::UnsignedInt>,
    #[doc = "If present, the maximum amount of time a server will allow before failing a notification attempt."]
    pub r#timeout: Option<super::super::types::UnsignedInt>,
    #[doc = "The MIME type to send the payload in - e.g., `application/fhir+xml` or `application/fhir+json`. Note that:\n\n* clients may request notifications in a specific FHIR version by using the [FHIR Version Parameter](<http.html>#version-parameter) - e.g., `application/fhir+json; fhirVersion=4.0`.\n\n* additional MIME types can be allowed by channels - e.g., `text/plain` and `text/html` are defined by the Email channel."]
    pub r#content_type: Option<super::super::types::Code>,
    #[doc = "How much of the resource content to deliver in the notification payload. The choices are an empty payload, only the resource id, or the full resource content."]
    pub r#content: Option<super::super::types::Code>,
    #[doc = "If present, the maximum number of events that will be included in a notification bundle. Note that this is not a strict limit on the number of entries in a bundle, as dependent resources can be included."]
    pub r#max_count: Option<super::super::types::PositiveInt>,
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
            r#identifier: Default::default(),
            r#name: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#topic: super::super::types::Canonical {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#contact: Default::default(),
            r#end: Default::default(),
            r#managing_entity: Default::default(),
            r#reason: Default::default(),
            r#filter_by: Default::default(),
            r#channel_type: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#endpoint: Default::default(),
            r#parameter: Default::default(),
            r#heartbeat_period: Default::default(),
            r#timeout: Default::default(),
            r#content_type: Default::default(),
            r#content: Default::default(),
            r#max_count: Default::default(),
        }
    }
}
