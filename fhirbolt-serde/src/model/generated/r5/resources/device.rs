// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::resources::DeviceUdiCarrier>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if self.output_json {
            if let Some(some) = self.value.r#device_identifier.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("deviceIdentifier", &some)?;
            }
            if self.value.r#device_identifier.id.is_some()
                || !self.value.r#device_identifier.extension.is_empty()
            {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#device_identifier.id.as_ref(),
                    extension: &self.value.r#device_identifier.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_deviceIdentifier", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#device_identifier, |ctx| {
                state.serialize_entry("deviceIdentifier", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#issuer.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("issuer", &some)?;
            }
            if self.value.r#issuer.id.is_some() || !self.value.r#issuer.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#issuer.id.as_ref(),
                    extension: &self.value.r#issuer.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_issuer", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#issuer, |ctx| {
                state.serialize_entry("issuer", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#jurisdiction.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("jurisdiction", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_jurisdiction", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#jurisdiction.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("jurisdiction", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#carrier_aidc.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("carrierAIDC", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_carrierAIDC", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#carrier_aidc.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("carrierAIDC", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#carrier_hrf.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("carrierHRF", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_carrierHRF", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#carrier_hrf.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("carrierHRF", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#entry_type.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("entryType", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_entryType", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#entry_type.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("entryType", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::DeviceUdiCarrier>,
    >
{
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>>,
    >
{
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::DeviceUdiCarrier,
    >
{
    type Value = fhirbolt_model::r5::resources::DeviceUdiCarrier;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::DeviceUdiCarrier,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::DeviceUdiCarrier;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceUdiCarrier")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::DeviceUdiCarrier, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::DeviceIdentifier => {
                            if self.0.from_json {
                                let some = r#device_identifier.get_or_insert(Default::default());
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
                                r#device_identifier = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::DeviceIdentifierPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#device_identifier.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_deviceIdentifier",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("deviceIdentifier");
                            }
                        }
                        Field::Issuer => {
                            if self.0.from_json {
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
                                r#issuer = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
                                )?);
                            }
                        }
                        Field::IssuerPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#issuer.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_issuer"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("issuer");
                            }
                        }
                        Field::Jurisdiction => {
                            if self.0.from_json {
                                let some = r#jurisdiction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
                                )?);
                            }
                        }
                        Field::JurisdictionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#jurisdiction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_jurisdiction"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("jurisdiction");
                            }
                        }
                        Field::CarrierAidc => {
                            if self.0.from_json {
                                let some = r#carrier_aidc.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierAIDC"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#carrier_aidc.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierAIDC"));
                                }
                                r#carrier_aidc = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r5::types::Base64Binary>(),
                                    )?,
                                );
                            }
                        }
                        Field::CarrierAidcPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#carrier_aidc.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_carrierAIDC"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("carrierAIDC");
                            }
                        }
                        Field::CarrierHrf => {
                            if self.0.from_json {
                                let some = r#carrier_hrf.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierHRF"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#carrier_hrf.is_some() {
                                    return Err(serde::de::Error::duplicate_field("carrierHRF"));
                                }
                                r#carrier_hrf = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::CarrierHrfPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#carrier_hrf.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_carrierHRF"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("carrierHRF");
                            }
                        }
                        Field::EntryType => {
                            if self.0.from_json {
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
                                r#entry_type = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::EntryTypePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#entry_type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_entryType"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                Ok(fhirbolt_model::r5::resources::DeviceUdiCarrier {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::DeviceUdiCarrier>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::DeviceUdiCarrier>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::DeviceUdiCarrier>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::DeviceUdiCarrier>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::DeviceUdiCarrier>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r5::resources::DeviceUdiCarrier>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r5::resources::DeviceUdiCarrier>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::resources::DeviceName>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if self.output_json {
            if let Some(some) = self.value.r#value.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("value", &some)?;
            }
            if self.value.r#value.id.is_some() || !self.value.r#value.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#value.id.as_ref(),
                    extension: &self.value.r#value.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_value", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#type.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("type", &some)?;
            }
            if self.value.r#type.id.is_some() || !self.value.r#type.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#type.id.as_ref(),
                    extension: &self.value.r#type.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_type", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#display.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("display", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_display", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#display.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("display", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r5::resources::DeviceName>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r5::resources::DeviceName>>
{
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::DeviceName>>,
    >
{
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r5::resources::DeviceName>
{
    type Value = fhirbolt_model::r5::resources::DeviceName;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::DeviceName,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::DeviceName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceName")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::DeviceName, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Value => {
                            if self.0.from_json {
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
                                r#value = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("value");
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
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
                                r#type = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Display => {
                            if self.0.from_json {
                                let some = r#display.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("display"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#display.is_some() {
                                    return Err(serde::de::Error::duplicate_field("display"));
                                }
                                r#display = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::DisplayPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#display.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_display"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                Ok(fhirbolt_model::r5::resources::DeviceName {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::DeviceName>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::DeviceName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::DeviceName>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::DeviceName>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::DeviceName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::DeviceName>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::DeviceName>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r5::resources::DeviceName>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r5::resources::DeviceName>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceName>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::DeviceName>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceName>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r5::resources::DeviceName>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::resources::DeviceVersion>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if self.output_json {
            if let Some(some) = self.value.r#install_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("installDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_installDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#install_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("installDate", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#value.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("value", &some)?;
            }
            if self.value.r#value.id.is_some() || !self.value.r#value.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#value.id.as_ref(),
                    extension: &self.value.r#value.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_value", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::DeviceVersion>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::DeviceVersion>,
    >
{
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::DeviceVersion>>,
    >
{
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::DeviceVersion,
    >
{
    type Value = fhirbolt_model::r5::resources::DeviceVersion;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::DeviceVersion,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::DeviceVersion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceVersion")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::DeviceVersion, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Component => {
                            if r#component.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            r#component = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Identifier>>(),
                                )?,
                            );
                        }
                        Field::InstallDate => {
                            if self.0.from_json {
                                let some = r#install_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("installDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#install_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("installDate"));
                                }
                                r#install_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::InstallDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#install_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_installDate"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("installDate");
                            }
                        }
                        Field::Value => {
                            if self.0.from_json {
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
                                r#value = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                Ok(fhirbolt_model::r5::resources::DeviceVersion {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::DeviceVersion>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::DeviceVersion>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::DeviceVersion>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::DeviceVersion>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::DeviceVersion>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::DeviceVersion>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::DeviceVersion>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r5::resources::DeviceVersion>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r5::resources::DeviceVersion>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceVersion>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::DeviceVersion>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceVersion>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r5::resources::DeviceVersion>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::resources::DeviceConformsTo>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        self.with_context(&self.value.r#specification, |ctx| {
            state.serialize_entry("specification", ctx)
        })?;
        if self.output_json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("version", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_version", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#version.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::DeviceConformsTo>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::DeviceConformsTo>,
    >
{
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::DeviceConformsTo>>,
    >
{
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::DeviceConformsTo,
    >
{
    type Value = fhirbolt_model::r5::resources::DeviceConformsTo;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::DeviceConformsTo,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::DeviceConformsTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceConformsTo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::DeviceConformsTo, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Specification => {
                            if r#specification.is_some() {
                                return Err(serde::de::Error::duplicate_field("specification"));
                            }
                            r#specification = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Version => {
                            if self.0.from_json {
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
                                r#version = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::VersionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                Ok(fhirbolt_model::r5::resources::DeviceConformsTo {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::DeviceConformsTo>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::DeviceConformsTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::DeviceConformsTo>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::DeviceConformsTo>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::DeviceConformsTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::DeviceConformsTo>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::DeviceConformsTo>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r5::resources::DeviceConformsTo>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r5::resources::DeviceConformsTo>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceConformsTo>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::DeviceConformsTo>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceConformsTo>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r5::resources::DeviceConformsTo>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::resources::DeviceProperty>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        match self.value.r#value {
            fhirbolt_model::r5::resources::DevicePropertyValue::Quantity(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
            }
            fhirbolt_model::r5::resources::DevicePropertyValue::CodeableConcept(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueCodeableConcept", ctx)
                })?;
            }
            fhirbolt_model::r5::resources::DevicePropertyValue::String(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
            fhirbolt_model::r5::resources::DevicePropertyValue::Boolean(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueBoolean", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
            fhirbolt_model::r5::resources::DevicePropertyValue::Integer(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueInteger", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
            fhirbolt_model::r5::resources::DevicePropertyValue::Range(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
            }
            fhirbolt_model::r5::resources::DevicePropertyValue::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
            }
            fhirbolt_model::r5::resources::DevicePropertyValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::DeviceProperty>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::DeviceProperty>,
    >
{
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::DeviceProperty>>,
    >
{
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::DeviceProperty,
    >
{
    type Value = fhirbolt_model::r5::resources::DeviceProperty;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::DeviceProperty,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::DeviceProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceProperty")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::DeviceProperty, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(
                                fhirbolt_model::r5::resources::DevicePropertyValue::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Quantity>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: DevicePropertyValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r5::resources::DevicePropertyValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::resources::DevicePropertyValue::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: DevicePropertyValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r5::resources::DevicePropertyValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::resources::DevicePropertyValue::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r5::resources::DevicePropertyValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::resources::DevicePropertyValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: DevicePropertyValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r5::resources::DevicePropertyValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::resources::DevicePropertyValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r5::resources::DevicePropertyValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::resources::DevicePropertyValue::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: DevicePropertyValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r5::resources::DevicePropertyValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::resources::DevicePropertyValue::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value =
                                Some(fhirbolt_model::r5::resources::DevicePropertyValue::Range(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r5::types::Range>>(),
                                    )?,
                                ));
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: DevicePropertyValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r5::resources::DeviceProperty {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::DeviceProperty>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::DeviceProperty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::DeviceProperty>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::DeviceProperty>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::DeviceProperty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::DeviceProperty>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::DeviceProperty>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r5::resources::DeviceProperty>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r5::resources::DeviceProperty>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceProperty>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::DeviceProperty>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::DeviceProperty>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r5::resources::DeviceProperty>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r5::resources::Device {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::resources::Device>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Device")?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("implicitRules", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("language", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#language.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
            }
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
        if self.output_json {
            if let Some(some) = self.value.r#display_name.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("displayName", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_displayName", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#display_name.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("displayName", ctx))?;
            }
        }
        if let Some(some) = self.value.r#definition.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("definition", ctx))?;
        }
        if !self.value.r#udi_carrier.is_empty() {
            self.with_context(&self.value.r#udi_carrier, |ctx| {
                state.serialize_entry("udiCarrier", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#status.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_status", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#status.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
            }
        }
        if let Some(some) = self.value.r#availability_status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("availabilityStatus", ctx))?;
        }
        if let Some(some) = self.value.r#biological_source_event.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("biologicalSourceEvent", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#manufacturer.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("manufacturer", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_manufacturer", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#manufacturer.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("manufacturer", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#manufacture_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("manufactureDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_manufactureDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#manufacture_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("manufactureDate", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#expiration_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("expirationDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_expirationDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#expiration_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("expirationDate", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#lot_number.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lotNumber", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lotNumber", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#lot_number.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("lotNumber", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#serial_number.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("serialNumber", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_serialNumber", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#serial_number.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("serialNumber", ctx))?;
            }
        }
        if !self.value.r#name.is_empty() {
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#model_number.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("modelNumber", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_modelNumber", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#model_number.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("modelNumber", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#part_number.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("partNumber", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_partNumber", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#part_number.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("partNumber", ctx))?;
            }
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
        if self.output_json {
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#url.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
            }
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r5::resources::Device>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r5::resources::Device>>
{
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<Box<fhirbolt_model::r5::resources::Device>>>
{
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
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<fhirbolt_model::r5::resources::Device>
{
    type Value = fhirbolt_model::r5::resources::Device;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r5::resources::Device>
{
    type Value = fhirbolt_model::r5::resources::Device;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::Device,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::Device;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Device")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::Device, V::Error>
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r5::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r5::types::Identifier>>> =
                    None;
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
                let mut r#category: Option<Vec<Box<fhirbolt_model::r5::types::CodeableConcept>>> =
                    None;
                let mut r#type: Option<Vec<Box<fhirbolt_model::r5::types::CodeableConcept>>> = None;
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
                let mut r#contact: Option<Vec<Box<fhirbolt_model::r5::types::ContactPoint>>> = None;
                let mut r#location: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#url: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#endpoint: Option<Vec<Box<fhirbolt_model::r5::types::Reference>>> = None;
                let mut r#gateway: Option<Vec<Box<fhirbolt_model::r5::types::CodeableReference>>> =
                    None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r5::types::Annotation>>> = None;
                let mut r#safety: Option<Vec<Box<fhirbolt_model::r5::types::CodeableConcept>>> =
                    None;
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
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Meta>>(),
                            )?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                r#implicit_rules = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
                                )?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from_json {
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
                                r#language = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            r#text = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<Box<fhirbolt_model::r5::Resource>>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r5::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::DisplayName => {
                            if self.0.from_json {
                                let some = r#display_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("displayName"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#display_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("displayName"));
                                }
                                r#display_name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::DisplayNamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#display_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_displayName"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            r#definition = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                        }
                        Field::UdiCarrier => {
                            if self.0.from_json {
                                if r#udi_carrier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udiCarrier"));
                                }
                                r#udi_carrier =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r5::resources::DeviceUdiCarrier,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#udi_carrier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: DeviceUdiCarrier > ()) ?) ;
                            }
                        }
                        Field::Status => {
                            if self.0.from_json {
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
                                r#status = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            r#availability_status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::BiologicalSourceEvent => {
                            if r#biological_source_event.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "biologicalSourceEvent",
                                ));
                            }
                            r#biological_source_event = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Identifier>>(),
                                )?,
                            );
                        }
                        Field::Manufacturer => {
                            if self.0.from_json {
                                let some = r#manufacturer.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                r#manufacturer = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::ManufacturerPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#manufacturer.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_manufacturer"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("manufacturer");
                            }
                        }
                        Field::ManufactureDate => {
                            if self.0.from_json {
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
                                r#manufacture_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::ManufactureDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#manufacture_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_manufactureDate",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("manufactureDate");
                            }
                        }
                        Field::ExpirationDate => {
                            if self.0.from_json {
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
                                r#expiration_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::ExpirationDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#expiration_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_expirationDate",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("expirationDate");
                            }
                        }
                        Field::LotNumber => {
                            if self.0.from_json {
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
                                r#lot_number = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::LotNumberPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#lot_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lotNumber"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lotNumber");
                            }
                        }
                        Field::SerialNumber => {
                            if self.0.from_json {
                                let some = r#serial_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("serialNumber"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#serial_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("serialNumber"));
                                }
                                r#serial_number = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::SerialNumberPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#serial_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_serialNumber"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("serialNumber");
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: DeviceName >> ()) ?) ;
                            } else {
                                let vec = r#name.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r5::resources::DeviceName>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModelNumber => {
                            if self.0.from_json {
                                let some = r#model_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modelNumber"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#model_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modelNumber"));
                                }
                                r#model_number = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::ModelNumberPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#model_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_modelNumber"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("modelNumber");
                            }
                        }
                        Field::PartNumber => {
                            if self.0.from_json {
                                let some = r#part_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partNumber"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#part_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partNumber"));
                                }
                                r#part_number = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::PartNumberPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#part_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_partNumber"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("partNumber");
                            }
                        }
                        Field::Category => {
                            if self.0.from_json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Version => {
                            if self.0.from_json {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                r#version = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: DeviceVersion >> ()) ?) ;
                            } else {
                                let vec = r#version.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: DeviceVersion > ()) ?) ;
                            }
                        }
                        Field::ConformsTo => {
                            if self.0.from_json {
                                if r#conforms_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("conformsTo"));
                                }
                                r#conforms_to =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r5::resources::DeviceConformsTo,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#conforms_to.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: DeviceConformsTo > ()) ?) ;
                            }
                        }
                        Field::Property => {
                            if self.0.from_json {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: DeviceProperty >> ()) ?) ;
                            } else {
                                let vec = r#property.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: DeviceProperty > ()) ?) ;
                            }
                        }
                        Field::Mode => {
                            if r#mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            r#mode = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Cycle => {
                            if r#cycle.is_some() {
                                return Err(serde::de::Error::duplicate_field("cycle"));
                            }
                            r#cycle = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Count>>(),
                            )?);
                        }
                        Field::Duration => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            r#duration = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Duration>>(),
                                )?,
                            );
                        }
                        Field::Owner => {
                            if r#owner.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            r#owner = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Contact => {
                            if self.0.from_json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: ContactPoint > >> ()) ?) ;
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: ContactPoint > > ()) ?) ;
                            }
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Url => {
                            if self.0.from_json {
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
                                r#url = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
                                )?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
                            }
                        }
                        Field::Endpoint => {
                            if self.0.from_json {
                                if r#endpoint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endpoint"));
                                }
                                r#endpoint = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#endpoint.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Gateway => {
                            if self.0.from_json {
                                if r#gateway.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gateway"));
                                }
                                r#gateway =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableReference>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#gateway.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Annotation > > ()) ?) ;
                            }
                        }
                        Field::Safety => {
                            if self.0.from_json {
                                if r#safety.is_some() {
                                    return Err(serde::de::Error::duplicate_field("safety"));
                                }
                                r#safety =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#safety.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Parent => {
                            if r#parent.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            r#parent = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r5::resources::Device {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r5::resources::Device>>
{
    type Value = Box<fhirbolt_model::r5::resources::Device>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::Device>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<fhirbolt_model::r5::resources::Device>>
{
    type Value = Vec<fhirbolt_model::r5::resources::Device>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::Device>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::Device>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0.transmute::<fhirbolt_model::r5::resources::Device>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r5::resources::Device>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::Device>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::Device>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::Device>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r5::resources::Device>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
