// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct DeviceDeviceName {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
    pub r#type: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct DeviceVersion {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#component: Option<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#value: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DeviceSpecialization {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#system_type: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#version: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct DeviceProperty {
    pub r#id: Option<std::string::String>,
    pub r#value_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#value_quantity: Vec<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DeviceUdiCarrier {
    pub r#jurisdiction: Option<super::super::types::Uri>,
    pub r#carrier_aidc: Option<super::super::types::Base64Binary>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#carrier_hrf: Option<super::super::types::String>,
    pub r#device_identifier: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#issuer: Option<super::super::types::Uri>,
    pub r#entry_type: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct Device {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#parent: Option<Box<super::super::types::Reference>>,
    pub r#part_number: Option<super::super::types::String>,
    pub r#status: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#device_name: Vec<DeviceDeviceName>,
    pub r#owner: Option<Box<super::super::types::Reference>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#version: Vec<DeviceVersion>,
    pub r#manufacture_date: Option<super::super::types::DateTime>,
    pub r#distinct_identifier: Option<super::super::types::String>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#lot_number: Option<super::super::types::String>,
    pub r#model_number: Option<super::super::types::String>,
    pub r#specialization: Vec<DeviceSpecialization>,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#definition: Option<Box<super::super::types::Reference>>,
    pub r#serial_number: Option<super::super::types::String>,
    pub r#safety: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#manufacturer: Option<super::super::types::String>,
    pub r#expiration_date: Option<super::super::types::DateTime>,
    pub r#property: Vec<DeviceProperty>,
    pub r#udi_carrier: Vec<DeviceUdiCarrier>,
}
