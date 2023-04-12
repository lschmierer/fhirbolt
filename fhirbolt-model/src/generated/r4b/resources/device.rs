// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceUdiCarrier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The device identifier (DI) is a mandatory, fixed portion of a UDI that identifies the labeler and the specific version or model of a device."]
    pub r#device_identifier: Option<super::super::types::String>,
    #[doc = "Organization that is charged with issuing UDIs for devices.  For example, the US FDA issuers include :\n1) GS1: \n<http://hl7.org/fhir/NamingSystem/gs1>-di, \n2) HIBCC:\n<http://hl7.org/fhir/NamingSystem/hibcc>-dI, \n3) ICCBBA for blood containers:\n<http://hl7.org/fhir/NamingSystem/iccbba>-blood-di, \n4) ICCBA for other devices:\n<http://hl7.org/fhir/NamingSystem/iccbba>-other-di."]
    pub r#issuer: Option<super::super::types::Uri>,
    #[doc = "The identity of the authoritative source for UDI generation within a  jurisdiction.  All UDIs are globally unique within a single namespace with the appropriate repository uri as the system.  For example,  UDIs of devices managed in the U.S. by the FDA, the value is  <http://hl7.org/fhir/NamingSystem/fda>-udi."]
    pub r#jurisdiction: Option<super::super::types::Uri>,
    #[doc = "The full UDI carrier of the Automatic Identification and Data Capture (AIDC) technology representation of the barcode string as printed on the packaging of the device - e.g., a barcode or RFID.   Because of limitations on character sets in XML and the need to round-trip JSON data through XML, AIDC Formats *SHALL* be base64 encoded."]
    pub r#carrier_aidc: Option<super::super::types::Base64Binary>,
    #[doc = "The full UDI carrier as the human readable form (HRF) representation of the barcode string as printed on the packaging of the device."]
    pub r#carrier_hrf: Option<super::super::types::String>,
    #[doc = "A coded entry to indicate how the data was entered."]
    pub r#entry_type: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for DeviceUdiCarrier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#device_identifier.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("deviceIdentifier", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_deviceIdentifier", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#device_identifier.as_ref() {
                    state.serialize_entry("deviceIdentifier", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#issuer.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("issuer", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_issuer", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#issuer.as_ref() {
                    state.serialize_entry("issuer", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#jurisdiction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("jurisdiction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_jurisdiction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#jurisdiction.as_ref() {
                    state.serialize_entry("jurisdiction", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#carrier_aidc.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("carrierAIDC", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_carrierAIDC", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#carrier_aidc.as_ref() {
                    state.serialize_entry("carrierAIDC", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#carrier_hrf.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("carrierHRF", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_carrierHRF", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#carrier_hrf.as_ref() {
                    state.serialize_entry("carrierHRF", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#entry_type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("entryType", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_entryType", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#entry_type.as_ref() {
                    state.serialize_entry("entryType", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "This represents the manufacturer's name of the device as provided by the device, from a UDI label, or by a person describing the Device.  This typically would be used when a person provides the name(s) or when the device represents one of the names available from DeviceDefinition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceDeviceName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name that identifies the device."]
    pub r#name: super::super::types::String,
    #[doc = "The type of deviceName.\nUDILabelName | UserFriendlyName | PatientReportedName | ManufactureDeviceName | ModelName."]
    pub r#type: super::super::types::Code,
}
impl serde::ser::Serialize for DeviceDeviceName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            state.end()
        })
    }
}
#[doc = "The capabilities supported on a  device, the standards to which the device conforms for a particular purpose, and used for the communication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceSpecialization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The standard that is used to operate and communicate."]
    pub r#system_type: Box<super::super::types::CodeableConcept>,
    #[doc = "The version of the standard that is used to operate and communicate."]
    pub r#version: Option<super::super::types::String>,
}
impl serde::ser::Serialize for DeviceSpecialization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("systemType", &self.r#system_type)?;
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "The actual design of the device or software version running on the device."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceVersion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of the device version, e.g. manufacturer, approved, internal."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A single component of the device version."]
    pub r#component: Option<Box<super::super::types::Identifier>>,
    #[doc = "The version text."]
    pub r#value: super::super::types::String,
}
impl serde::ser::Serialize for DeviceVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#component.as_ref() {
                state.serialize_entry("component", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            state.end()
        })
    }
}
#[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Code that specifies the property DeviceDefinitionPropetyCode (Extensible)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Property value as a quantity."]
    pub r#value_quantity: Vec<Box<super::super::types::Quantity>>,
    #[doc = "Property value as a code, e.g., NTP4 (synced to NTP)."]
    pub r#value_code: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for DeviceProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if !self.r#value_quantity.is_empty() {
                state.serialize_entry("valueQuantity", &self.r#value_quantity)?;
            }
            if !self.r#value_code.is_empty() {
                state.serialize_entry("valueCode", &self.r#value_code)?;
            }
            state.end()
        })
    }
}
#[doc = "A type of a manufactured item that is used in the provision of healthcare without being substantially changed through that activity. The device may be a medical or non-medical device.\n\nAllows institutions to track their devices."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Device {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique instance identifiers assigned to a device by manufacturers other organizations or owners."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The reference to the definition for the device."]
    pub r#definition: Option<Box<super::super::types::Reference>>,
    #[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
    pub r#udi_carrier: Vec<DeviceUdiCarrier>,
    #[doc = "Status of the Device availability."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Reason for the dtatus of the Device availability."]
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The distinct identification string as required by regulation for a human cell, tissue, or cellular and tissue-based product."]
    pub r#distinct_identifier: Option<super::super::types::String>,
    #[doc = "A name of the manufacturer."]
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
    pub r#device_name: Vec<DeviceDeviceName>,
    #[doc = "The manufacturer's model number for the device."]
    pub r#model_number: Option<super::super::types::String>,
    #[doc = "The part number or catalog number of the device."]
    pub r#part_number: Option<super::super::types::String>,
    #[doc = "The kind or type of device."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The capabilities supported on a  device, the standards to which the device conforms for a particular purpose, and used for the communication."]
    pub r#specialization: Vec<DeviceSpecialization>,
    #[doc = "The actual design of the device or software version running on the device."]
    pub r#version: Vec<DeviceVersion>,
    #[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
    pub r#property: Vec<DeviceProperty>,
    #[doc = "Patient information, If the device is affixed to a person."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "An organization that is responsible for the provision and ongoing maintenance of the device."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Contact details for an organization or a particular human that is responsible for the device."]
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "The place where the device can be found."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "A network address on which the device may be contacted directly."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "Descriptive information, usage information or implantation information that is not captured in an existing element."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Provides additional safety characteristics about a medical device.  For example devices containing latex."]
    pub r#safety: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device that this device is attached to or is part of."]
    pub r#parent: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Device")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
                }
            }
            if let Some(some) = self.r#text.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if !self.r#contained.is_empty() {
                state.serialize_entry("contained", &self.r#contained)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if let Some(some) = self.r#definition.as_ref() {
                state.serialize_entry("definition", some)?;
            }
            if !self.r#udi_carrier.is_empty() {
                state.serialize_entry("udiCarrier", &self.r#udi_carrier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if !self.r#status_reason.is_empty() {
                state.serialize_entry("statusReason", &self.r#status_reason)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#distinct_identifier.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("distinctIdentifier", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_distinctIdentifier", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#distinct_identifier.as_ref() {
                    state.serialize_entry("distinctIdentifier", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#manufacturer.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("manufacturer", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_manufacturer", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#manufacturer.as_ref() {
                    state.serialize_entry("manufacturer", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#manufacture_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("manufactureDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_manufactureDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#manufacture_date.as_ref() {
                    state.serialize_entry("manufactureDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#expiration_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("expirationDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_expirationDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#expiration_date.as_ref() {
                    state.serialize_entry("expirationDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#lot_number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lotNumber", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lotNumber", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#lot_number.as_ref() {
                    state.serialize_entry("lotNumber", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#serial_number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("serialNumber", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_serialNumber", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#serial_number.as_ref() {
                    state.serialize_entry("serialNumber", some)?;
                }
            }
            if !self.r#device_name.is_empty() {
                state.serialize_entry("deviceName", &self.r#device_name)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#model_number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("modelNumber", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_modelNumber", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#model_number.as_ref() {
                    state.serialize_entry("modelNumber", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#part_number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("partNumber", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_partNumber", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#part_number.as_ref() {
                    state.serialize_entry("partNumber", some)?;
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#specialization.is_empty() {
                state.serialize_entry("specialization", &self.r#specialization)?;
            }
            if !self.r#version.is_empty() {
                state.serialize_entry("version", &self.r#version)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if let Some(some) = self.r#patient.as_ref() {
                state.serialize_entry("patient", some)?;
            }
            if let Some(some) = self.r#owner.as_ref() {
                state.serialize_entry("owner", some)?;
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#safety.is_empty() {
                state.serialize_entry("safety", &self.r#safety)?;
            }
            if let Some(some) = self.r#parent.as_ref() {
                state.serialize_entry("parent", some)?;
            }
            state.end()
        })
    }
}
