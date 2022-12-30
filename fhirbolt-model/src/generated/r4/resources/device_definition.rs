// Generated on 2022-12-29 by fhirbolt-codegen v0.1.0
#[doc = "A name of the manufacturer."]
#[derive(Debug, Clone)]
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
#[derive(Default, Debug, Clone)]
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
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionUdiDeviceIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "deviceIdentifier")]
            DeviceIdentifier,
            #[serde(rename = "_deviceIdentifier")]
            DeviceIdentifierPrimitiveElement,
            #[serde(rename = "issuer")]
            Issuer,
            #[serde(rename = "_issuer")]
            IssuerPrimitiveElement,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "_jurisdiction")]
            JurisdictionPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionUdiDeviceIdentifier;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionUdiDeviceIdentifier")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<DeviceDefinitionUdiDeviceIdentifier, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#device_identifier: Option<super::super::types::String> = None;
                let mut r#issuer: Option<super::super::types::Uri> = None;
                let mut r#jurisdiction: Option<super::super::types::Uri> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::DeviceIdentifier => {
                                if _ctx.from_json {
                                    let some =
                                        r#device_identifier.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "deviceIdentifier",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#device_identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "deviceIdentifier",
                                        ));
                                    }
                                    r#device_identifier = Some(map_access.next_value()?);
                                }
                            }
                            Field::DeviceIdentifierPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#device_identifier.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_deviceIdentifier",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "deviceIdentifier",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "deviceIdentifier",
                                            "issuer",
                                            "jurisdiction",
                                        ],
                                    ));
                                }
                            }
                            Field::Issuer => {
                                if _ctx.from_json {
                                    let some = r#issuer.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issuer"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#issuer.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issuer"));
                                    }
                                    r#issuer = Some(map_access.next_value()?);
                                }
                            }
                            Field::IssuerPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#issuer.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_issuer"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "issuer",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "deviceIdentifier",
                                            "issuer",
                                            "jurisdiction",
                                        ],
                                    ));
                                }
                            }
                            Field::Jurisdiction => {
                                if _ctx.from_json {
                                    let some = r#jurisdiction.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "jurisdiction",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#jurisdiction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "jurisdiction",
                                        ));
                                    }
                                    r#jurisdiction = Some(map_access.next_value()?);
                                }
                            }
                            Field::JurisdictionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#jurisdiction.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_jurisdiction",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "jurisdiction",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "deviceIdentifier",
                                            "issuer",
                                            "jurisdiction",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "deviceIdentifier",
                                        "issuer",
                                        "jurisdiction",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceDefinitionUdiDeviceIdentifier {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#device_identifier: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#device_identifier.unwrap_or(Default::default())
                        } else {
                            r#device_identifier
                                .ok_or(serde::de::Error::missing_field("deviceIdentifier"))?
                        },
                        r#issuer: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#issuer.unwrap_or(Default::default())
                        } else {
                            r#issuer.ok_or(serde::de::Error::missing_field("issuer"))?
                        },
                        r#jurisdiction: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#jurisdiction.unwrap_or(Default::default())
                        } else {
                            r#jurisdiction.ok_or(serde::de::Error::missing_field("jurisdiction"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A name given to the device to identify it."]
#[derive(Default, Debug, Clone)]
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
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionDeviceName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionDeviceName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionDeviceName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionDeviceName, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_name"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &["id", "extension", "modifierExtension", "name", "type"],
                                    ));
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_type"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
                                        &["id", "extension", "modifierExtension", "name", "type"],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "name", "type"],
                                ));
                            },
                        }
                    }
                    Ok(DeviceDefinitionDeviceName {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The capabilities supported on a  device, the standards to which the device conforms for a particular purpose, and used for the communication."]
#[derive(Default, Debug, Clone)]
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
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionSpecialization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "systemType")]
            SystemType,
            #[serde(rename = "_systemType")]
            SystemTypePrimitiveElement,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionSpecialization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionSpecialization")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<DeviceDefinitionSpecialization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#system_type: Option<super::super::types::String> = None;
                let mut r#version: Option<super::super::types::String> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::SystemType => {
                                if _ctx.from_json {
                                    let some = r#system_type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "systemType",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#system_type.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "systemType",
                                        ));
                                    }
                                    r#system_type = Some(map_access.next_value()?);
                                }
                            }
                            Field::SystemTypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#system_type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_systemType",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "systemType",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "systemType",
                                            "version",
                                        ],
                                    ));
                                }
                            }
                            Field::Version => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_version"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "systemType",
                                            "version",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "systemType",
                                        "version",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceDefinitionSpecialization {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#system_type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#system_type.unwrap_or(Default::default())
                        } else {
                            r#system_type.ok_or(serde::de::Error::missing_field("systemType"))?
                        },
                        r#version,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Device capabilities."]
