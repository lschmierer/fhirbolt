// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::types::TimingRepeat>
{
    type Value = fhirbolt_model::r4b::types::TimingRepeat;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::TimingRepeat,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::TimingRepeat;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TimingRepeat")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::TimingRepeat, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::BoundsDuration => {
                            if r#bounds.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundsDuration"));
                            }
                            r#bounds =
                                Some(fhirbolt_model::r4b::types::TimingRepeatBounds::Duration(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::BoundsRange => {
                            if r#bounds.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundsRange"));
                            }
                            r#bounds = Some(fhirbolt_model::r4b::types::TimingRepeatBounds::Range(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                )?,
                            ));
                        }
                        Field::BoundsPeriod => {
                            if r#bounds.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundsPeriod"));
                            }
                            r#bounds =
                                Some(fhirbolt_model::r4b::types::TimingRepeatBounds::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                    )?,
                                ));
                        }
                        Field::Count => {
                            if self.0.from_json {
                                let some = r#count.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("count"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#count.is_some() {
                                    return Err(serde::de::Error::duplicate_field("count"));
                                }
                                r#count = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::PositiveInt>(),
                                    )?,
                                );
                            }
                        }
                        Field::CountPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#count.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_count"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("count");
                            }
                        }
                        Field::CountMax => {
                            if self.0.from_json {
                                let some = r#count_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("countMax"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#count_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("countMax"));
                                }
                                r#count_max = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::PositiveInt>(),
                                    )?,
                                );
                            }
                        }
                        Field::CountMaxPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#count_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_countMax"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("countMax");
                            }
                        }
                        Field::Duration => {
                            if self.0.from_json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                r#duration = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::DurationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_duration"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("duration");
                            }
                        }
                        Field::DurationMax => {
                            if self.0.from_json {
                                let some = r#duration_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationMax"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#duration_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationMax"));
                                }
                                r#duration_max = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::DurationMaxPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#duration_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_durationMax"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("durationMax");
                            }
                        }
                        Field::DurationUnit => {
                            if self.0.from_json {
                                let some = r#duration_unit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationUnit"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#duration_unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationUnit"));
                                }
                                r#duration_unit = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::DurationUnitPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#duration_unit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_durationUnit"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("durationUnit");
                            }
                        }
                        Field::Frequency => {
                            if self.0.from_json {
                                let some = r#frequency.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequency"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#frequency.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequency"));
                                }
                                r#frequency = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::PositiveInt>(),
                                    )?,
                                );
                            }
                        }
                        Field::FrequencyPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#frequency.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frequency"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("frequency");
                            }
                        }
                        Field::FrequencyMax => {
                            if self.0.from_json {
                                let some = r#frequency_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequencyMax"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#frequency_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequencyMax"));
                                }
                                r#frequency_max = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::PositiveInt>(),
                                    )?,
                                );
                            }
                        }
                        Field::FrequencyMaxPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#frequency_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frequencyMax"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("frequencyMax");
                            }
                        }
                        Field::Period => {
                            if self.0.from_json {
                                let some = r#period.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::PeriodPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#period.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_period"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("period");
                            }
                        }
                        Field::PeriodMax => {
                            if self.0.from_json {
                                let some = r#period_max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodMax"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#period_max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodMax"));
                                }
                                r#period_max = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::PeriodMaxPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#period_max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_periodMax"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("periodMax");
                            }
                        }
                        Field::PeriodUnit => {
                            if self.0.from_json {
                                let some = r#period_unit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodUnit"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#period_unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodUnit"));
                                }
                                r#period_unit = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::PeriodUnitPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#period_unit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_periodUnit"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("periodUnit");
                            }
                        }
                        Field::DayOfWeek => {
                            if self.0.from_json {
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
                            } else {
                                let vec = r#day_of_week.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::DayOfWeekPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
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
                            if self.0.from_json {
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
                            } else {
                                let vec = r#time_of_day.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Time>(),
                                )?);
                            }
                        }
                        Field::TimeOfDayPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
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
                            if self.0.from_json {
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
                            } else {
                                let vec = r#when.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::WhenPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
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
                            if self.0.from_json {
                                let some = r#offset.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offset"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#offset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offset"));
                                }
                                r#offset = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::UnsignedInt>(),
                                    )?,
                                );
                            }
                        }
                        Field::OffsetPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#offset.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_offset"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                Ok(fhirbolt_model::r4b::types::TimingRepeat {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::TimingRepeat>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::TimingRepeat>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::TimingRepeat>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::TimingRepeat>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::TimingRepeat>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::TimingRepeat>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::TimingRepeat>;
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
                        .transmute::<fhirbolt_model::r4b::types::TimingRepeat>(),
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
        Vec<Box<fhirbolt_model::r4b::types::TimingRepeat>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::TimingRepeat>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::TimingRepeat>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::TimingRepeat>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::TimingRepeat>>(),
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
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::types::Timing>
{
    type Value = fhirbolt_model::r4b::types::Timing;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::types::Timing>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::Timing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Timing")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::Timing, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#event: Option<Vec<fhirbolt_model::r4b::types::DateTime>> = None;
                let mut r#repeat: Option<fhirbolt_model::r4b::types::TimingRepeat> = None;
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Event => {
                            if self.0.from_json {
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
                            } else {
                                let vec = r#event.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::EventPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
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
                            r#repeat = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<fhirbolt_model::r4b::types::TimingRepeat>(),
                                )?,
                            );
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::types::Timing {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4b::types::Timing>>
{
    type Value = Box<fhirbolt_model::r4b::types::Timing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::Timing>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<fhirbolt_model::r4b::types::Timing>>
{
    type Value = Vec<fhirbolt_model::r4b::types::Timing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::Timing>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::Timing>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(self.0.transmute::<fhirbolt_model::r4b::types::Timing>())?
                {
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
        Vec<Box<fhirbolt_model::r4b::types::Timing>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::Timing>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::Timing>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::Timing>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}