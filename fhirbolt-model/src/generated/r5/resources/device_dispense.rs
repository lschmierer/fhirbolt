// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Indicates who or what performed the event."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDispensePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Distinguishes the type of performer in the dispense.  For example, date enterer, packager, final checker."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who performed the action.  It should be assumed that the actor is the dispenser of the device."]
    pub r#actor: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDispensePerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Indicates that a device is to be or has been dispensed for a named person/patient.  This includes a description of the product (supply) provided and the instructions for using the device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDispense {
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
    #[doc = "Business identifier for this dispensation."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The order or request that this dispense is fulfilling."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The bigger event that this dispense is a part of."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "A code specifying the state of the set of dispense events."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the reason why a dispense was or was not performed."]
    pub r#status_reason: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Indicates the type of device dispense."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Identifies the device being dispensed. This is either a link to a resource representing the details of the device or a simple attribute carrying a code that identifies the device from a known list of devices."]
    pub r#device: Box<super::super::types::CodeableReference>,
    #[doc = "A link to a resource representing the person to whom the device is intended."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "Identifies the person who picked up the device or the person or location where the device was delivered.  This may be a patient or their caregiver, but some cases exist where it can be a healthcare professional or a location."]
    pub r#receiver: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter that establishes the context for this event."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional information that supports the device being dispensed."]
    pub r#supporting_information: Vec<super::super::types::Reference>,
    #[doc = "Indicates who or what performed the event."]
    pub r#performer: Vec<DeviceDispensePerformer>,
    #[doc = "The principal physical location where the dispense was performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the type of dispensing event that is performed."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of devices that have been dispensed."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The time when the dispensed product was packaged and reviewed."]
    pub r#prepared_date: Option<super::super::types::DateTime>,
    #[doc = "The time the dispensed product was made available to the patient or their representative."]
    pub r#when_handed_over: Option<super::super::types::DateTime>,
    #[doc = "Identification of the facility/location where the device was /should be shipped to, as part of the dispense process."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Extra information about the dispense that could not be conveyed in the other attributes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The full representation of the instructions."]
    pub r#usage_instruction: Option<super::super::types::Markdown>,
    #[doc = "A summary of the events of interest that have occurred, such as when the dispense was verified."]
    pub r#event_history: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDispense {
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
            r#part_of: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#category: Default::default(),
            r#device: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#receiver: Default::default(),
            r#encounter: Default::default(),
            r#supporting_information: Default::default(),
            r#performer: Default::default(),
            r#location: Default::default(),
            r#type: Default::default(),
            r#quantity: Default::default(),
            r#prepared_date: Default::default(),
            r#when_handed_over: Default::default(),
            r#destination: Default::default(),
            r#note: Default::default(),
            r#usage_instruction: Default::default(),
            r#event_history: Default::default(),
        }
    }
}