#[derive(Default, Debug, Clone)]
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
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "description")]
            Description,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionCapability;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionCapability")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                r#description = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "type",
                                        "description",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceDefinitionCapability {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#description: r#description.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
#[derive(Default, Debug, Clone)]
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
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueCode")]
            ValueCode,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionProperty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value_quantity: Option<Vec<Box<super::super::types::Quantity>>> = None;
                let mut r#value_code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::ValueQuantity => {
                                if r#value_quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value_quantity = Some(map_access.next_value()?);
                            }
                            Field::ValueCode => {
                                if r#value_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                r#value_code = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "type",
                                        "valueQuantity",
                                        "valueCode",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceDefinitionProperty {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#value_quantity: r#value_quantity.unwrap_or(vec![]),
                        r#value_code: r#value_code.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A substance used to create the material(s) of which the device is made."]
#[derive(Default, Debug, Clone)]
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
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionMaterial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "substance")]
            Substance,
            #[serde(rename = "alternate")]
            Alternate,
            #[serde(rename = "_alternate")]
            AlternatePrimitiveElement,
            #[serde(rename = "allergenicIndicator")]
            AllergenicIndicator,
            #[serde(rename = "_allergenicIndicator")]
            AllergenicIndicatorPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionMaterial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionMaterial")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionMaterial, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#substance: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#alternate: Option<super::super::types::Boolean> = None;
                let mut r#allergenic_indicator: Option<super::super::types::Boolean> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Substance => {
                                if r#substance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("substance"));
                                }
                                r#substance = Some(map_access.next_value()?);
                            }
                            Field::Alternate => {
                                if _ctx.from_json {
                                    let some = r#alternate.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("alternate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#alternate.is_some() {
                                        return Err(serde::de::Error::duplicate_field("alternate"));
                                    }
                                    r#alternate = Some(map_access.next_value()?);
                                }
                            }
                            Field::AlternatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#alternate.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_alternate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "alternate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "substance",
                                            "alternate",
                                            "allergenicIndicator",
                                        ],
                                    ));
                                }
                            }
                            Field::AllergenicIndicator => {
                                if _ctx.from_json {
                                    let some =
                                        r#allergenic_indicator.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "allergenicIndicator",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#allergenic_indicator.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "allergenicIndicator",
                                        ));
                                    }
                                    r#allergenic_indicator = Some(map_access.next_value()?);
                                }
                            }
                            Field::AllergenicIndicatorPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#allergenic_indicator.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_allergenicIndicator",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "allergenicIndicator",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "substance",
                                            "alternate",
                                            "allergenicIndicator",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "substance",
                                        "alternate",
                                        "allergenicIndicator",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceDefinitionMaterial {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#substance: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#substance.unwrap_or(Default::default())
                        } else {
                            r#substance.ok_or(serde::de::Error::missing_field("substance"))?
                        },
                        r#alternate,
                        r#allergenic_indicator,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The characteristics, operational status and capabilities of a medical-related component of a medical device."]
