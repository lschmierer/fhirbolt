// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct DeviceMetricCalibration {
    pub r#state: Option<super::super::types::Code>,
    pub r#time: Option<super::super::types::Instant>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct DeviceMetric {
    pub r#category: super::super::types::Code,
    pub r#calibration: Vec<DeviceMetricCalibration>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#parent: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#measurement_period: Option<Box<super::super::types::Timing>>,
    pub r#operational_status: Option<super::super::types::Code>,
    pub r#color: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
