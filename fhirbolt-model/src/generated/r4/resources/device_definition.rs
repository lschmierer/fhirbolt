// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "A name of the manufacturer."]
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceDefinitionManufacturer {
    String(Box<super::super::types::String>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for DeviceDefinitionManufacturer {
    fn default() -> DeviceDefinitionManufacturer {
        DeviceDefinitionManufacturer::Invalid
    }
}
#[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdication porvided in the DeviceDefinition.udiDeviceIdentifier."]
    pub r#device_identifier: super::super::types::String,
    #[doc = "The organization that assigns the identifier algorithm."]
    pub r#issuer: super::super::types::Uri,
    #[doc = "The jurisdiction to which the deviceIdentifier applies."]
    pub r#jurisdiction: super::super::types::Uri,
}
impl serde::ser::Serialize for DeviceDefinitionUdiDeviceIdentifier {
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
                if let Some(some) = self.r#device_identifier.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("deviceIdentifier", &some)?;
                }
                if self.r#device_identifier.id.is_some()
                    || !self.r#device_identifier.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#device_identifier.id.as_ref(),
                        extension: &self.r#device_identifier.extension,
                    };
                    state.serialize_entry("_deviceIdentifier", &primitive_element)?;
                }
            } else {
                state.serialize_entry("deviceIdentifier", &self.r#device_identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#issuer.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("issuer", &some)?;
                }
                if self.r#issuer.id.is_some() || !self.r#issuer.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#issuer.id.as_ref(),
                        extension: &self.r#issuer.extension,
                    };
                    state.serialize_entry("_issuer", &primitive_element)?;
                }
            } else {
                state.serialize_entry("issuer", &self.r#issuer)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#jurisdiction.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("jurisdiction", &some)?;
                }
                if self.r#jurisdiction.id.is_some() || !self.r#jurisdiction.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#jurisdiction.id.as_ref(),
                        extension: &self.r#jurisdiction.extension,
                    };
                    state.serialize_entry("_jurisdiction", &primitive_element)?;
                }
            } else {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            state.end()
        })
    }
}
#[doc = "A name given to the device to identify it."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceDefinitionDeviceName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of the device."]
    pub r#name: super::super::types::String,
    #[doc = "The type of deviceName.\nUDILabelName | UserFriendlyName | PatientReportedName | ManufactureDeviceName | ModelName."]
    pub r#type: super::super::types::Code,
}
impl serde::ser::Serialize for DeviceDefinitionDeviceName {
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
pub struct DeviceDefinitionSpecialization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The standard that is used to operate and communicate."]
    pub r#system_type: super::super::types::String,
    #[doc = "The version of the standard that is used to operate and communicate."]
    pub r#version: Option<super::super::types::String>,
}
impl serde::ser::Serialize for DeviceDefinitionSpecialization {
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
                if let Some(some) = self.r#system_type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("systemType", &some)?;
                }
                if self.r#system_type.id.is_some() || !self.r#system_type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#system_type.id.as_ref(),
                        extension: &self.r#system_type.extension,
                    };
                    state.serialize_entry("_systemType", &primitive_element)?;
                }
            } else {
                state.serialize_entry("systemType", &self.r#system_type)?;
            }
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
#[doc = "Device capabilities."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceDefinitionCapability {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of capability."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Description of capability."]
    pub r#description: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for DeviceDefinitionCapability {
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
            if !self.r#description.is_empty() {
                state.serialize_entry("description", &self.r#description)?;
            }
            state.end()
        })
    }
}
#[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceDefinitionProperty {
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
impl serde::ser::Serialize for DeviceDefinitionProperty {
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
#[doc = "A substance used to create the material(s) of which the device is made."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceDefinitionMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The substance."]
    pub r#substance: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates an alternative material of the device."]
    pub r#alternate: Option<super::super::types::Boolean>,
    #[doc = "Whether the substance is a known or suspected allergen."]
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for DeviceDefinitionMaterial {
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
            state.serialize_entry("substance", &self.r#substance)?;
            if _ctx.output_json {
                if let Some(some) = self.r#alternate.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("alternate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_alternate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#alternate.as_ref() {
                    state.serialize_entry("alternate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#allergenic_indicator.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("allergenicIndicator", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_allergenicIndicator", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#allergenic_indicator.as_ref() {
                    state.serialize_entry("allergenicIndicator", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "The characteristics, operational status and capabilities of a medical-related component of a medical device."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceDefinition {
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
    #[doc = "Unique instance identifiers assigned to a device by the software, manufacturers, other organizations or owners. For example: handle ID."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
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
    pub r#safety: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Shelf Life and storage information."]
    pub r#shelf_life_storage: Vec<Box<super::super::types::ProductShelfLife>>,
    #[doc = "Dimensions, color etc."]
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    #[doc = "Language code for the human-readable text strings produced by the device (all supported)."]
    pub r#language_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Device capabilities."]
    pub r#capability: Vec<DeviceDefinitionCapability>,
    #[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
    pub r#property: Vec<DeviceDefinitionProperty>,
    #[doc = "An organization that is responsible for the provision and ongoing maintenance of the device."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Contact details for an organization or a particular human that is responsible for the device."]
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "A network address on which the device may be contacted directly."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "Access to on-line information about the device."]
    pub r#online_information: Option<super::super::types::Uri>,
    #[doc = "Descriptive information, usage information or implantation information that is not captured in an existing element."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The quantity of the device present in the packaging (e.g. the number of devices present in a pack, or the number of devices in the same package of the medicinal product)."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The parent device it can be part of."]
    pub r#parent_device: Option<Box<super::super::types::Reference>>,
    #[doc = "A substance used to create the material(s) of which the device is made."]
    pub r#material: Vec<DeviceDefinitionMaterial>,
}
impl serde::ser::Serialize for DeviceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "DeviceDefinition")?;
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
            if !self.r#udi_device_identifier.is_empty() {
                state.serialize_entry("udiDeviceIdentifier", &self.r#udi_device_identifier)?;
            }
            if let Some(some) = self.r#manufacturer.as_ref() {
                match some {
                    DeviceDefinitionManufacturer::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("manufacturerString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_manufacturerString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("manufacturerString", value)?;
                        }
                    }
                    DeviceDefinitionManufacturer::Reference(ref value) => {
                        state.serialize_entry("manufacturerReference", value)?;
                    }
                    DeviceDefinitionManufacturer::Invalid => {
                        return Err(serde::ser::Error::custom("manufacturer is invalid"))
                    }
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#specialization.is_empty() {
                state.serialize_entry("specialization", &self.r#specialization)?;
            }
            if _ctx.output_json {
                if !self.r#version.is_empty() {
                    let values = self
                        .r#version
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("version", &values)?;
                    }
                    let requires_elements = self
                        .r#version
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#version
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_version", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#version.is_empty() {
                    state.serialize_entry("version", &self.r#version)?;
                }
            }
            if !self.r#safety.is_empty() {
                state.serialize_entry("safety", &self.r#safety)?;
            }
            if !self.r#shelf_life_storage.is_empty() {
                state.serialize_entry("shelfLifeStorage", &self.r#shelf_life_storage)?;
            }
            if let Some(some) = self.r#physical_characteristics.as_ref() {
                state.serialize_entry("physicalCharacteristics", some)?;
            }
            if !self.r#language_code.is_empty() {
                state.serialize_entry("languageCode", &self.r#language_code)?;
            }
            if !self.r#capability.is_empty() {
                state.serialize_entry("capability", &self.r#capability)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if let Some(some) = self.r#owner.as_ref() {
                state.serialize_entry("owner", some)?;
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#online_information.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("onlineInformation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_onlineInformation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#online_information.as_ref() {
                    state.serialize_entry("onlineInformation", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if let Some(some) = self.r#parent_device.as_ref() {
                state.serialize_entry("parentDevice", some)?;
            }
            if !self.r#material.is_empty() {
                state.serialize_entry("material", &self.r#material)?;
            }
            state.end()
        })
    }
}
