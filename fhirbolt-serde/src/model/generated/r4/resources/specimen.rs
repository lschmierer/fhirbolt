// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::resources::SpecimenCollection;
impl serde::ser::Serialize for SerializationContext<&SpecimenCollection> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Specimen.collection", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#collector.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("collector", ctx)));
        }
        {
            use fhirbolt_model::r4::resources::SpecimenCollectionCollected as _Enum;
            if let Some(some) = self.value.r#collected.as_ref() {
                match some {
                    _Enum::DateTime(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("collectedDateTime", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_collectedDateTime", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("collectedDateTime", ctx)));
                        }
                    }
                    _Enum::Period(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("collectedPeriod", ctx)));
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("collected is invalid"))
                    }
                }
            }
        }
        if let Some(some) = self.value.r#duration.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("duration", ctx)));
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("quantity", ctx)));
        }
        if let Some(some) = self.value.r#method.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("method", ctx)));
        }
        if let Some(some) = self.value.r#body_site.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("bodySite", ctx)));
        }
        {
            use fhirbolt_model::r4::resources::SpecimenCollectionFastingStatus as _Enum;
            if let Some(some) = self.value.r#fasting_status.as_ref() {
                match some {
                    _Enum::CodeableConcept(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("fastingStatusCodeableConcept", ctx)));
                    }
                    _Enum::Duration(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("fastingStatusDuration", ctx)));
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("fasting_status is invalid"))
                    }
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SpecimenCollection>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SpecimenCollection>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SpecimenCollection> {
    type Value = SpecimenCollection;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SpecimenCollection>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenCollection;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenCollection")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenCollection, V::Error>
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
                    #[serde(rename = "collector")]
                    Collector,
                    #[serde(rename = "collectedDateTime")]
                    CollectedDateTime,
                    #[serde(rename = "_collectedDateTime")]
                    CollectedDateTimePrimitiveElement,
                    #[serde(rename = "collectedPeriod")]
                    CollectedPeriod,
                    #[serde(rename = "duration")]
                    Duration,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "bodySite")]
                    BodySite,
                    #[serde(rename = "fastingStatusCodeableConcept")]
                    FastingStatusCodeableConcept,
                    #[serde(rename = "fastingStatusDuration")]
                    FastingStatusDuration,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "collector",
                            "collectedDateTime",
                            "collectedPeriod",
                            "duration",
                            "quantity",
                            "method",
                            "bodySite",
                            "fastingStatusCodeableConcept",
                            "fastingStatusDuration",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#collector: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#collected: Option<
                    fhirbolt_model::r4::resources::SpecimenCollectionCollected,
                > = None;
                let mut r#duration: Option<Box<fhirbolt_model::r4::types::Duration>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#method: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#body_site: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#fasting_status: Option<
                    fhirbolt_model::r4::resources::SpecimenCollectionFastingStatus,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Collector => {
                            if r#collector.is_some() {
                                return Err(serde::de::Error::duplicate_field("collector"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#collector = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::CollectedDateTime => {
                            use fhirbolt_model::r4::resources::SpecimenCollectionCollected as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#collected.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "collectedDateTime",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("collected[x]"));
                                }
                            } else {
                                if r#collected.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "collectedDateTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::DateTime,
                                > = self.0.transmute();
                                r#collected = Some(_Enum::DateTime(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::CollectedDateTimePrimitiveElement => {
                            use fhirbolt_model::r4::resources::SpecimenCollectionCollected as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#collected.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_collectedDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_collected[x]"));
                                }
                            } else {
                                return unknown_field_error("collectedDateTime");
                            }
                        }
                        Field::CollectedPeriod => {
                            use fhirbolt_model::r4::resources::SpecimenCollectionCollected as _Enum;
                            if r#collected.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectedPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Period>,
                            > = self.0.transmute();
                            r#collected = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Duration => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Duration>,
                            > = self.0.transmute();
                            r#duration = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#method = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::BodySite => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#body_site = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::FastingStatusCodeableConcept => {
                            use fhirbolt_model::r4::resources::SpecimenCollectionFastingStatus as _Enum;
                            if r#fasting_status.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fastingStatusCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#fasting_status = Some(_Enum::CodeableConcept(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::FastingStatusDuration => {
                            use fhirbolt_model::r4::resources::SpecimenCollectionFastingStatus as _Enum;
                            if r#fasting_status.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fastingStatusDuration",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Duration>,
                            > = self.0.transmute();
                            r#fasting_status = Some(_Enum::Duration(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(SpecimenCollection {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#collector,
                    r#collected,
                    r#duration,
                    r#quantity,
                    r#method,
                    r#body_site,
                    r#fasting_status,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<SpecimenCollection>> {
    type Value = Box<SpecimenCollection>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenCollection>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<SpecimenCollection>> {
    type Value = Vec<SpecimenCollection>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SpecimenCollection>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenCollection>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SpecimenCollection> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::SpecimenProcessing;
impl serde::ser::Serialize for SerializationContext<&SpecimenProcessing> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Specimen.processing", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("description", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_description", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("description", ctx)));
        }
        if let Some(some) = self.value.r#procedure.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("procedure", ctx)));
        }
        if !self.value.r#additive.is_empty() {
            tri!(self.with_context(&self.value.r#additive, |ctx| state
                .serialize_entry("additive", ctx)));
        }
        {
            use fhirbolt_model::r4::resources::SpecimenProcessingTime as _Enum;
            if let Some(some) = self.value.r#time.as_ref() {
                match some {
                    _Enum::DateTime(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("timeDateTime", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_timeDateTime", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("timeDateTime", ctx)));
                        }
                    }
                    _Enum::Period(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("timePeriod", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("time is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SpecimenProcessing>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SpecimenProcessing>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SpecimenProcessing> {
    type Value = SpecimenProcessing;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SpecimenProcessing>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenProcessing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenProcessing")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenProcessing, V::Error>
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
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "procedure")]
                    Procedure,
                    #[serde(rename = "additive")]
                    Additive,
                    #[serde(rename = "timeDateTime")]
                    TimeDateTime,
                    #[serde(rename = "_timeDateTime")]
                    TimeDateTimePrimitiveElement,
                    #[serde(rename = "timePeriod")]
                    TimePeriod,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "description",
                            "procedure",
                            "additive",
                            "timeDateTime",
                            "timePeriod",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#description: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#procedure: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#additive: Option<Vec<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#time: Option<fhirbolt_model::r4::resources::SpecimenProcessingTime> =
                    None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#description =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Procedure => {
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field("procedure"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#procedure = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Additive => {
                            if self.0.from == crate::context::Format::Json {
                                if r#additive.is_some() {
                                    return Err(serde::de::Error::duplicate_field("additive"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Reference>,
                                > = self.0.transmute();
                                r#additive = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#additive.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::TimeDateTime => {
                            use fhirbolt_model::r4::resources::SpecimenProcessingTime as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#time.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timeDateTime",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("time[x]"));
                                }
                            } else {
                                if r#time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timeDateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::DateTime,
                                > = self.0.transmute();
                                r#time = Some(_Enum::DateTime(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::TimeDateTimePrimitiveElement => {
                            use fhirbolt_model::r4::resources::SpecimenProcessingTime as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#time.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timeDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_time[x]"));
                                }
                            } else {
                                return unknown_field_error("timeDateTime");
                            }
                        }
                        Field::TimePeriod => {
                            use fhirbolt_model::r4::resources::SpecimenProcessingTime as _Enum;
                            if r#time.is_some() {
                                return Err(serde::de::Error::duplicate_field("timePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Period>,
                            > = self.0.transmute();
                            r#time = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(SpecimenProcessing {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#description,
                    r#procedure,
                    r#additive: r#additive.unwrap_or(vec![]),
                    r#time,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<SpecimenProcessing>> {
    type Value = Box<SpecimenProcessing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenProcessing>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<SpecimenProcessing>> {
    type Value = Vec<SpecimenProcessing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SpecimenProcessing>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenProcessing>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SpecimenProcessing> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::SpecimenContainer;
impl serde::ser::Serialize for SerializationContext<&SpecimenContainer> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Specimen.container", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if !self.value.r#identifier.is_empty() {
            tri!(self.with_context(&self.value.r#identifier, |ctx| state
                .serialize_entry("identifier", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("description", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_description", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("description", ctx)));
        }
        if let Some(some) = self.value.r#type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("type", ctx)));
        }
        if let Some(some) = self.value.r#capacity.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("capacity", ctx)));
        }
        if let Some(some) = self.value.r#specimen_quantity.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("specimenQuantity", ctx)));
        }
        {
            use fhirbolt_model::r4::resources::SpecimenContainerAdditive as _Enum;
            if let Some(some) = self.value.r#additive.as_ref() {
                match some {
                    _Enum::CodeableConcept(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("additiveCodeableConcept", ctx)));
                    }
                    _Enum::Reference(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("additiveReference", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("additive is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SpecimenContainer>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SpecimenContainer>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SpecimenContainer> {
    type Value = SpecimenContainer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SpecimenContainer>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenContainer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenContainer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenContainer, V::Error>
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
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "capacity")]
                    Capacity,
                    #[serde(rename = "specimenQuantity")]
                    SpecimenQuantity,
                    #[serde(rename = "additiveCodeableConcept")]
                    AdditiveCodeableConcept,
                    #[serde(rename = "additiveReference")]
                    AdditiveReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "identifier",
                            "description",
                            "type",
                            "capacity",
                            "specimenQuantity",
                            "additiveCodeableConcept",
                            "additiveReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#description: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#capacity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#specimen_quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> =
                    None;
                let mut r#additive: Option<
                    fhirbolt_model::r4::resources::SpecimenContainerAdditive,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Identifier,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#description =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Capacity => {
                            if r#capacity.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#capacity = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::SpecimenQuantity => {
                            if r#specimen_quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("specimenQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#specimen_quantity =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::AdditiveCodeableConcept => {
                            use fhirbolt_model::r4::resources::SpecimenContainerAdditive as _Enum;
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additiveCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#additive = Some(_Enum::CodeableConcept(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::AdditiveReference => {
                            use fhirbolt_model::r4::resources::SpecimenContainerAdditive as _Enum;
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field("additiveReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#additive = Some(_Enum::Reference(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(SpecimenContainer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#description,
                    r#type,
                    r#capacity,
                    r#specimen_quantity,
                    r#additive,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<SpecimenContainer>> {
    type Value = Box<SpecimenContainer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenContainer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<SpecimenContainer>> {
    type Value = Vec<SpecimenContainer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SpecimenContainer>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenContainer>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SpecimenContainer> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::Specimen;
impl crate::Resource for Specimen {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for SerializationContext<&Specimen> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Specimen", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        tri!(state.serialize_entry("resourceType", "Specimen"));
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("id", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self
                        .with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("id", ctx)));
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("meta", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("implicitRules", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_implicitRules", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("language", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_language", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("language", ctx)));
        }
        if let Some(some) = self.value.r#text.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("text", ctx)));
        }
        if !self.value.r#contained.is_empty() {
            tri!(self.with_context(&self.value.r#contained, |ctx| state
                .serialize_entry("contained", ctx)));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if !self.value.r#identifier.is_empty() {
            tri!(self.with_context(&self.value.r#identifier, |ctx| state
                .serialize_entry("identifier", ctx)));
        }
        if let Some(some) = self.value.r#accession_identifier.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("accessionIdentifier", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#status.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("status", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_status", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#status.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("status", ctx)));
        }
        if let Some(some) = self.value.r#type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("type", ctx)));
        }
        if let Some(some) = self.value.r#subject.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("subject", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#received_time.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("receivedTime", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_receivedTime", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#received_time.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("receivedTime", ctx)));
        }
        if !self.value.r#parent.is_empty() {
            tri!(self.with_context(&self.value.r#parent, |ctx| state
                .serialize_entry("parent", ctx)));
        }
        if !self.value.r#request.is_empty() {
            tri!(self.with_context(&self.value.r#request, |ctx| state
                .serialize_entry("request", ctx)));
        }
        if let Some(some) = self.value.r#collection.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("collection", ctx)));
        }
        if !self.value.r#processing.is_empty() {
            tri!(self.with_context(&self.value.r#processing, |ctx| state
                .serialize_entry("processing", ctx)));
        }
        if !self.value.r#container.is_empty() {
            tri!(self.with_context(&self.value.r#container, |ctx| state
                .serialize_entry("container", ctx)));
        }
        if !self.value.r#condition.is_empty() {
            tri!(self.with_context(&self.value.r#condition, |ctx| state
                .serialize_entry("condition", ctx)));
        }
        if !self.value.r#note.is_empty() {
            tri!(self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Specimen>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Specimen>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Specimen> {
    type Value = Specimen;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Specimen> {
    type Value = Specimen;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Specimen>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Specimen;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Specimen")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Specimen, V::Error>
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
                    #[serde(rename = "accessionIdentifier")]
                    AccessionIdentifier,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "receivedTime")]
                    ReceivedTime,
                    #[serde(rename = "_receivedTime")]
                    ReceivedTimePrimitiveElement,
                    #[serde(rename = "parent")]
                    Parent,
                    #[serde(rename = "request")]
                    Request,
                    #[serde(rename = "collection")]
                    Collection,
                    #[serde(rename = "processing")]
                    Processing,
                    #[serde(rename = "container")]
                    Container,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "note")]
                    Note,
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
                            "accessionIdentifier",
                            "status",
                            "type",
                            "subject",
                            "receivedTime",
                            "parent",
                            "request",
                            "collection",
                            "processing",
                            "container",
                            "condition",
                            "note",
                        ],
                    ))
                }
                let mut r#id: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#accession_identifier: Option<Box<fhirbolt_model::r4::types::Identifier>> =
                    None;
                let mut r#status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#received_time: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#parent: Option<Vec<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#request: Option<Vec<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#collection: Option<fhirbolt_model::r4::resources::SpecimenCollection> =
                    None;
                let mut r#processing: Option<
                    Vec<fhirbolt_model::r4::resources::SpecimenProcessing>,
                > = None;
                let mut r#container: Option<Vec<fhirbolt_model::r4::resources::SpecimenContainer>> =
                    None;
                let mut r#condition: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r4::types::Annotation>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = tri!(map_access.next_value());
                            if value != "Specimen" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Specimen",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Id,
                                > = self.0.transmute();
                                r#id = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                                Box<fhirbolt_model::r4::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#language = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                                Box<fhirbolt_model::r4::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::Resource>,
                                > = self.0.transmute();
                                r#contained =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::Resource,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Identifier,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::AccessionIdentifier => {
                            if r#accession_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "accessionIdentifier",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Identifier>,
                            > = self.0.transmute();
                            r#accession_identifier =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#status = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#subject = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ReceivedTime => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#received_time.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("receivedTime"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#received_time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("receivedTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::DateTime,
                                > = self.0.transmute();
                                r#received_time =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ReceivedTimePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#received_time.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_receivedTime"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("receivedTime");
                            }
                        }
                        Field::Parent => {
                            if self.0.from == crate::context::Format::Json {
                                if r#parent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parent"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Reference>,
                                > = self.0.transmute();
                                r#parent = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#parent.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Request => {
                            if self.0.from == crate::context::Format::Json {
                                if r#request.is_some() {
                                    return Err(serde::de::Error::duplicate_field("request"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Reference>,
                                > = self.0.transmute();
                                r#request = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#request.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Collection => {
                            if r#collection.is_some() {
                                return Err(serde::de::Error::duplicate_field("collection"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4::resources::SpecimenCollection,
                            > = self.0.transmute();
                            r#collection = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Processing => {
                            if self.0.from == crate::context::Format::Json {
                                if r#processing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processing"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::SpecimenProcessing>,
                                > = self.0.transmute();
                                r#processing =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#processing.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::SpecimenProcessing,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Container => {
                            if self.0.from == crate::context::Format::Json {
                                if r#container.is_some() {
                                    return Err(serde::de::Error::duplicate_field("container"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::SpecimenContainer>,
                                > = self.0.transmute();
                                r#container =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#container.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::SpecimenContainer,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Condition => {
                            if self.0.from == crate::context::Format::Json {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#condition =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#condition.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Annotation,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                Ok(Specimen {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#accession_identifier,
                    r#status,
                    r#type,
                    r#subject,
                    r#received_time,
                    r#parent: r#parent.unwrap_or(vec![]),
                    r#request: r#request.unwrap_or(vec![]),
                    r#collection,
                    r#processing: r#processing.unwrap_or(vec![]),
                    r#container: r#container.unwrap_or(vec![]),
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Specimen>> {
    type Value = Box<Specimen>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Specimen>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Specimen>> {
    type Value = Vec<Specimen>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Specimen>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Specimen>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Specimen> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
