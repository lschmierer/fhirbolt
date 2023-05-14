// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::DeviceUdiCarrier;
impl serde::ser::Serialize for SerializationContext<&DeviceUdiCarrier> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Device.udiCarrier", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#device_identifier.id.as_deref() == Some("$invalid") {
                return missing_field_error("deviceIdentifier");
            }
            if let Some(some) = self.value.r#device_identifier.value.as_ref().map(Ok) {
                state.serialize_entry("deviceIdentifier", &some?)?;
            }
            if self.value.r#device_identifier.id.is_some()
                || !self.value.r#device_identifier.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#device_identifier.id.as_ref(),
                    extension: &self.value.r#device_identifier.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_deviceIdentifier", ctx)
                })?;
            }
        } else if self.value.r#device_identifier.id.as_deref() == Some("$invalid") {
            return missing_field_error("deviceIdentifier");
        } else {
            self.with_context(&self.value.r#device_identifier, |ctx| {
                state.serialize_entry("deviceIdentifier", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#issuer.id.as_deref() == Some("$invalid") {
                return missing_field_error("issuer");
            }
            if let Some(some) = self.value.r#issuer.value.as_ref().map(Ok) {
                state.serialize_entry("issuer", &some?)?;
            }
            if self.value.r#issuer.id.is_some() || !self.value.r#issuer.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#issuer.id.as_ref(),
                    extension: &self.value.r#issuer.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_issuer", ctx)
                })?;
            }
        } else if self.value.r#issuer.id.as_deref() == Some("$invalid") {
            return missing_field_error("issuer");
        } else {
            self.with_context(&self.value.r#issuer, |ctx| {
                state.serialize_entry("issuer", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#jurisdiction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("jurisdiction", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_jurisdiction", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#jurisdiction.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("jurisdiction", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#carrier_aidc.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("carrierAIDC", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_carrierAIDC", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#carrier_aidc.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("carrierAIDC", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#carrier_hrf.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("carrierHRF", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_carrierHRF", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#carrier_hrf.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("carrierHRF", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#entry_type.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("entryType", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_entryType", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#entry_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("entryType", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DeviceUdiCarrier>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DeviceUdiCarrier>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DeviceUdiCarrier> {
    type Value = DeviceUdiCarrier;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DeviceUdiCarrier>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DeviceUdiCarrier;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceUdiCarrier")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceUdiCarrier, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#device_identifier: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#issuer: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#jurisdiction: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#carrier_aidc: Option<fhirbolt_model::r5::types::Base64Binary> = None;
                let mut r#carrier_hrf: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#entry_type: Option<fhirbolt_model::r5::types::Code> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DeviceIdentifier => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#device_identifier.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "deviceIdentifier",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#device_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "deviceIdentifier",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#device_identifier =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DeviceIdentifierPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#device_identifier.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_deviceIdentifier",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("deviceIdentifier");
                            }
                        }
                        Field::Issuer => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#issuer.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issuer"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#issuer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issuer"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#issuer = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IssuerPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#issuer.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_issuer"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("issuer");
                            }
                        }
                        Field::Jurisdiction => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#jurisdiction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#jurisdiction = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::JurisdictionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#jurisdiction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_jurisdiction"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("jurisdiction");
                            }
                        }
                        Field::CarrierAidc => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#carrier_aidc.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierAIDC"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#carrier_aidc.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierAIDC"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Base64Binary,
                                > = self.0.transmute();
                                r#carrier_aidc = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CarrierAidcPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#carrier_aidc.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_carrierAIDC"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("carrierAIDC");
                            }
                        }
                        Field::CarrierHrf => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#carrier_hrf.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierHRF"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#carrier_hrf.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierHRF"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#carrier_hrf = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CarrierHrfPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#carrier_hrf.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_carrierHRF"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("carrierHRF");
                            }
                        }
                        Field::EntryType => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#entry_type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entryType"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#entry_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entryType"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#entry_type = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::EntryTypePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#entry_type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_entryType"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("entryType");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DeviceUdiCarrier {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#device_identifier: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#device_identifier.unwrap_or(Default::default())
                    } else {
                        r#device_identifier
                            .ok_or(serde::de::Error::missing_field("deviceIdentifier"))?
                    },
                    r#issuer: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#issuer.unwrap_or(Default::default())
                    } else {
                        r#issuer.ok_or(serde::de::Error::missing_field("issuer"))?
                    },
                    r#jurisdiction,
                    r#carrier_aidc,
                    r#carrier_hrf,
                    r#entry_type,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<DeviceUdiCarrier>> {
    type Value = Box<DeviceUdiCarrier>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DeviceUdiCarrier>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<DeviceUdiCarrier>> {
    type Value = Vec<DeviceUdiCarrier>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DeviceUdiCarrier>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DeviceUdiCarrier>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DeviceUdiCarrier> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::DeviceName;
impl serde::ser::Serialize for SerializationContext<&DeviceName> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Device.name", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#value.id.as_deref() == Some("$invalid") {
                return missing_field_error("value");
            }
            if let Some(some) = self.value.r#value.value.as_ref().map(Ok) {
                state.serialize_entry("value", &some?)?;
            }
            if self.value.r#value.id.is_some() || !self.value.r#value.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#value.id.as_ref(),
                    extension: &self.value.r#value.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_value", ctx)
                })?;
            }
        } else if self.value.r#value.id.as_deref() == Some("$invalid") {
            return missing_field_error("value");
        } else {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#type.id.as_deref() == Some("$invalid") {
                return missing_field_error("type");
            }
            if let Some(some) = self.value.r#type.value.as_ref().map(Ok) {
                state.serialize_entry("type", &some?)?;
            }
            if self.value.r#type.id.is_some() || !self.value.r#type.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#type.id.as_ref(),
                    extension: &self.value.r#type.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_type", ctx)
                })?;
            }
        } else if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#display.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("display", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_display", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#display.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("display", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DeviceName>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DeviceName>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DeviceName> {
    type Value = DeviceName;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DeviceName>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DeviceName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceName, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    #[serde(rename = "value")]
                    Value,
                    #[serde(rename = "_value")]
                    ValuePrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "display")]
                    Display,
                    #[serde(rename = "_display")]
                    DisplayPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "value",
                            "type",
                            "display",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#value: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#display: Option<fhirbolt_model::r5::types::Boolean> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Value => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#value = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("value");
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Display => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#display.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("display"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#display.is_some() {
                                    return Err(serde::de::Error::duplicate_field("display"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#display = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DisplayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#display.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_display"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("display");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DeviceName {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        r#value.ok_or(serde::de::Error::missing_field("value"))?
                    },
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#display,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<DeviceName>> {
    type Value = Box<DeviceName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DeviceName>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<DeviceName>> {
    type Value = Vec<DeviceName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DeviceName>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DeviceName>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DeviceName> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::DeviceVersion;
impl serde::ser::Serialize for SerializationContext<&DeviceVersion> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Device.version", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#component.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("component", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#install_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("installDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_installDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#install_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("installDate", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#value.id.as_deref() == Some("$invalid") {
                return missing_field_error("value");
            }
            if let Some(some) = self.value.r#value.value.as_ref().map(Ok) {
                state.serialize_entry("value", &some?)?;
            }
            if self.value.r#value.id.is_some() || !self.value.r#value.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#value.id.as_ref(),
                    extension: &self.value.r#value.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_value", ctx)
                })?;
            }
        } else if self.value.r#value.id.as_deref() == Some("$invalid") {
            return missing_field_error("value");
        } else {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DeviceVersion>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DeviceVersion>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DeviceVersion> {
    type Value = DeviceVersion;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DeviceVersion>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DeviceVersion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceVersion")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceVersion, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    #[serde(rename = "installDate")]
                    InstallDate,
                    #[serde(rename = "_installDate")]
                    InstallDatePrimitiveElement,
                    #[serde(rename = "value")]
                    Value,
                    #[serde(rename = "_value")]
                    ValuePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "component",
                            "installDate",
                            "value",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#component: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#install_date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#value: Option<fhirbolt_model::r5::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Component => {
                            if r#component.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#component = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::InstallDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#install_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("installDate"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#install_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("installDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#install_date = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::InstallDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#install_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_installDate"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("installDate");
                            }
                        }
                        Field::Value => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#value = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("value");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DeviceVersion {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#component,
                    r#install_date,
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        r#value.ok_or(serde::de::Error::missing_field("value"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<DeviceVersion>> {
    type Value = Box<DeviceVersion>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DeviceVersion>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<DeviceVersion>> {
    type Value = Vec<DeviceVersion>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DeviceVersion>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DeviceVersion>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DeviceVersion> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::DeviceConformsTo;
impl serde::ser::Serialize for SerializationContext<&DeviceConformsTo> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Device.conformsTo", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#category.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("category", ctx))?;
        }
        if self.value.r#specification.id.as_deref() == Some("$invalid") {
            return missing_field_error("specification");
        } else {
            self.with_context(&self.value.r#specification, |ctx| {
                state.serialize_entry("specification", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("version", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_version", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#version.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DeviceConformsTo>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DeviceConformsTo>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DeviceConformsTo> {
    type Value = DeviceConformsTo;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DeviceConformsTo>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DeviceConformsTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceConformsTo")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceConformsTo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "specification")]
                    Specification,
                    #[serde(rename = "version")]
                    Version,
                    #[serde(rename = "_version")]
                    VersionPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "category",
                            "specification",
                            "version",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#specification: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#version: Option<fhirbolt_model::r5::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Specification => {
                            if r#specification.is_some() {
                                return Err(serde::de::Error::duplicate_field("specification"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#specification = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Version => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#version = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::VersionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("version");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DeviceConformsTo {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category,
                    r#specification: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#specification.unwrap_or(Default::default())
                    } else {
                        r#specification.ok_or(serde::de::Error::missing_field("specification"))?
                    },
                    r#version,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<DeviceConformsTo>> {
    type Value = Box<DeviceConformsTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DeviceConformsTo>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<DeviceConformsTo>> {
    type Value = Vec<DeviceConformsTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DeviceConformsTo>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DeviceConformsTo>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DeviceConformsTo> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::DeviceProperty;
impl serde::ser::Serialize for SerializationContext<&DeviceProperty> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Device.property", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        {
            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
            match self.value.r#value {
                _Enum::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
                }
                _Enum::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueCodeableConcept", ctx)
                    })?;
                }
                _Enum::String(ref value) => {
                    if self.output == crate::context::Format::Json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueString", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueString", ctx))?;
                    }
                }
                _Enum::Boolean(ref value) => {
                    if self.output == crate::context::Format::Json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueBoolean", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueBoolean", ctx))?;
                    }
                }
                _Enum::Integer(ref value) => {
                    if self.output == crate::context::Format::Json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueInteger", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueInteger", ctx))?;
                    }
                }
                _Enum::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
                }
                _Enum::Attachment(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DeviceProperty>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DeviceProperty>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DeviceProperty> {
    type Value = DeviceProperty;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DeviceProperty>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DeviceProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceProperty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueInteger")]
                    ValueInteger,
                    #[serde(rename = "_valueInteger")]
                    ValueIntegerPrimitiveElement,
                    #[serde(rename = "valueRange")]
                    ValueRange,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "valueQuantity",
                            "valueCodeableConcept",
                            "valueString",
                            "valueBoolean",
                            "valueInteger",
                            "valueRange",
                            "valueAttachment",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#value: Option<fhirbolt_model::r5::resources::DevicePropertyValue> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ValueQuantity => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueCodeableConcept => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueString => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::String>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueBoolean => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Boolean>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueInteger => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Integer>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueRange => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Range>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueAttachment => {
                            use fhirbolt_model::r5::resources::DevicePropertyValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Attachment>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Attachment(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DeviceProperty {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<DeviceProperty>> {
    type Value = Box<DeviceProperty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DeviceProperty>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<DeviceProperty>> {
    type Value = Vec<DeviceProperty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DeviceProperty>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DeviceProperty>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DeviceProperty> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::Device;
impl crate::Resource for Device {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&Device> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Device", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Device")?;
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("id", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx))?;
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("id", ctx))?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("implicitRules", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("language", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        if !self.value.r#contained.is_empty() {
            self.with_context(&self.value.r#contained, |ctx| {
                state.serialize_entry("contained", ctx)
            })?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#display_name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("displayName", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_displayName", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#display_name.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("displayName", ctx))?;
        }
        if let Some(some) = self.value.r#definition.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("definition", ctx))?;
        }
        if !self.value.r#udi_carrier.is_empty() {
            self.with_context(&self.value.r#udi_carrier, |ctx| {
                state.serialize_entry("udiCarrier", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#status.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("status", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_status", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
        }
        if let Some(some) = self.value.r#availability_status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("availabilityStatus", ctx))?;
        }
        if let Some(some) = self.value.r#biological_source_event.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("biologicalSourceEvent", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#manufacturer.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("manufacturer", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_manufacturer", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#manufacturer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("manufacturer", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#manufacture_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("manufactureDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_manufactureDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#manufacture_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("manufactureDate", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#expiration_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("expirationDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_expirationDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#expiration_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expirationDate", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#lot_number.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("lotNumber", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lotNumber", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#lot_number.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("lotNumber", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#serial_number.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("serialNumber", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_serialNumber", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#serial_number.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("serialNumber", ctx))?;
        }
        if !self.value.r#name.is_empty() {
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#model_number.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("modelNumber", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_modelNumber", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#model_number.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("modelNumber", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#part_number.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("partNumber", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_partNumber", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#part_number.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("partNumber", ctx))?;
        }
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#version.is_empty() {
            self.with_context(&self.value.r#version, |ctx| {
                state.serialize_entry("version", ctx)
            })?;
        }
        if !self.value.r#conforms_to.is_empty() {
            self.with_context(&self.value.r#conforms_to, |ctx| {
                state.serialize_entry("conformsTo", ctx)
            })?;
        }
        if !self.value.r#property.is_empty() {
            self.with_context(&self.value.r#property, |ctx| {
                state.serialize_entry("property", ctx)
            })?;
        }
        if let Some(some) = self.value.r#mode.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("mode", ctx))?;
        }
        if let Some(some) = self.value.r#cycle.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("cycle", ctx))?;
        }
        if let Some(some) = self.value.r#duration.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("duration", ctx))?;
        }
        if let Some(some) = self.value.r#owner.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("owner", ctx))?;
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if let Some(some) = self.value.r#location.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("location", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("url", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#url.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
        }
        if !self.value.r#endpoint.is_empty() {
            self.with_context(&self.value.r#endpoint, |ctx| {
                state.serialize_entry("endpoint", ctx)
            })?;
        }
        if !self.value.r#gateway.is_empty() {
            self.with_context(&self.value.r#gateway, |ctx| {
                state.serialize_entry("gateway", ctx)
            })?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if !self.value.r#safety.is_empty() {
            self.with_context(&self.value.r#safety, |ctx| {
                state.serialize_entry("safety", ctx)
            })?;
        }
        if let Some(some) = self.value.r#parent.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("parent", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Device>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Device>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Device> {
    type Value = Device;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Device> {
    type Value = Device;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Device>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Device;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Device")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Device, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "resourceType")]
                    ResourceType,
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "_id")]
                    IdPrimitiveElement,
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
                    #[serde(rename = "displayName")]
                    DisplayName,
                    #[serde(rename = "_displayName")]
                    DisplayNamePrimitiveElement,
                    #[serde(rename = "definition")]
                    Definition,
                    #[serde(rename = "udiCarrier")]
                    UdiCarrier,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "availabilityStatus")]
                    AvailabilityStatus,
                    #[serde(rename = "biologicalSourceEvent")]
                    BiologicalSourceEvent,
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
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "modelNumber")]
                    ModelNumber,
                    #[serde(rename = "_modelNumber")]
                    ModelNumberPrimitiveElement,
                    #[serde(rename = "partNumber")]
                    PartNumber,
                    #[serde(rename = "_partNumber")]
                    PartNumberPrimitiveElement,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "version")]
                    Version,
                    #[serde(rename = "conformsTo")]
                    ConformsTo,
                    #[serde(rename = "property")]
                    Property,
                    #[serde(rename = "mode")]
                    Mode,
                    #[serde(rename = "cycle")]
                    Cycle,
                    #[serde(rename = "duration")]
                    Duration,
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
                    #[serde(rename = "endpoint")]
                    Endpoint,
                    #[serde(rename = "gateway")]
                    Gateway,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "safety")]
                    Safety,
                    #[serde(rename = "parent")]
                    Parent,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                            "displayName",
                            "definition",
                            "udiCarrier",
                            "status",
                            "availabilityStatus",
                            "biologicalSourceEvent",
                            "manufacturer",
                            "manufactureDate",
                            "expirationDate",
                            "lotNumber",
                            "serialNumber",
                            "name",
                            "modelNumber",
                            "partNumber",
                            "category",
                            "type",
                            "version",
                            "conformsTo",
                            "property",
                            "mode",
                            "cycle",
                            "duration",
                            "owner",
                            "contact",
                            "location",
                            "url",
                            "endpoint",
                            "gateway",
                            "note",
                            "safety",
                            "parent",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r5::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#display_name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#definition: Option<Box<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#udi_carrier: Option<
                    Vec<fhirbolt_model::r5::resources::DeviceUdiCarrier>,
                > = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#availability_status: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#biological_source_event: Option<
                    Box<fhirbolt_model::r5::types::Identifier>,
                > = None;
                let mut r#manufacturer: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#manufacture_date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#expiration_date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#lot_number: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#serial_number: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#name: Option<Vec<fhirbolt_model::r5::resources::DeviceName>> = None;
                let mut r#model_number: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#part_number: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#category: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#version: Option<Vec<fhirbolt_model::r5::resources::DeviceVersion>> = None;
                let mut r#conforms_to: Option<
                    Vec<fhirbolt_model::r5::resources::DeviceConformsTo>,
                > = None;
                let mut r#property: Option<Vec<fhirbolt_model::r5::resources::DeviceProperty>> =
                    None;
                let mut r#mode: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#cycle: Option<Box<fhirbolt_model::r5::types::Count>> = None;
                let mut r#duration: Option<Box<fhirbolt_model::r5::types::Duration>> = None;
                let mut r#owner: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#contact: Option<Vec<fhirbolt_model::r5::types::ContactPoint>> = None;
                let mut r#location: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#url: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#endpoint: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#gateway: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                let mut r#safety: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#parent: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
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
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_id"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("id");
                            }
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("language");
                            }
                        }
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DisplayName => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#display_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("displayName"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#display_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("displayName"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#display_name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DisplayNamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#display_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_displayName"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("displayName");
                            }
                        }
                        Field::Definition => {
                            if r#definition.is_some() {
                                return Err(serde::de::Error::duplicate_field("definition"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableReference>,
                            > = self.0.transmute();
                            r#definition = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UdiCarrier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#udi_carrier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udiCarrier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::DeviceUdiCarrier>,
                                > = self.0.transmute();
                                r#udi_carrier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#udi_carrier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::DeviceUdiCarrier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::AvailabilityStatus => {
                            if r#availability_status.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "availabilityStatus",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#availability_status =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::BiologicalSourceEvent => {
                            if r#biological_source_event.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "biologicalSourceEvent",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#biological_source_event =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Manufacturer => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#manufacturer.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#manufacturer = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ManufacturerPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#manufacturer.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_manufacturer"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("manufacturer");
                            }
                        }
                        Field::ManufactureDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#manufacture_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "manufactureDate",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#manufacture_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "manufactureDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#manufacture_date =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ManufactureDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#manufacture_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_manufactureDate",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("manufactureDate");
                            }
                        }
                        Field::ExpirationDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#expiration_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "expirationDate",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#expiration_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "expirationDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#expiration_date =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExpirationDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#expiration_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_expirationDate",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("expirationDate");
                            }
                        }
                        Field::LotNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#lot_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lotNumber"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#lot_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lotNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#lot_number = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LotNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#lot_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lotNumber"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lotNumber");
                            }
                        }
                        Field::SerialNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#serial_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("serialNumber"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#serial_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("serialNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#serial_number = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SerialNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#serial_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_serialNumber"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("serialNumber");
                            }
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::DeviceName>,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#name.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::DeviceName,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModelNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#model_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modelNumber"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#model_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modelNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#model_number = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModelNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#model_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_modelNumber"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("modelNumber");
                            }
                        }
                        Field::PartNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#part_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partNumber"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#part_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#part_number = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PartNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#part_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_partNumber"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("partNumber");
                            }
                        }
                        Field::Category => {
                            if self.0.from == crate::context::Format::Json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#category = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Version => {
                            if self.0.from == crate::context::Format::Json {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::DeviceVersion>,
                                > = self.0.transmute();
                                r#version = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#version.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::DeviceVersion,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ConformsTo => {
                            if self.0.from == crate::context::Format::Json {
                                if r#conforms_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("conformsTo"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::DeviceConformsTo>,
                                > = self.0.transmute();
                                r#conforms_to = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#conforms_to.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::DeviceConformsTo,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Property => {
                            if self.0.from == crate::context::Format::Json {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::DeviceProperty>,
                                > = self.0.transmute();
                                r#property = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#property.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::DeviceProperty,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Mode => {
                            if r#mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#mode = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Cycle => {
                            if r#cycle.is_some() {
                                return Err(serde::de::Error::duplicate_field("cycle"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Count>,
                            > = self.0.transmute();
                            r#cycle = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Duration => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Duration>,
                            > = self.0.transmute();
                            r#duration = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Owner => {
                            if r#owner.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#owner = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contact => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::ContactPoint>,
                                > = self.0.transmute();
                                r#contact = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::ContactPoint,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#location = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Url => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#url = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
                            }
                        }
                        Field::Endpoint => {
                            if self.0.from == crate::context::Format::Json {
                                if r#endpoint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endpoint"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#endpoint = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#endpoint.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Gateway => {
                            if self.0.from == crate::context::Format::Json {
                                if r#gateway.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gateway"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#gateway = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#gateway.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Annotation,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Safety => {
                            if self.0.from == crate::context::Format::Json {
                                if r#safety.is_some() {
                                    return Err(serde::de::Error::duplicate_field("safety"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#safety = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#safety.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Parent => {
                            if r#parent.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#parent = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
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
                    r#display_name,
                    r#definition,
                    r#udi_carrier: r#udi_carrier.unwrap_or(vec![]),
                    r#status,
                    r#availability_status,
                    r#biological_source_event,
                    r#manufacturer,
                    r#manufacture_date,
                    r#expiration_date,
                    r#lot_number,
                    r#serial_number,
                    r#name: r#name.unwrap_or(vec![]),
                    r#model_number,
                    r#part_number,
                    r#category: r#category.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#version: r#version.unwrap_or(vec![]),
                    r#conforms_to: r#conforms_to.unwrap_or(vec![]),
                    r#property: r#property.unwrap_or(vec![]),
                    r#mode,
                    r#cycle,
                    r#duration,
                    r#owner,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#location,
                    r#url,
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                    r#gateway: r#gateway.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#safety: r#safety.unwrap_or(vec![]),
                    r#parent,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Device>> {
    type Value = Box<Device>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Device>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Device>> {
    type Value = Vec<Device>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Device>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Device>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Device> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
