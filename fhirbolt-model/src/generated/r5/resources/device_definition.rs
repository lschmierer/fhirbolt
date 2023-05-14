// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "The value of the property specified by the associated property.type code."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DeviceDefinitionPropertyValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "Indicates where and when the device is available on the market."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionUdiDeviceIdentifierMarketDistribution {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Begin and end dates for the commercial distribution of the device."]
    pub r#market_period: Box<super::super::types::Period>,
    #[doc = "National state or territory to which the marketDistribution recers, typically where the device is commercialized."]
    pub r#sub_jurisdiction: super::super::types::Uri,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionUdiDeviceIdentifierMarketDistribution {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#market_period: Box::new(super::super::types::Period {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#sub_jurisdiction: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdiction provided in the DeviceDefinition.udiDeviceIdentifier."]
    pub r#device_identifier: super::super::types::String,
    #[doc = "The organization that assigns the identifier algorithm."]
    pub r#issuer: super::super::types::Uri,
    #[doc = "The jurisdiction to which the deviceIdentifier applies."]
    pub r#jurisdiction: super::super::types::Uri,
    #[doc = "Indicates where and when the device is available on the market."]
    pub r#market_distribution: Vec<DeviceDefinitionUdiDeviceIdentifierMarketDistribution>,
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
            r#market_distribution: Default::default(),
        }
    }
}
#[doc = "Identifier associated with the regulatory documentation (certificates, technical documentation, post-market surveillance documentation and reports) of a set of device models sharing the same intended purpose, risk class and essential design and manufacturing characteristics. One example is the Basic UDI-DI in Europe."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionRegulatoryIdentifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of identifier itself."]
    pub r#type: super::super::types::Code,
    #[doc = "The identifier itself."]
    pub r#device_identifier: super::super::types::String,
    #[doc = "The organization that issued this identifier."]
    pub r#issuer: super::super::types::Uri,
    #[doc = "The jurisdiction to which the deviceIdentifier applies."]
    pub r#jurisdiction: super::super::types::Uri,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionRegulatoryIdentifier {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
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
#[doc = "The name or names of the device as given by the manufacturer."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionDeviceName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A human-friendly name that is used to refer to the device - depending on the type, it can be the brand name, the common name or alias, or other."]
    pub r#name: super::super::types::String,
    #[doc = "The type of deviceName.\nRegisteredName | UserFriendlyName | PatientReportedName."]
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
#[doc = "What kind of device or device system this is."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A classification or risk class of the device model."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Further information qualifying this classification of the device model."]
    pub r#justification: Vec<super::super::types::RelatedArtifact>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionClassification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#justification: Default::default(),
        }
    }
}
#[doc = "Identifies the standards, specifications, or formal guidances for the capabilities supported by the device. The device may be certified as conformant to these specifications e.g., communication, performance, process, measurement, or specialization standards."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionConformsTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes the type of the standard, specification, or formal guidance."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code that identifies the specific standard, specification, protocol, formal guidance, regulation, legislation, or certification scheme to which the device adheres."]
    pub r#specification: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the specific form or variant of the standard, specification, or formal guidance. This may be a 'version number', release, document edition, publication year, or other label."]
    pub r#version: Vec<super::super::types::String>,
    #[doc = "Standard, regulation, certification, or guidance website, document, or other publication, or similar, supporting the conformance."]
    pub r#source: Vec<super::super::types::RelatedArtifact>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionConformsTo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Default::default(),
            r#specification: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#version: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "A device that is part (for example a component) of the present device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionHasPart {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Reference to the device that is part of the current device."]
    pub r#reference: Box<super::super::types::Reference>,
    #[doc = "Number of instances of the component device in the current device."]
    pub r#count: Option<super::super::types::Integer>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionHasPart {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#reference: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#count: Default::default(),
        }
    }
}
#[doc = "An organization that distributes the packaged device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionPackagingDistributor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Distributor's human-readable name."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Distributor as an Organization resource."]
    pub r#organization_reference: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionPackagingDistributor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#organization_reference: Default::default(),
        }
    }
}
#[doc = "Information about the packaging of the device, i.e. how the device is packaged."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionPackaging {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The business identifier of the packaged medication."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "A code that defines the specific type of packaging."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of items contained in the package (devices or sub-packages)."]
    pub r#count: Option<super::super::types::Integer>,
    #[doc = "An organization that distributes the packaged device."]
    pub r#distributor: Vec<DeviceDefinitionPackagingDistributor>,
    #[doc = "Unique Device Identifier (UDI) Barcode string on the packaging."]
    pub r#udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,
    #[doc = "Allows packages within packages."]
    pub r#packaging: Vec<DeviceDefinitionPackaging>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionPackaging {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#type: Default::default(),
            r#count: Default::default(),
            r#distributor: Default::default(),
            r#udi_device_identifier: Default::default(),
            r#packaging: Default::default(),
        }
    }
}
#[doc = "The version of the device or software."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionVersion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of the device version, e.g. manufacturer, approved, internal."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The hardware or software module of the device to which the version applies."]
    pub r#component: Option<Box<super::super::types::Identifier>>,
    #[doc = "The version text."]
    pub r#value: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionVersion {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#component: Default::default(),
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Static or essentially fixed characteristics or features of this kind of device that are otherwise not captured in more specific attributes, e.g., time or timing attributes, resolution, accuracy, and physical attributes."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code that specifies the property such as a resolution or color being represented."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the property specified by the associated property.type code."]
    pub r#value: DeviceDefinitionPropertyValue,
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
            r#value: Default::default(),
        }
    }
}
#[doc = "An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type indicates the relationship of the related device to the device instance."]
    pub r#relation: Box<super::super::types::Coding>,
    #[doc = "A reference to the linked device."]
    pub r#related_device: Box<super::super::types::CodeableReference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionLink {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#relation: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#related_device: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "A substance used to create the material(s) of which the device is made."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A substance that the device contains, may contain, or is made of - for example latex - to be used to determine patient compatibility. This is not intended to represent the composition of the device, only the clinically relevant materials."]
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
#[doc = "Information aimed at providing directions for the usage of this model of device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionGuideline {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The circumstances that form the setting for using the device."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "Detailed written and visual directions for the user on how to use the device."]
    pub r#usage_instruction: Option<super::super::types::Markdown>,
    #[doc = "A source of information or reference for this guideline."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "A clinical condition for which the device was designed to be used."]
    pub r#indication: Vec<super::super::types::CodeableConcept>,
    #[doc = "A specific situation when a device should not be used because it may cause harm."]
    pub r#contraindication: Vec<super::super::types::CodeableConcept>,
    #[doc = "Specific hazard alert information that a user needs to know before using the device."]
    pub r#warning: Vec<super::super::types::CodeableConcept>,
    #[doc = "A description of the general purpose or medical use of the device or its function."]
    pub r#intended_use: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionGuideline {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#use_context: Default::default(),
            r#usage_instruction: Default::default(),
            r#related_artifact: Default::default(),
            r#indication: Default::default(),
            r#contraindication: Default::default(),
            r#warning: Default::default(),
            r#intended_use: Default::default(),
        }
    }
}
#[doc = "Tracking of latest field safety corrective action."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionCorrectiveAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Whether the last corrective action known for this device was a recall."]
    pub r#recall: super::super::types::Boolean,
    #[doc = "The scope of the corrective action - whether the action targeted all units of a given device model, or only a specific set of batches identified by lot numbers, or individually identified devices identified by the serial name."]
    pub r#scope: Option<super::super::types::Code>,
    #[doc = "Start and end dates of the  corrective action."]
    pub r#period: Box<super::super::types::Period>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionCorrectiveAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#recall: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#scope: Default::default(),
            r#period: Box::new(super::super::types::Period {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Billing code or reference associated with the device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionChargeItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The code or reference for the charge item."]
    pub r#charge_item_code: Box<super::super::types::CodeableReference>,
    #[doc = "Coefficient applicable to the billing code."]
    pub r#count: Box<super::super::types::Quantity>,
    #[doc = "A specific time period in which this charge item applies."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "The context to which this charge item applies."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceDefinitionChargeItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#charge_item_code: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#count: Box::new(super::super::types::Quantity {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#effective_period: Default::default(),
            r#use_context: Default::default(),
        }
    }
}
#[doc = "This is a specialized resource that defines the characteristics and capabilities of a device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinition {
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
    #[doc = "Additional information to describe the device."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Unique instance identifiers assigned to a device by the software, manufacturers, other organizations or owners. For example: handle ID. The identifier is typically valued if the udiDeviceIdentifier, partNumber or modelNumber is not valued and represents a different type of identifier.  However, it is permissible to still include those identifiers in DeviceDefinition.identifier with the appropriate identifier.type."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
    pub r#udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,
    #[doc = "Identifier associated with the regulatory documentation (certificates, technical documentation, post-market surveillance documentation and reports) of a set of device models sharing the same intended purpose, risk class and essential design and manufacturing characteristics. One example is the Basic UDI-DI in Europe."]
    pub r#regulatory_identifier: Vec<DeviceDefinitionRegulatoryIdentifier>,
    #[doc = "The part number or catalog number of the device."]
    pub r#part_number: Option<super::super::types::String>,
    #[doc = "A name of the manufacturer  or legal representative e.g. labeler. Whether this is the actual manufacturer or the labeler or responsible depends on implementation and jurisdiction."]
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    #[doc = "The name or names of the device as given by the manufacturer."]
    pub r#device_name: Vec<DeviceDefinitionDeviceName>,
    #[doc = "The model number for the device for example as defined by the manufacturer or labeler, or other agency."]
    pub r#model_number: Option<super::super::types::String>,
    #[doc = "What kind of device or device system this is."]
    pub r#classification: Vec<DeviceDefinitionClassification>,
    #[doc = "Identifies the standards, specifications, or formal guidances for the capabilities supported by the device. The device may be certified as conformant to these specifications e.g., communication, performance, process, measurement, or specialization standards."]
    pub r#conforms_to: Vec<DeviceDefinitionConformsTo>,
    #[doc = "A device that is part (for example a component) of the present device."]
    pub r#has_part: Vec<DeviceDefinitionHasPart>,
    #[doc = "Information about the packaging of the device, i.e. how the device is packaged."]
    pub r#packaging: Vec<DeviceDefinitionPackaging>,
    #[doc = "The version of the device or software."]
    pub r#version: Vec<DeviceDefinitionVersion>,
    #[doc = "Safety characteristics of the device."]
    pub r#safety: Vec<super::super::types::CodeableConcept>,
    #[doc = "Shelf Life and storage information."]
    pub r#shelf_life_storage: Vec<super::super::types::ProductShelfLife>,
    #[doc = "Language code for the human-readable text strings produced by the device (all supported)."]
    pub r#language_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Static or essentially fixed characteristics or features of this kind of device that are otherwise not captured in more specific attributes, e.g., time or timing attributes, resolution, accuracy, and physical attributes."]
    pub r#property: Vec<DeviceDefinitionProperty>,
    #[doc = "An organization that is responsible for the provision and ongoing maintenance of the device."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Contact details for an organization or a particular human that is responsible for the device."]
    pub r#contact: Vec<super::super::types::ContactPoint>,
    #[doc = "An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device."]
    pub r#link: Vec<DeviceDefinitionLink>,
    #[doc = "Descriptive information, usage information or implantation information that is not captured in an existing element."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "A substance used to create the material(s) of which the device is made."]
    pub r#material: Vec<DeviceDefinitionMaterial>,
    #[doc = "Indicates the production identifier(s) that are expected to appear in the UDI carrier on the device label."]
    pub r#production_identifier_in_udi: Vec<super::super::types::Code>,
    #[doc = "Information aimed at providing directions for the usage of this model of device."]
    pub r#guideline: Option<DeviceDefinitionGuideline>,
    #[doc = "Tracking of latest field safety corrective action."]
    pub r#corrective_action: Option<DeviceDefinitionCorrectiveAction>,
    #[doc = "Billing code or reference associated with the device."]
    pub r#charge_item: Vec<DeviceDefinitionChargeItem>,
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
            r#description: Default::default(),
            r#identifier: Default::default(),
            r#udi_device_identifier: Default::default(),
            r#regulatory_identifier: Default::default(),
            r#part_number: Default::default(),
            r#manufacturer: Default::default(),
            r#device_name: Default::default(),
            r#model_number: Default::default(),
            r#classification: Default::default(),
            r#conforms_to: Default::default(),
            r#has_part: Default::default(),
            r#packaging: Default::default(),
            r#version: Default::default(),
            r#safety: Default::default(),
            r#shelf_life_storage: Default::default(),
            r#language_code: Default::default(),
            r#property: Default::default(),
            r#owner: Default::default(),
            r#contact: Default::default(),
            r#link: Default::default(),
            r#note: Default::default(),
            r#material: Default::default(),
            r#production_identifier_in_udi: Default::default(),
            r#guideline: Default::default(),
            r#corrective_action: Default::default(),
            r#charge_item: Default::default(),
        }
    }
}