#[derive(Default, Debug, Clone)]
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
impl crate::AnyResource for DeviceDefinition {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for DeviceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for DeviceDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "udiDeviceIdentifier")]
            UdiDeviceIdentifier,
            #[serde(rename = "manufacturerString")]
            ManufacturerString,
            #[serde(rename = "_manufacturerString")]
            ManufacturerStringPrimitiveElement,
            #[serde(rename = "manufacturerReference")]
            ManufacturerReference,
            #[serde(rename = "deviceName")]
            DeviceName,
            #[serde(rename = "modelNumber")]
            ModelNumber,
            #[serde(rename = "_modelNumber")]
            ModelNumberPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "specialization")]
            Specialization,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "safety")]
            Safety,
            #[serde(rename = "shelfLifeStorage")]
            ShelfLifeStorage,
            #[serde(rename = "physicalCharacteristics")]
            PhysicalCharacteristics,
            #[serde(rename = "languageCode")]
            LanguageCode,
            #[serde(rename = "capability")]
            Capability,
            #[serde(rename = "property")]
            Property,
            #[serde(rename = "owner")]
            Owner,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "onlineInformation")]
            OnlineInformation,
            #[serde(rename = "_onlineInformation")]
            OnlineInformationPrimitiveElement,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "parentDevice")]
            ParentDevice,
            #[serde(rename = "material")]
            Material,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifier>> =
                    None;
                let mut r#manufacturer: Option<DeviceDefinitionManufacturer> = None;
                let mut r#device_name: Option<Vec<DeviceDefinitionDeviceName>> = None;
                let mut r#model_number: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#specialization: Option<Vec<DeviceDefinitionSpecialization>> = None;
                let mut r#version: Option<Vec<super::super::types::String>> = None;
                let mut r#safety: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#shelf_life_storage: Option<
                    Vec<Box<super::super::types::ProductShelfLife>>,
                > = None;
                let mut r#physical_characteristics: Option<
                    Box<super::super::types::ProdCharacteristic>,
                > = None;
                let mut r#language_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#capability: Option<Vec<DeviceDefinitionCapability>> = None;
                let mut r#property: Option<Vec<DeviceDefinitionProperty>> = None;
                let mut r#owner: Option<Box<super::super::types::Reference>> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#online_information: Option<super::super::types::Uri> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#parent_device: Option<Box<super::super::types::Reference>> = None;
                let mut r#material: Option<Vec<DeviceDefinitionMaterial>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "DeviceDefinition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"DeviceDefinition",
                                    ));
                                }
                            }
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Meta => {
                                if r#meta.is_some() {
                                    return Err(serde::de::Error::duplicate_field("meta"));
                                }
                                r#meta = Some(map_access.next_value()?);
                            }
                            Field::ImplicitRules => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_implicitRules",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implicitRules",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "udiDeviceIdentifier",
                                            "manufacturerString",
                                            "manufacturerReference",
                                            "deviceName",
                                            "modelNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "safety",
                                            "shelfLifeStorage",
                                            "physicalCharacteristics",
                                            "languageCode",
                                            "capability",
                                            "property",
                                            "owner",
                                            "contact",
                                            "url",
                                            "onlineInformation",
                                            "note",
                                            "quantity",
                                            "parentDevice",
                                            "material",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_language"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "udiDeviceIdentifier",
                                            "manufacturerString",
                                            "manufacturerReference",
                                            "deviceName",
                                            "modelNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "safety",
                                            "shelfLifeStorage",
                                            "physicalCharacteristics",
                                            "languageCode",
                                            "capability",
                                            "property",
                                            "owner",
                                            "contact",
                                            "url",
                                            "onlineInformation",
                                            "note",
                                            "quantity",
                                            "parentDevice",
                                            "material",
                                        ],
                                    ));
                                }
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::UdiDeviceIdentifier => {
                                if r#udi_device_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "udiDeviceIdentifier",
                                    ));
                                }
                                r#udi_device_identifier = Some(map_access.next_value()?);
                            }
                            Field::ManufacturerString => {
                                if _ctx.from_json {
                                    let r#enum = r#manufacturer.get_or_insert(
                                        DeviceDefinitionManufacturer::String(Default::default()),
                                    );
                                    if let DeviceDefinitionManufacturer::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "manufacturerString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "manufacturer[x]",
                                        ));
                                    }
                                } else {
                                    if r#manufacturer.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "manufacturerString",
                                        ));
                                    }
                                    r#manufacturer = Some(DeviceDefinitionManufacturer::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ManufacturerStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#manufacturer.get_or_insert(
                                        DeviceDefinitionManufacturer::String(Default::default()),
                                    );
                                    if let DeviceDefinitionManufacturer::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_manufacturerString",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_manufacturer[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "manufacturerString",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "udiDeviceIdentifier",
                                            "manufacturerString",
                                            "manufacturerReference",
                                            "deviceName",
                                            "modelNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "safety",
                                            "shelfLifeStorage",
                                            "physicalCharacteristics",
                                            "languageCode",
                                            "capability",
                                            "property",
                                            "owner",
                                            "contact",
                                            "url",
                                            "onlineInformation",
                                            "note",
                                            "quantity",
                                            "parentDevice",
                                            "material",
                                        ],
                                    ));
                                }
                            }
                            Field::ManufacturerReference => {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "manufacturerReference",
                                    ));
                                }
                                r#manufacturer = Some(DeviceDefinitionManufacturer::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DeviceName => {
                                if r#device_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("deviceName"));
                                }
                                r#device_name = Some(map_access.next_value()?);
                            }
                            Field::ModelNumber => {
                                if _ctx.from_json {
                                    let some = r#model_number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modelNumber",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#model_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modelNumber",
                                        ));
                                    }
                                    r#model_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::ModelNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#model_number.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_modelNumber",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "modelNumber",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "udiDeviceIdentifier",
                                            "manufacturerString",
                                            "manufacturerReference",
                                            "deviceName",
                                            "modelNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "safety",
                                            "shelfLifeStorage",
                                            "physicalCharacteristics",
                                            "languageCode",
                                            "capability",
                                            "property",
                                            "owner",
                                            "contact",
                                            "url",
                                            "onlineInformation",
                                            "note",
                                            "quantity",
                                            "parentDevice",
                                            "material",
                                        ],
                                    ));
                                }
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Specialization => {
                                if r#specialization.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialization",
                                    ));
                                }
                                r#specialization = Some(map_access.next_value()?);
                            }
                            Field::Version => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#version.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(values.len())
                                            .collect::<Vec<_>>(),
                                    );
                                    if vec.len() != values.len() {
                                        return Err(serde::de::Error::invalid_length(
                                            values.len(),
                                            &"primitive elements length",
                                        ));
                                    }
                                    if vec.iter().any(|v| v.value.is_some()) {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#version.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(elements.len())
                                            .collect::<Vec<_>>(),
                                    );
                                    if vec.len() != elements.len() {
                                        return Err(serde::de::Error::invalid_length(
                                            elements.len(),
                                            &"primitive values length",
                                        ));
                                    }
                                    if vec
                                        .iter()
                                        .any(|e| e.id.is_some() || !e.extension.is_empty())
                                    {
                                        return Err(serde::de::Error::duplicate_field("_version"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "udiDeviceIdentifier",
                                            "manufacturerString",
                                            "manufacturerReference",
                                            "deviceName",
                                            "modelNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "safety",
                                            "shelfLifeStorage",
                                            "physicalCharacteristics",
                                            "languageCode",
                                            "capability",
                                            "property",
                                            "owner",
                                            "contact",
                                            "url",
                                            "onlineInformation",
                                            "note",
                                            "quantity",
                                            "parentDevice",
                                            "material",
                                        ],
                                    ));
                                }
                            }
                            Field::Safety => {
                                if r#safety.is_some() {
                                    return Err(serde::de::Error::duplicate_field("safety"));
                                }
                                r#safety = Some(map_access.next_value()?);
                            }
                            Field::ShelfLifeStorage => {
                                if r#shelf_life_storage.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "shelfLifeStorage",
                                    ));
                                }
                                r#shelf_life_storage = Some(map_access.next_value()?);
                            }
                            Field::PhysicalCharacteristics => {
                                if r#physical_characteristics.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "physicalCharacteristics",
                                    ));
                                }
                                r#physical_characteristics = Some(map_access.next_value()?);
                            }
                            Field::LanguageCode => {
                                if r#language_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("languageCode"));
                                }
                                r#language_code = Some(map_access.next_value()?);
                            }
                            Field::Capability => {
                                if r#capability.is_some() {
                                    return Err(serde::de::Error::duplicate_field("capability"));
                                }
                                r#capability = Some(map_access.next_value()?);
                            }
                            Field::Property => {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some(map_access.next_value()?);
                            }
                            Field::Owner => {
                                if r#owner.is_some() {
                                    return Err(serde::de::Error::duplicate_field("owner"));
                                }
                                r#owner = Some(map_access.next_value()?);
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
                            }
                            Field::Url => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#url.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    r#url = Some(map_access.next_value()?);
                                }
                            }
                            Field::UrlPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_url"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "url",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "udiDeviceIdentifier",
                                            "manufacturerString",
                                            "manufacturerReference",
                                            "deviceName",
                                            "modelNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "safety",
                                            "shelfLifeStorage",
                                            "physicalCharacteristics",
                                            "languageCode",
                                            "capability",
                                            "property",
                                            "owner",
                                            "contact",
                                            "url",
                                            "onlineInformation",
                                            "note",
                                            "quantity",
                                            "parentDevice",
                                            "material",
                                        ],
                                    ));
                                }
                            }
                            Field::OnlineInformation => {
                                if _ctx.from_json {
                                    let some =
                                        r#online_information.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "onlineInformation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#online_information.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "onlineInformation",
                                        ));
                                    }
                                    r#online_information = Some(map_access.next_value()?);
                                }
                            }
                            Field::OnlineInformationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#online_information.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_onlineInformation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "onlineInformation",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "udiDeviceIdentifier",
                                            "manufacturerString",
                                            "manufacturerReference",
                                            "deviceName",
                                            "modelNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "safety",
                                            "shelfLifeStorage",
                                            "physicalCharacteristics",
                                            "languageCode",
                                            "capability",
                                            "property",
                                            "owner",
                                            "contact",
                                            "url",
                                            "onlineInformation",
                                            "note",
                                            "quantity",
                                            "parentDevice",
                                            "material",
                                        ],
                                    ));
                                }
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::ParentDevice => {
                                if r#parent_device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parentDevice"));
                                }
                                r#parent_device = Some(map_access.next_value()?);
                            }
                            Field::Material => {
                                if r#material.is_some() {
                                    return Err(serde::de::Error::duplicate_field("material"));
                                }
                                r#material = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "identifier",
                                        "udiDeviceIdentifier",
                                        "manufacturerString",
                                        "manufacturerReference",
                                        "deviceName",
                                        "modelNumber",
                                        "type",
                                        "specialization",
                                        "version",
                                        "safety",
                                        "shelfLifeStorage",
                                        "physicalCharacteristics",
                                        "languageCode",
                                        "capability",
                                        "property",
                                        "owner",
                                        "contact",
                                        "url",
                                        "onlineInformation",
                                        "note",
                                        "quantity",
                                        "parentDevice",
                                        "material",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceDefinition {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#udi_device_identifier: r#udi_device_identifier.unwrap_or(vec![]),
                        r#manufacturer,
                        r#device_name: r#device_name.unwrap_or(vec![]),
                        r#model_number,
                        r#type,
                        r#specialization: r#specialization.unwrap_or(vec![]),
                        r#version: r#version.unwrap_or(vec![]),
                        r#safety: r#safety.unwrap_or(vec![]),
                        r#shelf_life_storage: r#shelf_life_storage.unwrap_or(vec![]),
                        r#physical_characteristics,
                        r#language_code: r#language_code.unwrap_or(vec![]),
                        r#capability: r#capability.unwrap_or(vec![]),
                        r#property: r#property.unwrap_or(vec![]),
                        r#owner,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#url,
                        r#online_information,
                        r#note: r#note.unwrap_or(vec![]),
                        r#quantity,
                        r#parent_device,
                        r#material: r#material.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
