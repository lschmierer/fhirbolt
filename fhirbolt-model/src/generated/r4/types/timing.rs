// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "Either a duration for the length of the timing schedule, a range of possible length, or outer bounds for start and/or end limits of the timing schedule."]
#[derive(Debug, Clone)]
pub enum TimingRepeatBounds {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for TimingRepeatBounds {
    fn default() -> TimingRepeatBounds {
        TimingRepeatBounds::Invalid
    }
}
#[doc = "A set of rules that describe when the event is scheduled."]
#[derive(Default, Debug, Clone)]
pub struct TimingRepeat {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Either a duration for the length of the timing schedule, a range of possible length, or outer bounds for start and/or end limits of the timing schedule."]
    pub r#bounds: Option<TimingRepeatBounds>,
    #[doc = "A total count of the desired number of repetitions across the duration of the entire timing specification. If countMax is present, this element indicates the lower bound of the allowed range of count values."]
    pub r#count: Option<super::super::types::PositiveInt>,
    #[doc = "If present, indicates that the count is a range - so to perform the action between `count` and `count_max` times."]
    pub r#count_max: Option<super::super::types::PositiveInt>,
    #[doc = "How long this thing happens for when it happens. If durationMax is present, this element indicates the lower bound of the allowed range of the duration."]
    pub r#duration: Option<super::super::types::Decimal>,
    #[doc = "If present, indicates that the duration is a range - so to perform the action between `duration` and `duration_max` time length."]
    pub r#duration_max: Option<super::super::types::Decimal>,
    #[doc = "The units of time for the duration, in UCUM units."]
    pub r#duration_unit: Option<super::super::types::Code>,
    #[doc = "The number of times to repeat the action within the specified period. If frequencyMax is present, this element indicates the lower bound of the allowed range of the frequency."]
    pub r#frequency: Option<super::super::types::PositiveInt>,
    #[doc = "If present, indicates that the frequency is a range - so to repeat between `frequency` and `frequency_max` times within the period or period range."]
    pub r#frequency_max: Option<super::super::types::PositiveInt>,
    #[doc = "Indicates the duration of time over which repetitions are to occur; e.g. to express \"3 times per day\", 3 would be the frequency and \"1 day\" would be the period. If periodMax is present, this element indicates the lower bound of the allowed range of the period length."]
    pub r#period: Option<super::super::types::Decimal>,
    #[doc = "If present, indicates that the period is a range from `period` to `period_max`, allowing expressing concepts such as \"do this once every 3-5 days."]
    pub r#period_max: Option<super::super::types::Decimal>,
    #[doc = "The units of time for the period in UCUM units."]
    pub r#period_unit: Option<super::super::types::Code>,
    #[doc = "If one or more days of week is provided, then the action happens only on the specified day(s)."]
    pub r#day_of_week: Vec<super::super::types::Code>,
    #[doc = "Specified time of day for action to take place."]
    pub r#time_of_day: Vec<super::super::types::Time>,
    #[doc = "An approximate time period during the day, potentially linked to an event of daily living that indicates when the action should occur."]
    pub r#when: Vec<super::super::types::Code>,
    #[doc = "The number of minutes from the event. If the event code does not indicate whether the minutes is before or after the event, then the offset is assumed to be after the event."]
    pub r#offset: Option<super::super::types::UnsignedInt>,
}
impl crate::AnyResource for TimingRepeat {}
impl serde::ser::Serialize for TimingRepeat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#bounds.as_ref() {
            match some {
                TimingRepeatBounds::Duration(ref value) => {
                    state.serialize_entry("boundsDuration", value)?;
                }
                TimingRepeatBounds::Range(ref value) => {
                    state.serialize_entry("boundsRange", value)?;
                }
                TimingRepeatBounds::Period(ref value) => {
                    state.serialize_entry("boundsPeriod", value)?;
                }
                TimingRepeatBounds::Invalid => {
                    return Err(serde::ser::Error::custom("bounds is invalid"))
                }
            }
        }
        if let Some(some) = self.r#count.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("count", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_count", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#count_max.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("countMax", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_countMax", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#duration.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("duration", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_duration", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#duration_max.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("durationMax", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_durationMax", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#duration_unit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("durationUnit", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_durationUnit", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#frequency.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("frequency", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_frequency", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#frequency_max.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("frequencyMax", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_frequencyMax", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("period", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_period", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period_max.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("periodMax", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_periodMax", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period_unit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("periodUnit", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_periodUnit", &primitive_element)?;
            }
        }
        if !self.r#day_of_week.is_empty() {
            let values = self
                .r#day_of_week
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("dayOfWeek", &values)?;
            }
            let requires_elements = self
                .r#day_of_week
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#day_of_week
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_dayOfWeek", &primitive_elements)?;
            }
        }
        if !self.r#time_of_day.is_empty() {
            let values = self
                .r#time_of_day
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("timeOfDay", &values)?;
            }
            let requires_elements = self
                .r#time_of_day
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#time_of_day
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_timeOfDay", &primitive_elements)?;
            }
        }
        if !self.r#when.is_empty() {
            let values = self
                .r#when
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("when", &values)?;
            }
            let requires_elements = self
                .r#when
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#when
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_when", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#offset.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("offset", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_offset", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for TimingRepeat {
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
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TimingRepeat;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TimingRepeat")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TimingRepeat, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#bounds: Option<TimingRepeatBounds> = None;
                let mut r#count: Option<super::super::types::PositiveInt> = None;
                let mut r#count_max: Option<super::super::types::PositiveInt> = None;
                let mut r#duration: Option<super::super::types::Decimal> = None;
                let mut r#duration_max: Option<super::super::types::Decimal> = None;
                let mut r#duration_unit: Option<super::super::types::Code> = None;
                let mut r#frequency: Option<super::super::types::PositiveInt> = None;
                let mut r#frequency_max: Option<super::super::types::PositiveInt> = None;
                let mut r#period: Option<super::super::types::Decimal> = None;
                let mut r#period_max: Option<super::super::types::Decimal> = None;
                let mut r#period_unit: Option<super::super::types::Code> = None;
                let mut r#day_of_week: Option<Vec<super::super::types::Code>> = None;
                let mut r#time_of_day: Option<Vec<super::super::types::Time>> = None;
                let mut r#when: Option<Vec<super::super::types::Code>> = None;
                let mut r#offset: Option<super::super::types::UnsignedInt> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::BoundsDuration => {
                                if r#bounds.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "boundsDuration",
                                    ));
                                }
                                r#bounds =
                                    Some(TimingRepeatBounds::Duration(map_access.next_value()?));
                            }
                            Field::BoundsRange => {
                                if r#bounds.is_some() {
                                    return Err(serde::de::Error::duplicate_field("boundsRange"));
                                }
                                r#bounds =
                                    Some(TimingRepeatBounds::Range(map_access.next_value()?));
                            }
                            Field::BoundsPeriod => {
                                if r#bounds.is_some() {
                                    return Err(serde::de::Error::duplicate_field("boundsPeriod"));
                                }
                                r#bounds =
                                    Some(TimingRepeatBounds::Period(map_access.next_value()?));
                            }
                            Field::Count => {
                                let some = r#count.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("count"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CountPrimitiveElement => {
                                let some = r#count.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_count"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::CountMax => {
                                let some = r#count_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("countMax"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CountMaxPrimitiveElement => {
                                let some = r#count_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_countMax"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Duration => {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::DurationPrimitiveElement => {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_duration"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DurationMax => {
                                let some = r#duration_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationMax"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::DurationMaxPrimitiveElement => {
                                let some = r#duration_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_durationMax"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DurationUnit => {
                                let some = r#duration_unit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationUnit"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DurationUnitPrimitiveElement => {
                                let some = r#duration_unit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_durationUnit"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Frequency => {
                                let some = r#frequency.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequency"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::FrequencyPrimitiveElement => {
                                let some = r#frequency.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frequency"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::FrequencyMax => {
                                let some = r#frequency_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequencyMax"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::FrequencyMaxPrimitiveElement => {
                                let some = r#frequency_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frequencyMax"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Period => {
                                let some = r#period.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::PeriodPrimitiveElement => {
                                let some = r#period.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_period"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::PeriodMax => {
                                let some = r#period_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodMax"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::PeriodMaxPrimitiveElement => {
                                let some = r#period_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_periodMax"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::PeriodUnit => {
                                let some = r#period_unit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodUnit"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PeriodUnitPrimitiveElement => {
                                let some = r#period_unit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_periodUnit"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DayOfWeek => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                            }
                            Field::DayOfWeekPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
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
                            }
                            Field::TimeOfDay => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                            }
                            Field::TimeOfDayPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
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
                            }
                            Field::When => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                            }
                            Field::WhenPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
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
                            }
                            Field::Offset => {
                                let some = r#offset.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offset"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::OffsetPrimitiveElement => {
                                let some = r#offset.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_offset"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
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
                                    ));
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Base StructureDefinition for Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.\n\nNeed to able to track proposed timing schedules. There are several different ways to do this: one or more specified times, a simple rules like three times a day, or  before/after meals."]
#[derive(Default, Debug, Clone)]
pub struct Timing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies specific times when the event occurs."]
    pub r#event: Vec<super::super::types::DateTime>,
    #[doc = "A set of rules that describe when the event is scheduled."]
    pub r#repeat: Option<TimingRepeat>,
    #[doc = "A code for the timing schedule (or just text in code.text). Some codes such as BID are ubiquitous, but many institutions define their own additional codes. If a code is provided, the code is understood to be a complete statement of whatever is specified in the structured timing data, and either the code or the data may be used to interpret the Timing, with the exception that .repeat.bounds still applies over the code (and is not contained in the code)."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for Timing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if !self.r#event.is_empty() {
            let values = self
                .r#event
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("event", &values)?;
            }
            let requires_elements = self
                .r#event
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#event
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_event", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#repeat.as_ref() {
            state.serialize_entry("repeat", some)?;
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Timing {
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
            #[serde(rename = "event")]
            Event,
            #[serde(rename = "_event")]
            EventPrimitiveElement,
            #[serde(rename = "repeat")]
            Repeat,
            #[serde(rename = "code")]
            Code,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Timing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Timing")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Timing, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#event: Option<Vec<super::super::types::DateTime>> = None;
                let mut r#repeat: Option<TimingRepeat> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Event => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                            }
                            Field::EventPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
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
                            }
                            Field::Repeat => {
                                if r#repeat.is_some() {
                                    return Err(serde::de::Error::duplicate_field("repeat"));
                                }
                                r#repeat = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "event",
                                            "repeat",
                                            "code",
                                        ],
                                    ));
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
