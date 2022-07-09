// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct DeviceVersion {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#component: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct DeviceSpecialization {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#system_type: Box<super::super::types::CodeableConcept>,
    pub r#version: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DeviceUdiCarrier {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#carrier_aidc: Option<super::super::types::Base64Binary>,
    pub r#issuer: Option<super::super::types::Uri>,
    pub r#device_identifier: Option<super::super::types::String>,
    pub r#jurisdiction: Option<super::super::types::Uri>,
    pub r#carrier_hrf: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#entry_type: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct DeviceProperty {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value_quantity: Vec<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#value_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct DeviceDeviceName {
    pub r#type: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct Device {
    pub r#version: Vec<DeviceVersion>,
    pub r#manufacturer: Option<super::super::types::String>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#specialization: Vec<DeviceSpecialization>,
    pub r#id: Option<std::string::String>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#owner: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#distinct_identifier: Option<super::super::types::String>,
    pub r#expiration_date: Option<super::super::types::DateTime>,
    pub r#parent: Option<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#udi_carrier: Vec<DeviceUdiCarrier>,
    pub r#definition: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#lot_number: Option<super::super::types::String>,
    pub r#property: Vec<DeviceProperty>,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#device_name: Vec<DeviceDeviceName>,
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#model_number: Option<super::super::types::String>,
    pub r#part_number: Option<super::super::types::String>,
    pub r#safety: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#manufacture_date: Option<super::super::types::DateTime>,
    pub r#status: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#serial_number: Option<super::super::types::String>,
}
