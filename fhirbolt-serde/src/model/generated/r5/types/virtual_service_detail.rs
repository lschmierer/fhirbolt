// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::types::VirtualServiceDetail>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "VirtualServiceDetail", field
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
        if let Some(some) = self.value.r#channel_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("channelType", ctx))?;
        }
        if let Some(some) = self.value.r#address.as_ref() {
            match some {
                fhirbolt_model::r5::types::VirtualServiceDetailAddress::Url(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("addressUrl", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_addressUrl", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("addressUrl", ctx))?;
                    }
                }
                fhirbolt_model::r5::types::VirtualServiceDetailAddress::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("addressString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_addressString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("addressString", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r5::types::VirtualServiceDetailAddress::ContactPoint(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("addressContactPoint", ctx)
                    })?;
                }
                fhirbolt_model::r5::types::VirtualServiceDetailAddress::ExtendedContactDetail(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("addressExtendedContactDetail", ctx)
                    })?;
                }
                fhirbolt_model::r5::types::VirtualServiceDetailAddress::Invalid => {
                    return Err(serde::ser::Error::custom("address is invalid"))
                }
            }
        }
        if self.output_json {
            if !self.value.r#additional_info.is_empty() {
                let values = self
                    .value
                    .r#additional_info
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("additionalInfo", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#additional_info
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#additional_info
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_additionalInfo", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#additional_info.is_empty() {
                self.with_context(&self.value.r#additional_info, |ctx| {
                    state.serialize_entry("additionalInfo", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#max_participants.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("maxParticipants", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_maxParticipants", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#max_participants.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("maxParticipants", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#session_key.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sessionKey", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sessionKey", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#session_key.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("sessionKey", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::types::VirtualServiceDetail>,
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
        &Vec<fhirbolt_model::r5::types::VirtualServiceDetail>,
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
        &Vec<Box<fhirbolt_model::r5::types::VirtualServiceDetail>>,
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
        fhirbolt_model::r5::types::VirtualServiceDetail,
    >
{
    type Value = fhirbolt_model::r5::types::VirtualServiceDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::types::VirtualServiceDetail,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::types::VirtualServiceDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VirtualServiceDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::types::VirtualServiceDetail, V::Error>
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
                    #[serde(rename = "channelType")]
                    ChannelType,
                    #[serde(rename = "addressUrl")]
                    AddressUrl,
                    #[serde(rename = "_addressUrl")]
                    AddressUrlPrimitiveElement,
                    #[serde(rename = "addressString")]
                    AddressString,
                    #[serde(rename = "_addressString")]
                    AddressStringPrimitiveElement,
                    #[serde(rename = "addressContactPoint")]
                    AddressContactPoint,
                    #[serde(rename = "addressExtendedContactDetail")]
                    AddressExtendedContactDetail,
                    #[serde(rename = "additionalInfo")]
                    AdditionalInfo,
                    #[serde(rename = "_additionalInfo")]
                    AdditionalInfoPrimitiveElement,
                    #[serde(rename = "maxParticipants")]
                    MaxParticipants,
                    #[serde(rename = "_maxParticipants")]
                    MaxParticipantsPrimitiveElement,
                    #[serde(rename = "sessionKey")]
                    SessionKey,
                    #[serde(rename = "_sessionKey")]
                    SessionKeyPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "channelType",
                            "addressUrl",
                            "addressString",
                            "addressContactPoint",
                            "addressExtendedContactDetail",
                            "additionalInfo",
                            "maxParticipants",
                            "sessionKey",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#channel_type: Option<Box<fhirbolt_model::r5::types::Coding>> = None;
                let mut r#address: Option<fhirbolt_model::r5::types::VirtualServiceDetailAddress> =
                    None;
                let mut r#additional_info: Option<Vec<fhirbolt_model::r5::types::Url>> = None;
                let mut r#max_participants: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#session_key: Option<fhirbolt_model::r5::types::String> = None;
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
                        Field::ChannelType => {
                            if r#channel_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelType"));
                            }
                            r#channel_type = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Coding>>(),
                            )?);
                        }
                        Field::AddressUrl => {
                            if self.0.from_json {
                                let r#enum = r#address.get_or_insert(
                                    fhirbolt_model::r5::types::VirtualServiceDetailAddress::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::types::VirtualServiceDetailAddress::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "addressUrl",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("address[x]"));
                                }
                            } else {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addressUrl"));
                                }
                                r#address = Some(
                                    fhirbolt_model::r5::types::VirtualServiceDetailAddress::Url(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r5::types::Url>>(),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::AddressUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#address.get_or_insert(
                                    fhirbolt_model::r5::types::VirtualServiceDetailAddress::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r5::types::VirtualServiceDetailAddress::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_addressUrl",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_address[x]"));
                                }
                            } else {
                                return unknown_field_error("addressUrl");
                            }
                        }
                        Field::AddressString => {
                            if self.0.from_json {
                                let r#enum = r#address.get_or_insert(
                                    fhirbolt_model::r5::types::VirtualServiceDetailAddress::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r5 :: types :: VirtualServiceDetailAddress :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("addressString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("address[x]")) ; }
                            } else {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addressString"));
                                }
                                r#address = Some (fhirbolt_model :: r5 :: types :: VirtualServiceDetailAddress :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::AddressStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#address.get_or_insert(
                                    fhirbolt_model::r5::types::VirtualServiceDetailAddress::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r5 :: types :: VirtualServiceDetailAddress :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_addressString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_address[x]")) ; }
                            } else {
                                return unknown_field_error("addressString");
                            }
                        }
                        Field::AddressContactPoint => {
                            if r#address.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "addressContactPoint",
                                ));
                            }
                            r#address = Some (fhirbolt_model :: r5 :: types :: VirtualServiceDetailAddress :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::AddressExtendedContactDetail => {
                            if r#address.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "addressExtendedContactDetail",
                                ));
                            }
                            r#address = Some (fhirbolt_model :: r5 :: types :: VirtualServiceDetailAddress :: ExtendedContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: ExtendedContactDetail > > ()) ?)) ;
                        }
                        Field::AdditionalInfo => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#additional_info.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalInfo",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#additional_info.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Url>(),
                                )?);
                            }
                        }
                        Field::AdditionalInfoPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#additional_info.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_additionalInfo",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("additionalInfo");
                            }
                        }
                        Field::MaxParticipants => {
                            if self.0.from_json {
                                let some = r#max_participants.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxParticipants",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#max_participants.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxParticipants",
                                    ));
                                }
                                r#max_participants = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::PositiveInt>(),
                                )?);
                            }
                        }
                        Field::MaxParticipantsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#max_participants.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_maxParticipants",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("maxParticipants");
                            }
                        }
                        Field::SessionKey => {
                            if self.0.from_json {
                                let some = r#session_key.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sessionKey"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#session_key.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sessionKey"));
                                }
                                r#session_key = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::SessionKeyPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#session_key.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sessionKey"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sessionKey");
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
                Ok(fhirbolt_model::r5::types::VirtualServiceDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#channel_type,
                    r#address,
                    r#additional_info: r#additional_info.unwrap_or(vec![]),
                    r#max_participants,
                    r#session_key,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::types::VirtualServiceDetail>,
    >
{
    type Value = Box<fhirbolt_model::r5::types::VirtualServiceDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::types::VirtualServiceDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::types::VirtualServiceDetail>,
    >
{
    type Value = Vec<fhirbolt_model::r5::types::VirtualServiceDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::types::VirtualServiceDetail>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::types::VirtualServiceDetail>;
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
                        .transmute::<fhirbolt_model::r5::types::VirtualServiceDetail>(),
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
        Vec<Box<fhirbolt_model::r5::types::VirtualServiceDetail>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::types::VirtualServiceDetail>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::types::VirtualServiceDetail>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::types::VirtualServiceDetail>>;
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
                        .transmute::<Box<fhirbolt_model::r5::types::VirtualServiceDetail>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
