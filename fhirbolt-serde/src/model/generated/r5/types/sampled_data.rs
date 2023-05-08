// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::types::SampledData;
impl serde::ser::Serialize for SerializationContext<&SampledData> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SampledData", field
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
        if self.value.r#origin.id.as_deref() == Some("$invalid") {
            return missing_field_error("origin");
        } else {
            self.with_context(&self.value.r#origin, |ctx| {
                state.serialize_entry("origin", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#interval.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("interval", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_interval", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#interval.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("interval", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#interval_unit.id.as_deref() == Some("$invalid") {
                return missing_field_error("intervalUnit");
            }
            if let Some(some) = self.value.r#interval_unit.value.as_ref().map(Ok) {
                state.serialize_entry("intervalUnit", &some?)?;
            }
            if self.value.r#interval_unit.id.is_some()
                || !self.value.r#interval_unit.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#interval_unit.id.as_ref(),
                    extension: &self.value.r#interval_unit.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_intervalUnit", ctx)
                })?;
            }
        } else if self.value.r#interval_unit.id.as_deref() == Some("$invalid") {
            return missing_field_error("intervalUnit");
        } else {
            self.with_context(&self.value.r#interval_unit, |ctx| {
                state.serialize_entry("intervalUnit", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#factor.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("factor", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_factor", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#factor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("factor", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#lower_limit.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("lowerLimit", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lowerLimit", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#lower_limit.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("lowerLimit", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#upper_limit.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("upperLimit", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_upperLimit", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#upper_limit.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("upperLimit", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#dimensions.id.as_deref() == Some("$invalid") {
                return missing_field_error("dimensions");
            }
            if let Some(some) = self.value.r#dimensions.value.as_ref().map(Ok) {
                state.serialize_entry("dimensions", &some?)?;
            }
            if self.value.r#dimensions.id.is_some() || !self.value.r#dimensions.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#dimensions.id.as_ref(),
                    extension: &self.value.r#dimensions.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_dimensions", ctx)
                })?;
            }
        } else if self.value.r#dimensions.id.as_deref() == Some("$invalid") {
            return missing_field_error("dimensions");
        } else {
            self.with_context(&self.value.r#dimensions, |ctx| {
                state.serialize_entry("dimensions", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#code_map.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("codeMap", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_codeMap", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#code_map.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("codeMap", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#offsets.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("offsets", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_offsets", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#offsets.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("offsets", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#data.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("data", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_data", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#data.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("data", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SampledData>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SampledData>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SampledData> {
    type Value = SampledData;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SampledData>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SampledData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SampledData")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SampledData, V::Error>
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
                    #[serde(rename = "origin")]
                    Origin,
                    #[serde(rename = "interval")]
                    Interval,
                    #[serde(rename = "_interval")]
                    IntervalPrimitiveElement,
                    #[serde(rename = "intervalUnit")]
                    IntervalUnit,
                    #[serde(rename = "_intervalUnit")]
                    IntervalUnitPrimitiveElement,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "lowerLimit")]
                    LowerLimit,
                    #[serde(rename = "_lowerLimit")]
                    LowerLimitPrimitiveElement,
                    #[serde(rename = "upperLimit")]
                    UpperLimit,
                    #[serde(rename = "_upperLimit")]
                    UpperLimitPrimitiveElement,
                    #[serde(rename = "dimensions")]
                    Dimensions,
                    #[serde(rename = "_dimensions")]
                    DimensionsPrimitiveElement,
                    #[serde(rename = "codeMap")]
                    CodeMap,
                    #[serde(rename = "_codeMap")]
                    CodeMapPrimitiveElement,
                    #[serde(rename = "offsets")]
                    Offsets,
                    #[serde(rename = "_offsets")]
                    OffsetsPrimitiveElement,
                    #[serde(rename = "data")]
                    Data,
                    #[serde(rename = "_data")]
                    DataPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "origin",
                            "interval",
                            "intervalUnit",
                            "factor",
                            "lowerLimit",
                            "upperLimit",
                            "dimensions",
                            "codeMap",
                            "offsets",
                            "data",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#origin: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#interval: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#interval_unit: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#lower_limit: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#upper_limit: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#dimensions: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#code_map: Option<fhirbolt_model::r5::types::Canonical> = None;
                let mut r#offsets: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#data: Option<fhirbolt_model::r5::types::String> = None;
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
                        Field::Origin => {
                            if r#origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#origin = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Interval => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#interval.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("interval"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#interval.is_some() {
                                    return Err(serde::de::Error::duplicate_field("interval"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#interval = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IntervalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#interval.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_interval"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("interval");
                            }
                        }
                        Field::IntervalUnit => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#interval_unit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intervalUnit"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#interval_unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intervalUnit"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#interval_unit = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IntervalUnitPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#interval_unit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_intervalUnit"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("intervalUnit");
                            }
                        }
                        Field::Factor => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#factor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("factor");
                            }
                        }
                        Field::LowerLimit => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#lower_limit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lowerLimit"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#lower_limit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lowerLimit"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#lower_limit = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LowerLimitPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#lower_limit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lowerLimit"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lowerLimit");
                            }
                        }
                        Field::UpperLimit => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#upper_limit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("upperLimit"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#upper_limit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("upperLimit"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#upper_limit = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::UpperLimitPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#upper_limit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_upperLimit"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("upperLimit");
                            }
                        }
                        Field::Dimensions => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#dimensions.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dimensions"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#dimensions.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dimensions"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#dimensions = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DimensionsPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#dimensions.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_dimensions"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("dimensions");
                            }
                        }
                        Field::CodeMap => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#code_map.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("codeMap"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#code_map.is_some() {
                                    return Err(serde::de::Error::duplicate_field("codeMap"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                r#code_map = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CodeMapPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#code_map.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_codeMap"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("codeMap");
                            }
                        }
                        Field::Offsets => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#offsets.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offsets"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#offsets.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offsets"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#offsets = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OffsetsPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#offsets.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_offsets"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("offsets");
                            }
                        }
                        Field::Data => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#data.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#data.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#data = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DataPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#data.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_data"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("data");
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
                Ok(SampledData {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#origin: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#origin.unwrap_or(Default::default())
                    } else {
                        r#origin.ok_or(serde::de::Error::missing_field("origin"))?
                    },
                    r#interval,
                    r#interval_unit: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#interval_unit.unwrap_or(Default::default())
                    } else {
                        r#interval_unit.ok_or(serde::de::Error::missing_field("intervalUnit"))?
                    },
                    r#factor,
                    r#lower_limit,
                    r#upper_limit,
                    r#dimensions: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#dimensions.unwrap_or(Default::default())
                    } else {
                        r#dimensions.ok_or(serde::de::Error::missing_field("dimensions"))?
                    },
                    r#code_map,
                    r#offsets,
                    r#data,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<SampledData>> {
    type Value = Box<SampledData>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SampledData>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<SampledData>> {
    type Value = Vec<SampledData>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SampledData>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SampledData>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SampledData> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
