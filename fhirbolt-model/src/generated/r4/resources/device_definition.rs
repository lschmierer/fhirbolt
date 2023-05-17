// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A name of the manufacturer."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DeviceDefinitionManufacturer {
    String(super::super::types::String),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdication porvided in the DeviceDefinition.udiDeviceIdentifier."]
    pub r#device_identifier: super::super::types::String,
    #[doc = "The organization that assigns the identifier algorithm."]
    pub r#issuer: super::super::types::Uri,
    #[doc = "The jurisdiction to which the deviceIdentifier applies."]
    pub r#jurisdiction: super::super::types::Uri,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionUdiDeviceIdentifier {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#device_identifier: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#issuer: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#jurisdiction: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "A name given to the device to identify it."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionDeviceName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The name of the device."]
    pub r#name: super::super::types::String,
    #[doc = "The type of deviceName.\nUDILabelName | UserFriendlyName | PatientReportedName | ManufactureDeviceName | ModelName."]
    pub r#type: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionDeviceName {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The capabilities supported on a  device, the standards to which the device conforms for a particular purpose, and used for the communication."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionSpecialization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The standard that is used to operate and communicate."]
    pub r#system_type: super::super::types::String,
    #[doc = "The version of the standard that is used to operate and communicate."]
    pub r#version: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionSpecialization {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#system_type: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#version: Default::default(),
        }
    }
}
#[doc = "Device capabilities."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionCapability {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Type of capability."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Description of capability."]
    pub r#description: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionCapability {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#description: Default::default(),
        }
    }
}
#[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code that specifies the property DeviceDefinitionPropetyCode (Extensible)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Property value as a quantity."]
    pub r#value_quantity: Vec<super::super::types::Quantity>,
    #[doc = "Property value as a code, e.g., NTP4 (synced to NTP)."]
    pub r#value_code: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value_quantity: Default::default(),
            r#value_code: Default::default(),
        }
    }
}
#[doc = "A substance used to create the material(s) of which the device is made."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The substance."]
    pub r#substance: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates an alternative material of the device."]
    pub r#alternate: Option<super::super::types::Boolean>,
    #[doc = "Whether the substance is a known or suspected allergen."]
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionMaterial {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#substance: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#alternate: Default::default(),
            r#allergenic_indicator: Default::default(),
        }
    }
}
#[doc = "The characteristics, operational status and capabilities of a medical-related component of a medical device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinition {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique instance identifiers assigned to a device by the software, manufacturers, other organizations or owners. For example: handle ID."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
    pub r#udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,
    #[doc = "A name of the manufacturer."]
    pub r#manufacturer: Option<DeviceDefinitionManufacturer>,
    #[doc = "A name given to the device to identify it."]
    pub r#device_name: Vec<DeviceDefinitionDeviceName>,
    #[doc = "The model number for the device."]
    pub r#model_number: Option<super::super::types::String>,
    #[doc = "What kind of device or device system this is."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The capabilities supported on a  device, the standards to which the device conforms for a particular purpose, and used for the communication."]
    pub r#specialization: Vec<DeviceDefinitionSpecialization>,
    #[doc = "The available versions of the device, e.g., software versions."]
    pub r#version: Vec<super::super::types::String>,
    #[doc = "Safety characteristics of the device."]
    pub r#safety: Vec<super::super::types::CodeableConcept>,
    #[doc = "Shelf Life and storage information."]
    pub r#shelf_life_storage: Vec<super::super::types::ProductShelfLife>,
    #[doc = "Dimensions, color etc."]
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    #[doc = "Language code for the human-readable text strings produced by the device (all supported)."]
    pub r#language_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Device capabilities."]
    pub r#capability: Vec<DeviceDefinitionCapability>,
    #[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
    pub r#property: Vec<DeviceDefinitionProperty>,
    #[doc = "An organization that is responsible for the provision and ongoing maintenance of the device."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Contact details for an organization or a particular human that is responsible for the device."]
    pub r#contact: Vec<super::super::types::ContactPoint>,
    #[doc = "A network address on which the device may be contacted directly."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "Access to on-line information about the device."]
    pub r#online_information: Option<super::super::types::Uri>,
    #[doc = "Descriptive information, usage information or implantation information that is not captured in an existing element."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The quantity of the device present in the packaging (e.g. the number of devices present in a pack, or the number of devices in the same package of the medicinal product)."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The parent device it can be part of."]
    pub r#parent_device: Option<Box<super::super::types::Reference>>,
    #[doc = "A substance used to create the material(s) of which the device is made."]
    pub r#material: Vec<DeviceDefinitionMaterial>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinition {
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
            r#udi_device_identifier: Default::default(),
            r#manufacturer: Default::default(),
            r#device_name: Default::default(),
            r#model_number: Default::default(),
            r#type: Default::default(),
            r#specialization: Default::default(),
            r#version: Default::default(),
            r#safety: Default::default(),
            r#shelf_life_storage: Default::default(),
            r#physical_characteristics: Default::default(),
            r#language_code: Default::default(),
            r#capability: Default::default(),
            r#property: Default::default(),
            r#owner: Default::default(),
            r#contact: Default::default(),
            r#url: Default::default(),
            r#online_information: Default::default(),
            r#note: Default::default(),
            r#quantity: Default::default(),
            r#parent_device: Default::default(),
            r#material: Default::default(),
        }
    }
}
