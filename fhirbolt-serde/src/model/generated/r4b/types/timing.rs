// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::types::TimingRepeat;
impl serde::ser::Serialize for SerializationContext<&TimingRepeat> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Timing.repeat", field
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
        {
            use fhirbolt_model::r4b::types::TimingRepeatBounds as _Enum;
            if let Some(some) = self.value.r#bounds.as_ref() {
                match some {
                    _Enum::Duration(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("boundsDuration", ctx)));
                    }
                    _Enum::Range(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("boundsRange", ctx)));
                    }
                    _Enum::Period(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("boundsPeriod", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("bounds is invalid")),
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#count.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("count", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_count", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#count.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("count", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#count_max.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("countMax", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_countMax", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#count_max.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("countMax", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#duration.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    tri!(state.serialize_entry("duration", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_duration", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#duration.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("duration", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#duration_max.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    tri!(state.serialize_entry("durationMax", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_durationMax", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#duration_max.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("durationMax", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#duration_unit.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("durationUnit", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_durationUnit", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#duration_unit.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("durationUnit", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#frequency.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("frequency", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_frequency", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#frequency.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("frequency", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#frequency_max.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("frequencyMax", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_frequencyMax", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#frequency_max.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("frequencyMax", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#period.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    tri!(state.serialize_entry("period", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_period", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#period.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("period", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#period_max.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    tri!(state.serialize_entry("periodMax", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_periodMax", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#period_max.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("periodMax", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#period_unit.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("periodUnit", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_periodUnit", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#period_unit.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("periodUnit", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#day_of_week.is_empty() {
                let values = tri!(self
                    .value
                    .r#day_of_week
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("dayOfWeek", &values));
                }
                let requires_elements = self
                    .value
                    .r#day_of_week
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#day_of_week
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_dayOfWeek", ctx)));
                }
            }
        } else if !self.value.r#day_of_week.is_empty() {
            tri!(self.with_context(&self.value.r#day_of_week, |ctx| state
                .serialize_entry("dayOfWeek", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#time_of_day.is_empty() {
                let values = tri!(self
                    .value
                    .r#time_of_day
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("timeOfDay", &values));
                }
                let requires_elements = self
                    .value
                    .r#time_of_day
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#time_of_day
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_timeOfDay", ctx)));
                }
            }
        } else if !self.value.r#time_of_day.is_empty() {
            tri!(self.with_context(&self.value.r#time_of_day, |ctx| state
                .serialize_entry("timeOfDay", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#when.is_empty() {
                let values = tri!(self
                    .value
                    .r#when
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("when", &values));
                }
                let requires_elements = self
                    .value
                    .r#when
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#when
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_when", ctx)));
                }
            }
        } else if !self.value.r#when.is_empty() {
            tri!(self.with_context(&self.value.r#when, |ctx| state.serialize_entry("when", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#offset.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("offset", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_offset", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#offset.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("offset", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TimingRepeat>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TimingRepeat>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TimingRepeat> {
    type Value = TimingRepeat;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TimingRepeat>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TimingRepeat;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TimingRepeat")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TimingRepeat, V::Error>
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
                    #[serde(rename = "boundsDuration")]
                    BoundsDuration,
                    #[serde(rename = "boundsRange")]
                    BoundsRange,
                    #[serde(rename = "boundsPeriod")]
                    BoundsPeriod,
                    #[serde(rename = "count")]
                    Count,
                    #[serde(rename = "_count")]
                    CountPrimitiveElement,
                    #[serde(rename = "countMax")]
                    CountMax,
                    #[serde(rename = "_countMax")]
                    CountMaxPrimitiveElement,
                    #[serde(rename = "duration")]
                    Duration,
                    #[serde(rename = "_duration")]
                    DurationPrimitiveElement,
                    #[serde(rename = "durationMax")]
                    DurationMax,
                    #[serde(rename = "_durationMax")]
                    DurationMaxPrimitiveElement,
                    #[serde(rename = "durationUnit")]
                    DurationUnit,
                    #[serde(rename = "_durationUnit")]
                    DurationUnitPrimitiveElement,
                    #[serde(rename = "frequency")]
                    Frequency,
                    #[serde(rename = "_frequency")]
                    FrequencyPrimitiveElement,
                    #[serde(rename = "frequencyMax")]
                    FrequencyMax,
                    #[serde(rename = "_frequencyMax")]
                    FrequencyMaxPrimitiveElement,
                    #[serde(rename = "period")]
                    Period,
                    #[serde(rename = "_period")]
                    PeriodPrimitiveElement,
                    #[serde(rename = "periodMax")]
                    PeriodMax,
                    #[serde(rename = "_periodMax")]
                    PeriodMaxPrimitiveElement,
                    #[serde(rename = "periodUnit")]
                    PeriodUnit,
                    #[serde(rename = "_periodUnit")]
                    PeriodUnitPrimitiveElement,
                    #[serde(rename = "dayOfWeek")]
                    DayOfWeek,
                    #[serde(rename = "_dayOfWeek")]
                    DayOfWeekPrimitiveElement,
                    #[serde(rename = "timeOfDay")]
                    TimeOfDay,
                    #[serde(rename = "_timeOfDay")]
                    TimeOfDayPrimitiveElement,
                    #[serde(rename = "when")]
                    When,
                    #[serde(rename = "_when")]
                    WhenPrimitiveElement,
                    #[serde(rename = "offset")]
                    Offset,
                    #[serde(rename = "_offset")]
                    OffsetPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "boundsDuration",
                            "boundsRange",
                            "boundsPeriod",
                            "count",
                            "countMax",
                            "duration",
                            "durationMax",
                            "durationUnit",
                            "frequency",
                            "frequencyMax",
                            "period",
                            "periodMax",
                            "periodUnit",
                            "dayOfWeek",
                            "timeOfDay",
                            "when",
                            "offset",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#bounds: Option<fhirbolt_model::r4b::types::TimingRepeatBounds> = None;
                let mut r#count: Option<fhirbolt_model::r4b::types::PositiveInt> = None;
                let mut r#count_max: Option<fhirbolt_model::r4b::types::PositiveInt> = None;
                let mut r#duration: Option<fhirbolt_model::r4b::types::Decimal> = None;
                let mut r#duration_max: Option<fhirbolt_model::r4b::types::Decimal> = None;
                let mut r#duration_unit: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#frequency: Option<fhirbolt_model::r4b::types::PositiveInt> = None;
                let mut r#frequency_max: Option<fhirbolt_model::r4b::types::PositiveInt> = None;
                let mut r#period: Option<fhirbolt_model::r4b::types::Decimal> = None;
                let mut r#period_max: Option<fhirbolt_model::r4b::types::Decimal> = None;
                let mut r#period_unit: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#day_of_week: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#time_of_day: Option<Vec<fhirbolt_model::r4b::types::Time>> = None;
                let mut r#when: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#offset: Option<fhirbolt_model::r4b::types::UnsignedInt> = None;
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
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::BoundsDuration => {
                            use fhirbolt_model::r4b::types::TimingRepeatBounds as _Enum;
                            if r#bounds.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundsDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Duration>,
                            > = self.0.transmute();
                            r#bounds = Some(_Enum::Duration(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::BoundsRange => {
                            use fhirbolt_model::r4b::types::TimingRepeatBounds as _Enum;
                            if r#bounds.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundsRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Range>,
                            > = self.0.transmute();
                            r#bounds = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::BoundsPeriod => {
                            use fhirbolt_model::r4b::types::TimingRepeatBounds as _Enum;
                            if r#bounds.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundsPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#bounds = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Count => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#count.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("count"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#count.is_some() {
                                    return Err(serde::de::Error::duplicate_field("count"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::PositiveInt,
                                > = self.0.transmute();
                                r#count = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::CountPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#count.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_count"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("count");
                            }
                        }
                        Field::CountMax => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#count_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("countMax"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#count_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("countMax"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::PositiveInt,
                                > = self.0.transmute();
                                r#count_max =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::CountMaxPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#count_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_countMax"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("countMax");
                            }
                        }
                        Field::Duration => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Decimal,
                                > = self.0.transmute();
                                r#duration = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DurationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_duration"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("duration");
                            }
                        }
                        Field::DurationMax => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationMax"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#duration_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationMax"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Decimal,
                                > = self.0.transmute();
                                r#duration_max =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DurationMaxPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_durationMax"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("durationMax");
                            }
                        }
                        Field::DurationUnit => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration_unit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationUnit"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#duration_unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationUnit"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#duration_unit =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DurationUnitPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration_unit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_durationUnit"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("durationUnit");
                            }
                        }
                        Field::Frequency => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#frequency.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequency"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#frequency.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequency"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::PositiveInt,
                                > = self.0.transmute();
                                r#frequency =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FrequencyPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#frequency.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frequency"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("frequency");
                            }
                        }
                        Field::FrequencyMax => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#frequency_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequencyMax"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#frequency_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequencyMax"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::PositiveInt,
                                > = self.0.transmute();
                                r#frequency_max =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FrequencyMaxPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#frequency_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frequencyMax"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("frequencyMax");
                            }
                        }
                        Field::Period => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#period.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Decimal,
                                > = self.0.transmute();
                                r#period = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PeriodPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#period.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_period"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("period");
                            }
                        }
                        Field::PeriodMax => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#period_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodMax"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#period_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodMax"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Decimal,
                                > = self.0.transmute();
                                r#period_max =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PeriodMaxPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#period_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_periodMax"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("periodMax");
                            }
                        }
                        Field::PeriodUnit => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#period_unit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodUnit"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#period_unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodUnit"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#period_unit =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PeriodUnitPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#period_unit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_periodUnit"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("periodUnit");
                            }
                        }
                        Field::DayOfWeek => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#day_of_week.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("dayOfWeek"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#day_of_week.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DayOfWeekPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#day_of_week.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_dayOfWeek"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("dayOfWeek");
                            }
                        }
                        Field::TimeOfDay => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#time_of_day.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("timeOfDay"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#time_of_day.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Time,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::TimeOfDayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#time_of_day.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_timeOfDay"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("timeOfDay");
                            }
                        }
                        Field::When => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#when.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("when"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#when.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::WhenPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#when.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_when"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("when");
                            }
                        }
                        Field::Offset => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#offset.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offset"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#offset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offset"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::UnsignedInt,
                                > = self.0.transmute();
                                r#offset = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::OffsetPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#offset.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_offset"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("offset");
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
                Ok(TimingRepeat {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#bounds,
                    r#count,
                    r#count_max,
                    r#duration,
                    r#duration_max,
                    r#duration_unit,
                    r#frequency,
                    r#frequency_max,
                    r#period,
                    r#period_max,
                    r#period_unit,
                    r#day_of_week: r#day_of_week.unwrap_or(vec![]),
                    r#time_of_day: r#time_of_day.unwrap_or(vec![]),
                    r#when: r#when.unwrap_or(vec![]),
                    r#offset,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TimingRepeat>> {
    type Value = Box<TimingRepeat>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TimingRepeat>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TimingRepeat>> {
    type Value = Vec<TimingRepeat>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TimingRepeat>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TimingRepeat>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TimingRepeat> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::Timing;
impl serde::ser::Serialize for SerializationContext<&Timing> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Timing", field
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
            if !self.value.r#event.is_empty() {
                let values = tri!(self
                    .value
                    .r#event
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("event", &values));
                }
                let requires_elements = self
                    .value
                    .r#event
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#event
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_event", ctx)));
                }
            }
        } else if !self.value.r#event.is_empty() {
            tri!(self.with_context(&self.value.r#event, |ctx| state
                .serialize_entry("event", ctx)));
        }
        if let Some(some) = self.value.r#repeat.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("repeat", ctx)));
        }
        if let Some(some) = self.value.r#code.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("code", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Timing>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Timing>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Timing> {
    type Value = Timing;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Timing>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Timing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Timing")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Timing, V::Error>
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
                    #[serde(rename = "event")]
                    Event,
                    #[serde(rename = "_event")]
                    EventPrimitiveElement,
                    #[serde(rename = "repeat")]
                    Repeat,
                    #[serde(rename = "code")]
                    Code,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "event",
                            "repeat",
                            "code",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#event: Option<Vec<fhirbolt_model::r4b::types::DateTime>> = None;
                let mut r#repeat: Option<fhirbolt_model::r4b::types::TimingRepeat> = None;
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
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
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
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
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Event => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#event.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("event"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#event.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::DateTime,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::EventPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#event.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_event"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("event");
                            }
                        }
                        Field::Repeat => {
                            if r#repeat.is_some() {
                                return Err(serde::de::Error::duplicate_field("repeat"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4b::types::TimingRepeat,
                            > = self.0.transmute();
                            r#repeat = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#code = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(Timing {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#event: r#event.unwrap_or(vec![]),
                    r#repeat,
                    r#code,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Timing>> {
    type Value = Box<Timing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Timing>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Timing>> {
    type Value = Vec<Timing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Timing>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Timing>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Timing> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
