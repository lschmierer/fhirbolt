// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct DeviceMetricCalibration {
    pub r#time: Option<super::super::types::Instant>,
    pub r#type: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#state: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DeviceMetric {
    pub r#color: Option<super::super::types::Code>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#operational_status: Option<super::super::types::Code>,
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#calibration: Vec<DeviceMetricCalibration>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#category: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#measurement_period: Option<Box<super::super::types::Timing>>,
    pub r#parent: Option<Box<super::super::types::Reference>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#language: Option<super::super::types::Code>,
}
