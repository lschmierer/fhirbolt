// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum TimingRepeatBounds {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct TimingRepeat {
    pub r#count_max: Option<super::super::types::PositiveInt>,
    pub r#frequency: Option<super::super::types::PositiveInt>,
    pub r#count: Option<super::super::types::PositiveInt>,
    pub r#time_of_day: Vec<super::super::types::Time>,
    pub r#duration_unit: Option<super::super::types::Code>,
    pub r#period_max: Option<super::super::types::Decimal>,
    pub r#period_unit: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#bounds: Option<TimingRepeatBounds>,
    pub r#day_of_week: Vec<super::super::types::Code>,
    pub r#offset: Option<super::super::types::UnsignedInt>,
    pub r#frequency_max: Option<super::super::types::PositiveInt>,
    pub r#when: Vec<super::super::types::Code>,
    pub r#period: Option<super::super::types::Decimal>,
    pub r#duration: Option<super::super::types::Decimal>,
    pub r#duration_max: Option<super::super::types::Decimal>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct Timing {
    pub r#repeat: Option<Box<super::super::types::Element>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#event: Vec<super::super::types::DateTime>,
}
