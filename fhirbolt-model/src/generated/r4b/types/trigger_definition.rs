// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "The timing of the event (if this is a periodic trigger)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TriggerDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    Reference(Box<super::super::types::Reference>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    #[default]
    Invalid,
}
#[doc = "Base StructureDefinition for TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element."]
#[derive(Debug, Clone, PartialEq)]
pub struct TriggerDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The type of triggering event."]
    pub r#type: super::super::types::Code,
    #[doc = "A formal name for the event. This may be an absolute URI that identifies the event formally (e.g. from a trigger registry), or a simple relative URI that identifies the event in a local context."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The timing of the event (if this is a periodic trigger)."]
    pub r#timing: Option<TriggerDefinitionTiming>,
    #[doc = "The triggering data of the event (if this is a data trigger). If more than one data is requirement is specified, then all the data requirements must be true."]
    pub r#data: Vec<super::super::types::DataRequirement>,
    #[doc = "A boolean-valued expression that is evaluated in the context of the container of the trigger definition and returns whether or not the trigger fires."]
    pub r#condition: Option<Box<super::super::types::Expression>>,
}
#[allow(clippy::derivable_impls)]
impl Default for TriggerDefinition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#name: Default::default(),
            r#timing: Default::default(),
            r#data: Default::default(),
            r#condition: Default::default(),
        }
    }
}
