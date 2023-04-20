// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "The timing of the event (if this is a periodic trigger)."]
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    Reference(Box<super::super::types::Reference>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for TriggerDefinitionTiming {
    fn default() -> TriggerDefinitionTiming {
        TriggerDefinitionTiming::Invalid
    }
}
#[doc = "TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TriggerDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of triggering event."]
    pub r#type: super::super::types::Code,
    #[doc = "A formal name for the event. This may be an absolute URI that identifies the event formally (e.g. from a trigger registry), or a simple relative URI that identifies the event in a local context."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A code that identifies the event."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to a SubscriptionTopic resource that defines the event. If this element is provided, no other information about the trigger definition may be supplied."]
    pub r#subscription_topic: Option<super::super::types::Canonical>,
    #[doc = "The timing of the event (if this is a periodic trigger)."]
    pub r#timing: Option<TriggerDefinitionTiming>,
    #[doc = "The triggering data of the event (if this is a data trigger). If more than one data is requirement is specified, then all the data requirements must be true."]
    pub r#data: Vec<Box<super::super::types::DataRequirement>>,
    #[doc = "A boolean-valued expression that is evaluated in the context of the container of the trigger definition and returns whether or not the trigger fires."]
    pub r#condition: Option<Box<super::super::types::Expression>>,
}
