// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.\n\nThere is a need for a concise way to handle the data produced by devices that sample a physical state at a high frequency."]
#[derive(Debug, Clone, PartialEq)]
pub struct SampledData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The base quantity that a measured value of zero represents. In addition, this provides the units of the entire measurement series."]
    pub r#origin: Box<super::super::types::Quantity>,
    #[doc = "Amount of intervalUnits between samples, e.g. milliseconds for time-based sampling."]
    pub r#interval: Option<super::super::types::Decimal>,
    #[doc = "The measurement unit in which the sample interval is expressed."]
    pub r#interval_unit: super::super::types::Code,
    #[doc = "A correction factor that is applied to the sampled data points before they are added to the origin."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The lower limit of detection of the measured points. This is needed if any of the data points have the value \"L\" (lower than detection limit)."]
    pub r#lower_limit: Option<super::super::types::Decimal>,
    #[doc = "The upper limit of detection of the measured points. This is needed if any of the data points have the value \"U\" (higher than detection limit)."]
    pub r#upper_limit: Option<super::super::types::Decimal>,
    #[doc = "The number of sample points at each time point. If this value is greater than one, then the dimensions will be interlaced - all the sample points for a point in time will be recorded at once."]
    pub r#dimensions: super::super::types::PositiveInt,
    #[doc = "Reference to ConceptMap that defines the codes used in the data."]
    pub r#code_map: Option<super::super::types::Canonical>,
    #[doc = "A series of data points which are decimal values separated by a single space (character u20).  The units in which the offsets are expressed are found in intervalUnit.  The absolute point at which the measurements begin SHALL be conveyed outside the scope of this datatype, e.g. Observation.effectiveDateTime for a timing offset."]
    pub r#offsets: Option<super::super::types::String>,
    #[doc = "A series of data points which are decimal values or codes separated by a single space (character u20). The special codes \"E\" (error), \"L\" (below detection limit) and \"U\" (above detection limit) are also defined for used in place of decimal values."]
    pub r#data: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for SampledData {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#origin: Box::new(super::super::types::Quantity {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#interval: Default::default(),
            r#interval_unit: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#factor: Default::default(),
            r#lower_limit: Default::default(),
            r#upper_limit: Default::default(),
            r#dimensions: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#code_map: Default::default(),
            r#offsets: Default::default(),
            r#data: Default::default(),
        }
    }
}
