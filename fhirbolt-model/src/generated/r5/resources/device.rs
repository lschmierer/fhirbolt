// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "The value of the property specified by the associated property.type code."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DevicePropertyValue {
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
#[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceUdiCarrier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The device identifier (DI) is a mandatory, fixed portion of a UDI that identifies the labeler and the specific version or model of a device."]
    pub r#device_identifier: super::super::types::String,
    #[doc = "Organization that is charged with issuing UDIs for devices. For example, the US FDA issuers include: \n1) GS1: <http://hl7.org/fhir/NamingSystem/gs1>-di, \n2) HIBCC: <http://hl7.org/fhir/NamingSystem/hibcc>-diI, \n3) ICCBBA for blood containers: <http://hl7.org/fhir/NamingSystem/iccbba>-blood-di, \n4) ICCBA for other devices: <http://hl7.org/fhir/NamingSystem/iccbba>-other-di # Informationsstelle für Arzneispezialitäten (IFA GmbH) (EU only): <http://hl7.org/fhir/NamingSystem/ifa>-gmbh-di."]
    pub r#issuer: super::super::types::Uri,
    #[doc = "The identity of the authoritative source for UDI generation within a jurisdiction. All UDIs are globally unique within a single namespace with the appropriate repository uri as the system. For example, UDIs of devices managed in the U.S. by the FDA, the value is <http://hl7.org/fhir/NamingSystem/us>-fda-udi or in the European Union by the European Commission <http://hl7.org/fhir/NamingSystem/eu>-ec-udi."]
    pub r#jurisdiction: Option<super::super::types::Uri>,
    #[doc = "The full UDI carrier of the Automatic Identification and Data Capture (AIDC) technology representation of the barcode string as printed on the packaging of the device - e.g., a barcode or RFID.   Because of limitations on character sets in XML and the need to round-trip JSON data through XML, AIDC Formats *SHALL* be base64 encoded."]
    pub r#carrier_aidc: Option<super::super::types::Base64Binary>,
    #[doc = "The full UDI carrier as the human readable form (HRF) representation of the barcode string as printed on the packaging of the device."]
    pub r#carrier_hrf: Option<super::super::types::String>,
    #[doc = "A coded entry to indicate how the data was entered."]
    pub r#entry_type: Option<super::super::types::Code>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceUdiCarrier {
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
            r#jurisdiction: Default::default(),
            r#carrier_aidc: Default::default(),
            r#carrier_hrf: Default::default(),
            r#entry_type: Default::default(),
        }
    }
}
#[doc = "This represents the manufacturer's name of the device as provided by the device, from a UDI label, or by a person describing the Device.  This typically would be used when a person provides the name(s) or when the device represents one of the names available from DeviceDefinition."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The actual name that identifies the device."]
    pub r#value: super::super::types::String,
    #[doc = "Indicates the kind of name. RegisteredName | UserFriendlyName | PatientReportedName."]
    pub r#type: super::super::types::Code,
    #[doc = "Indicates the default or preferred name to be displayed."]
    pub r#display: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceName {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#display: Default::default(),
        }
    }
}
#[doc = "The actual design of the device or software version running on the device."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceVersion {
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
    #[doc = "The date the version was installed on the device."]
    pub r#install_date: Option<super::super::types::DateTime>,
    #[doc = "The version text."]
    pub r#value: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceVersion {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#component: Default::default(),
            r#install_date: Default::default(),
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Identifies the standards, specifications, or formal guidances for the capabilities supported by the device. The device may be certified as conformant to these specifications e.g., communication, performance, process, measurement, or specialization standards."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceConformsTo {
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
    pub r#version: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceConformsTo {
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
        }
    }
}
#[doc = "Static or essentially fixed characteristics or features of the device (e.g., time or timing attributes, resolution, accuracy, intended use or instructions for use, and physical attributes) that are not otherwise captured in more specific attributes."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code that specifies the property, such as resolution, color, size, being represented."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the property specified by the associated property.type code."]
    pub r#value: DevicePropertyValue,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceProperty {
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
#[doc = "This resource describes the properties (regulated, has real time clock, etc.), adminstrative (manufacturer name, model number, serial number, firmware, etc.), and type (knee replacement, blood pressure cuff, MRI, etc.) of a physical unit (these values do not change much within a given module, for example the serail number, manufacturer name, and model number). An actual unit may consist of several modules in a distinct hierarchy and these are represented by multiple Device resources and bound through the 'parent' element.\n\nAllows institutions to track their devices."]
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
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
    #[doc = "Unique instance identifiers assigned to a device by manufacturers other organizations or owners."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The name used to display by default when the device is referenced. Based on intent of use by the resource creator, this may reflect one of the names in Device.name, or may be another simple name."]
    pub r#display_name: Option<super::super::types::String>,
    #[doc = "The reference to the definition for the device."]
    pub r#definition: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
    pub r#udi_carrier: Vec<DeviceUdiCarrier>,
    #[doc = "The Device record status. This is not the status of the device like availability."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The availability of the device."]
    pub r#availability_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled."]
    pub r#biological_source_event: Option<Box<super::super::types::Identifier>>,
    #[doc = "A name of the manufacturer or entity legally responsible for the device."]
    pub r#manufacturer: Option<super::super::types::String>,
    #[doc = "The date and time when the device was manufactured."]
    pub r#manufacture_date: Option<super::super::types::DateTime>,
    #[doc = "The date and time beyond which this device is no longer valid or should not be used (if applicable)."]
    pub r#expiration_date: Option<super::super::types::DateTime>,
    #[doc = "Lot number assigned by the manufacturer."]
    pub r#lot_number: Option<super::super::types::String>,
    #[doc = "The serial number assigned by the organization when the device was manufactured."]
    pub r#serial_number: Option<super::super::types::String>,
    #[doc = "This represents the manufacturer's name of the device as provided by the device, from a UDI label, or by a person describing the Device.  This typically would be used when a person provides the name(s) or when the device represents one of the names available from DeviceDefinition."]
    pub r#name: Vec<DeviceName>,
    #[doc = "The manufacturer's model number for the device."]
    pub r#model_number: Option<super::super::types::String>,
    #[doc = "The part number or catalog number of the device."]
    pub r#part_number: Option<super::super::types::String>,
    #[doc = "Devices may be associated with one or more categories."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The kind or type of device. A device instance may have more than one type - in which case those are the types that apply to the specific instance of the device."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The actual design of the device or software version running on the device."]
    pub r#version: Vec<DeviceVersion>,
    #[doc = "Identifies the standards, specifications, or formal guidances for the capabilities supported by the device. The device may be certified as conformant to these specifications e.g., communication, performance, process, measurement, or specialization standards."]
    pub r#conforms_to: Vec<DeviceConformsTo>,
    #[doc = "Static or essentially fixed characteristics or features of the device (e.g., time or timing attributes, resolution, accuracy, intended use or instructions for use, and physical attributes) that are not otherwise captured in more specific attributes."]
    pub r#property: Vec<DeviceProperty>,
    #[doc = "The designated condition for performing a task with the device."]
    pub r#mode: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The series of occurrences that repeats during the operation of the device."]
    pub r#cycle: Option<Box<super::super::types::Count>>,
    #[doc = "A measurement of time during the device's operation (e.g., days, hours, mins, etc.)."]
    pub r#duration: Option<Box<super::super::types::Duration>>,
    #[doc = "An organization that is responsible for the provision and ongoing maintenance of the device."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Contact details for an organization or a particular human that is responsible for the device."]
    pub r#contact: Vec<super::super::types::ContactPoint>,
    #[doc = "The place where the device can be found."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "A network address on which the device may be contacted directly."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "Technical endpoints providing access to services provided by the device defined at this resource."]
    pub r#endpoint: Vec<super::super::types::Reference>,
    #[doc = "The linked device acting as a communication controller, data collector, translator, or concentrator for the current device (e.g., mobile phone application that relays a blood pressure device's data)."]
    pub r#gateway: Vec<super::super::types::CodeableReference>,
    #[doc = "Descriptive information, usage information or implantation information that is not captured in an existing element."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Provides additional safety characteristics about a medical device.  For example devices containing latex."]
    pub r#safety: Vec<super::super::types::CodeableConcept>,
    #[doc = "The higher level or encompassing device that this device is a logical part of."]
    pub r#parent: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Device {
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
            r#display_name: Default::default(),
            r#definition: Default::default(),
            r#udi_carrier: Default::default(),
            r#status: Default::default(),
            r#availability_status: Default::default(),
            r#biological_source_event: Default::default(),
            r#manufacturer: Default::default(),
            r#manufacture_date: Default::default(),
            r#expiration_date: Default::default(),
            r#lot_number: Default::default(),
            r#serial_number: Default::default(),
            r#name: Default::default(),
            r#model_number: Default::default(),
            r#part_number: Default::default(),
            r#category: Default::default(),
            r#type: Default::default(),
            r#version: Default::default(),
            r#conforms_to: Default::default(),
            r#property: Default::default(),
            r#mode: Default::default(),
            r#cycle: Default::default(),
            r#duration: Default::default(),
            r#owner: Default::default(),
            r#contact: Default::default(),
            r#location: Default::default(),
            r#url: Default::default(),
            r#endpoint: Default::default(),
            r#gateway: Default::default(),
            r#note: Default::default(),
            r#safety: Default::default(),
            r#parent: Default::default(),
        }
    }
}
