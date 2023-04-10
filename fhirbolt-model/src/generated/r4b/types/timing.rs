// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Either a duration for the length of the timing schedule, a range of possible length, or outer bounds for start and/or end limits of the timing schedule."]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for TimingRepeat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#count.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("count", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_count", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#count.as_ref() {
                    state.serialize_entry("count", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#count_max.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("countMax", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_countMax", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#count_max.as_ref() {
                    state.serialize_entry("countMax", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#duration.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("duration", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_duration", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#duration.as_ref() {
                    state.serialize_entry("duration", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#duration_max.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("durationMax", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_durationMax", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#duration_max.as_ref() {
                    state.serialize_entry("durationMax", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#duration_unit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("durationUnit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_durationUnit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#duration_unit.as_ref() {
                    state.serialize_entry("durationUnit", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#frequency.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("frequency", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_frequency", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#frequency.as_ref() {
                    state.serialize_entry("frequency", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#frequency_max.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("frequencyMax", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_frequencyMax", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#frequency_max.as_ref() {
                    state.serialize_entry("frequencyMax", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#period.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("period", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_period", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#period.as_ref() {
                    state.serialize_entry("period", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#period_max.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("periodMax", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_periodMax", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#period_max.as_ref() {
                    state.serialize_entry("periodMax", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#period_unit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("periodUnit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_periodUnit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#period_unit.as_ref() {
                    state.serialize_entry("periodUnit", some)?;
                }
            }
            if _ctx.output_json {
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#day_of_week.is_empty() {
                    state.serialize_entry("dayOfWeek", &self.r#day_of_week)?;
                }
            }
            if _ctx.output_json {
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#time_of_day.is_empty() {
                    state.serialize_entry("timeOfDay", &self.r#time_of_day)?;
                }
            }
            if _ctx.output_json {
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#when.is_empty() {
                    state.serialize_entry("when", &self.r#when)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#offset.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("offset", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_offset", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#offset.as_ref() {
                    state.serialize_entry("offset", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Base StructureDefinition for Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.\n\nNeed to able to track proposed timing schedules. There are several different ways to do this: one or more specified times, a simple rules like three times a day, or  before/after meals."]
#[derive(Default, Debug, Clone, PartialEq)]
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
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#event.is_empty() {
                    state.serialize_entry("event", &self.r#event)?;
                }
            }
            if let Some(some) = self.r#repeat.as_ref() {
                state.serialize_entry("repeat", some)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            state.end()
        })
    }
}
