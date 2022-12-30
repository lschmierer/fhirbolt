// Generated on 2022-12-29 by fhirbolt-codegen v0.1.0
#[doc = "Unique device identifier (UDI) assigned to device label or package.  Note that the Device may include multiple udiCarriers as it either may include just the udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it could have been sold."]
#[derive(Default, Debug, Clone)]
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
impl<'de> serde::de::Deserialize<'de> for DeviceUdiCarrier {
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
            #[serde(rename = "carrierAIDC")]
            CarrierAidc,
            #[serde(rename = "_carrierAIDC")]
            CarrierAidcPrimitiveElement,
            #[serde(rename = "carrierHRF")]
            CarrierHrf,
            #[serde(rename = "_carrierHRF")]
            CarrierHrfPrimitiveElement,
            #[serde(rename = "entryType")]
            EntryType,
            #[serde(rename = "_entryType")]
            EntryTypePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceUdiCarrier;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceUdiCarrier")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceUdiCarrier, V::Error>
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
                let mut r#carrier_aidc: Option<super::super::types::Base64Binary> = None;
                let mut r#carrier_hrf: Option<super::super::types::String> = None;
                let mut r#entry_type: Option<super::super::types::Code> = None;
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
                                            "carrierAIDC",
                                            "carrierHRF",
                                            "entryType",
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
                                            "carrierAIDC",
                                            "carrierHRF",
                                            "entryType",
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
                                            "carrierAIDC",
                                            "carrierHRF",
                                            "entryType",
                                        ],
                                    ));
                                }
                            }
                            Field::CarrierAidc => {
                                if _ctx.from_json {
                                    let some = r#carrier_aidc.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "carrierAIDC",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#carrier_aidc.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "carrierAIDC",
                                        ));
                                    }
                                    r#carrier_aidc = Some(map_access.next_value()?);
                                }
                            }
                            Field::CarrierAidcPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#carrier_aidc.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_carrierAIDC",
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
                                        "carrierAIDC",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "deviceIdentifier",
                                            "issuer",
                                            "jurisdiction",
                                            "carrierAIDC",
                                            "carrierHRF",
                                            "entryType",
                                        ],
                                    ));
                                }
                            }
                            Field::CarrierHrf => {
                                if _ctx.from_json {
                                    let some = r#carrier_hrf.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "carrierHRF",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#carrier_hrf.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "carrierHRF",
                                        ));
                                    }
                                    r#carrier_hrf = Some(map_access.next_value()?);
                                }
                            }
                            Field::CarrierHrfPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#carrier_hrf.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_carrierHRF",
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
                                        "carrierHRF",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "deviceIdentifier",
                                            "issuer",
                                            "jurisdiction",
                                            "carrierAIDC",
                                            "carrierHRF",
                                            "entryType",
                                        ],
                                    ));
                                }
                            }
                            Field::EntryType => {
                                if _ctx.from_json {
                                    let some = r#entry_type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("entryType"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#entry_type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("entryType"));
                                    }
                                    r#entry_type = Some(map_access.next_value()?);
                                }
                            }
                            Field::EntryTypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#entry_type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_entryType",
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
                                        "entryType",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "deviceIdentifier",
                                            "issuer",
                                            "jurisdiction",
                                            "carrierAIDC",
                                            "carrierHRF",
                                            "entryType",
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
                                        "carrierAIDC",
                                        "carrierHRF",
                                        "entryType",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceUdiCarrier {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#device_identifier,
                        r#issuer,
                        r#jurisdiction,
                        r#carrier_aidc,
                        r#carrier_hrf,
                        r#entry_type,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This represents the manufacturer's name of the device as provided by the device, from a UDI label, or by a person describing the Device.  This typically would be used when a person provides the name(s) or when the device represents one of the names available from DeviceDefinition."]
#[derive(Default, Debug, Clone)]
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
impl<'de> serde::de::Deserialize<'de> for DeviceDeviceName {
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
            type Value = DeviceDeviceName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDeviceName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDeviceName, V::Error>
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
                    Ok(DeviceDeviceName {
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
impl<'de> serde::de::Deserialize<'de> for DeviceSpecialization {
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
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceSpecialization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceSpecialization")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceSpecialization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#system_type: Option<Box<super::super::types::CodeableConcept>> = None;
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
                                if r#system_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("systemType"));
                                }
                                r#system_type = Some(map_access.next_value()?);
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
                    Ok(DeviceSpecialization {
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
#[doc = "The actual design of the device or software version running on the device."]
#[derive(Default, Debug, Clone)]
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
impl<'de> serde::de::Deserialize<'de> for DeviceVersion {
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
            #[serde(rename = "component")]
            Component,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceVersion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceVersion")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceVersion, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#component: Option<Box<super::super::types::Identifier>> = None;
                let mut r#value: Option<super::super::types::String> = None;
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
                            Field::Component => {
                                if r#component.is_some() {
                                    return Err(serde::de::Error::duplicate_field("component"));
                                }
                                r#component = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "component",
                                            "value",
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
                                        "type",
                                        "component",
                                        "value",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(DeviceVersion {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#component,
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties."]
#[derive(Default, Debug, Clone)]
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
impl<'de> serde::de::Deserialize<'de> for DeviceProperty {
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
            type Value = DeviceProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceProperty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceProperty, V::Error>
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
                    Ok(DeviceProperty {
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
#[doc = "A type of a manufactured item that is used in the provision of healthcare without being substantially changed through that activity. The device may be a medical or non-medical device.\n\nAllows institutions to track their devices."]
#[derive(Default, Debug, Clone)]
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
impl crate::AnyResource for Device {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
impl<'de> serde::de::Deserialize<'de> for Device {
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
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "udiCarrier")]
            UdiCarrier,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "distinctIdentifier")]
            DistinctIdentifier,
            #[serde(rename = "_distinctIdentifier")]
            DistinctIdentifierPrimitiveElement,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "_manufacturer")]
            ManufacturerPrimitiveElement,
            #[serde(rename = "manufactureDate")]
            ManufactureDate,
            #[serde(rename = "_manufactureDate")]
            ManufactureDatePrimitiveElement,
            #[serde(rename = "expirationDate")]
            ExpirationDate,
            #[serde(rename = "_expirationDate")]
            ExpirationDatePrimitiveElement,
            #[serde(rename = "lotNumber")]
            LotNumber,
            #[serde(rename = "_lotNumber")]
            LotNumberPrimitiveElement,
            #[serde(rename = "serialNumber")]
            SerialNumber,
            #[serde(rename = "_serialNumber")]
            SerialNumberPrimitiveElement,
            #[serde(rename = "deviceName")]
            DeviceName,
            #[serde(rename = "modelNumber")]
            ModelNumber,
            #[serde(rename = "_modelNumber")]
            ModelNumberPrimitiveElement,
            #[serde(rename = "partNumber")]
            PartNumber,
            #[serde(rename = "_partNumber")]
            PartNumberPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "specialization")]
            Specialization,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "property")]
            Property,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "owner")]
            Owner,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "safety")]
            Safety,
            #[serde(rename = "parent")]
            Parent,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Device;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Device")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Device, V::Error>
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
                let mut r#definition: Option<Box<super::super::types::Reference>> = None;
                let mut r#udi_carrier: Option<Vec<DeviceUdiCarrier>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#distinct_identifier: Option<super::super::types::String> = None;
                let mut r#manufacturer: Option<super::super::types::String> = None;
                let mut r#manufacture_date: Option<super::super::types::DateTime> = None;
                let mut r#expiration_date: Option<super::super::types::DateTime> = None;
                let mut r#lot_number: Option<super::super::types::String> = None;
                let mut r#serial_number: Option<super::super::types::String> = None;
                let mut r#device_name: Option<Vec<DeviceDeviceName>> = None;
                let mut r#model_number: Option<super::super::types::String> = None;
                let mut r#part_number: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#specialization: Option<Vec<DeviceSpecialization>> = None;
                let mut r#version: Option<Vec<DeviceVersion>> = None;
                let mut r#property: Option<Vec<DeviceProperty>> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#owner: Option<Box<super::super::types::Reference>> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#safety: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#parent: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Device" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Device",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
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
                            Field::Definition => {
                                if r#definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definition"));
                                }
                                r#definition = Some(map_access.next_value()?);
                            }
                            Field::UdiCarrier => {
                                if r#udi_carrier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udiCarrier"));
                                }
                                r#udi_carrier = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_status"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
                            }
                            Field::StatusReason => {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason = Some(map_access.next_value()?);
                            }
                            Field::DistinctIdentifier => {
                                if _ctx.from_json {
                                    let some =
                                        r#distinct_identifier.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "distinctIdentifier",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#distinct_identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "distinctIdentifier",
                                        ));
                                    }
                                    r#distinct_identifier = Some(map_access.next_value()?);
                                }
                            }
                            Field::DistinctIdentifierPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#distinct_identifier.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_distinctIdentifier",
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
                                        "distinctIdentifier",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
                            }
                            Field::Manufacturer => {
                                if _ctx.from_json {
                                    let some = r#manufacturer.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "manufacturer",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#manufacturer.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "manufacturer",
                                        ));
                                    }
                                    r#manufacturer = Some(map_access.next_value()?);
                                }
                            }
                            Field::ManufacturerPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#manufacturer.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_manufacturer",
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
                                        "manufacturer",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
                            }
                            Field::ManufactureDate => {
                                if _ctx.from_json {
                                    let some = r#manufacture_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "manufactureDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#manufacture_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "manufactureDate",
                                        ));
                                    }
                                    r#manufacture_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::ManufactureDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#manufacture_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_manufactureDate",
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
                                        "manufactureDate",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
                            }
                            Field::ExpirationDate => {
                                if _ctx.from_json {
                                    let some = r#expiration_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "expirationDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#expiration_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "expirationDate",
                                        ));
                                    }
                                    r#expiration_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExpirationDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#expiration_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_expirationDate",
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
                                        "expirationDate",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
                            }
                            Field::LotNumber => {
                                if _ctx.from_json {
                                    let some = r#lot_number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("lotNumber"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#lot_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field("lotNumber"));
                                    }
                                    r#lot_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::LotNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#lot_number.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_lotNumber",
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
                                        "lotNumber",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
                            }
                            Field::SerialNumber => {
                                if _ctx.from_json {
                                    let some = r#serial_number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "serialNumber",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#serial_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "serialNumber",
                                        ));
                                    }
                                    r#serial_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::SerialNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#serial_number.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_serialNumber",
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
                                        "serialNumber",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
                                        ],
                                    ));
                                }
                            }
                            Field::PartNumber => {
                                if _ctx.from_json {
                                    let some = r#part_number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "partNumber",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#part_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "partNumber",
                                        ));
                                    }
                                    r#part_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::PartNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#part_number.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_partNumber",
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
                                        "partNumber",
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
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
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                r#version = Some(map_access.next_value()?);
                            }
                            Field::Property => {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some(map_access.next_value()?);
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
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
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
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
                                            "definition",
                                            "udiCarrier",
                                            "status",
                                            "statusReason",
                                            "distinctIdentifier",
                                            "manufacturer",
                                            "manufactureDate",
                                            "expirationDate",
                                            "lotNumber",
                                            "serialNumber",
                                            "deviceName",
                                            "modelNumber",
                                            "partNumber",
                                            "type",
                                            "specialization",
                                            "version",
                                            "property",
                                            "patient",
                                            "owner",
                                            "contact",
                                            "location",
                                            "url",
                                            "note",
                                            "safety",
                                            "parent",
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
                            Field::Safety => {
                                if r#safety.is_some() {
                                    return Err(serde::de::Error::duplicate_field("safety"));
                                }
                                r#safety = Some(map_access.next_value()?);
                            }
                            Field::Parent => {
                                if r#parent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parent"));
                                }
                                r#parent = Some(map_access.next_value()?);
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
                                        "definition",
                                        "udiCarrier",
                                        "status",
                                        "statusReason",
                                        "distinctIdentifier",
                                        "manufacturer",
                                        "manufactureDate",
                                        "expirationDate",
                                        "lotNumber",
                                        "serialNumber",
                                        "deviceName",
                                        "modelNumber",
                                        "partNumber",
                                        "type",
                                        "specialization",
                                        "version",
                                        "property",
                                        "patient",
                                        "owner",
                                        "contact",
                                        "location",
                                        "url",
                                        "note",
                                        "safety",
                                        "parent",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Device {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#definition,
                        r#udi_carrier: r#udi_carrier.unwrap_or(vec![]),
                        r#status,
                        r#status_reason: r#status_reason.unwrap_or(vec![]),
                        r#distinct_identifier,
                        r#manufacturer,
                        r#manufacture_date,
                        r#expiration_date,
                        r#lot_number,
                        r#serial_number,
                        r#device_name: r#device_name.unwrap_or(vec![]),
                        r#model_number,
                        r#part_number,
                        r#type,
                        r#specialization: r#specialization.unwrap_or(vec![]),
                        r#version: r#version.unwrap_or(vec![]),
                        r#property: r#property.unwrap_or(vec![]),
                        r#patient,
                        r#owner,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#location,
                        r#url,
                        r#note: r#note.unwrap_or(vec![]),
                        r#safety: r#safety.unwrap_or(vec![]),
                        r#parent,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
