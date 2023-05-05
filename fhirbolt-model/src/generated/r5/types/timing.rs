// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Either a duration for the length of the timing schedule, a range of possible length, or outer bounds for start and/or end limits of the timing schedule."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TimingRepeatBounds {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "A set of rules that describe when the event is scheduled."]
#[derive(Debug, Clone, PartialEq)]
pub struct TimingRepeat {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
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
    #[doc = "The units of time for the duration, in UCUM units\nNormal practice is to use the 'mo' code as a calendar month when calculating the next occurrence."]
    pub r#duration_unit: Option<super::super::types::Code>,
    #[doc = "The number of times to repeat the action within the specified period. If frequencyMax is present, this element indicates the lower bound of the allowed range of the frequency."]
    pub r#frequency: Option<super::super::types::PositiveInt>,
    #[doc = "If present, indicates that the frequency is a range - so to repeat between `frequency` and `frequency_max` times within the period or period range."]
    pub r#frequency_max: Option<super::super::types::PositiveInt>,
    #[doc = "Indicates the duration of time over which repetitions are to occur; e.g. to express \"3 times per day\", 3 would be the frequency and \"1 day\" would be the period. If periodMax is present, this element indicates the lower bound of the allowed range of the period length."]
    pub r#period: Option<super::super::types::Decimal>,
    #[doc = "If present, indicates that the period is a range from `period` to `period_max`, allowing expressing concepts such as \"do this once every 3-5 days."]
    pub r#period_max: Option<super::super::types::Decimal>,
    #[doc = "The units of time for the period in UCUM units\nNormal practice is to use the 'mo' code as a calendar month when calculating the next occurrence."]
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
#[allow(clippy::derivable_impls)]
impl Default for TimingRepeat {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#bounds: Default::default(),
            r#count: Default::default(),
            r#count_max: Default::default(),
            r#duration: Default::default(),
            r#duration_max: Default::default(),
            r#duration_unit: Default::default(),
            r#frequency: Default::default(),
            r#frequency_max: Default::default(),
            r#period: Default::default(),
            r#period_max: Default::default(),
            r#period_unit: Default::default(),
            r#day_of_week: Default::default(),
            r#time_of_day: Default::default(),
            r#when: Default::default(),
            r#offset: Default::default(),
        }
    }
}
#[doc = "Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.\n\nNeed to able to track proposed timing schedules. There are several different ways to do this: one or more specified times, a simple rules like three times a day, or  before/after meals."]
#[derive(Debug, Clone, PartialEq)]
pub struct Timing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies specific times when the event occurs."]
    pub r#event: Vec<super::super::types::DateTime>,
    #[doc = "A set of rules that describe when the event is scheduled."]
    pub r#repeat: Option<TimingRepeat>,
    #[doc = "A code for the timing schedule (or just text in code.text). Some codes such as BID are ubiquitous, but many institutions define their own additional codes. If a code is provided, the code is understood to be a complete statement of whatever is specified in the structured timing data, and either the code or the data may be used to interpret the Timing, with the exception that .repeat.bounds still applies over the code (and is not contained in the code)."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Timing {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#event: Default::default(),
            r#repeat: Default::default(),
            r#code: Default::default(),
        }
    }
}
