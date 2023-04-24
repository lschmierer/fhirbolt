// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The details about the device when it is in use to describe its operation."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceAssociationOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Device operational condition corresponding to the association."]
    pub r#status: Box<super::super::types::CodeableConcept>,
    #[doc = "The individual performing the action enabled by the device."]
    pub r#operator: Vec<super::super::types::Reference>,
    #[doc = "Begin and end dates and times for the device's operation."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl Default for DeviceAssociationOperation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#status: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#operator: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "A record of association of a device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceAssociation {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Instance identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Reference to the devices associated with the patient or group."]
    pub r#device: Box<super::super::types::Reference>,
    #[doc = "Describes the relationship between the device and subject."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates the state of the Device association."]
    pub r#status: Box<super::super::types::CodeableConcept>,
    #[doc = "The reasons given for the current association status."]
    pub r#status_reason: Vec<super::super::types::CodeableConcept>,
    #[doc = "The individual, group of individuals or device that the device is on or associated with."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Current anatomical location of the device in/on subject."]
    pub r#body_structure: Option<Box<super::super::types::Reference>>,
    #[doc = "Begin and end dates and times for the device association."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The details about the device when it is in use to describe its operation."]
    pub r#operation: Vec<DeviceAssociationOperation>,
}
impl Default for DeviceAssociation {
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
            r#device: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#category: Default::default(),
            r#status: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status_reason: Default::default(),
            r#subject: Default::default(),
            r#body_structure: Default::default(),
            r#period: Default::default(),
            r#operation: Default::default(),
        }
    }
}
