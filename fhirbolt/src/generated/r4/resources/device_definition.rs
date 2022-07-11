// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum DeviceDefinitionManufacturer {
    String(Box<super::super::types::String>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct DeviceDefinitionMaterial {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#substance: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
    pub r#alternate: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct DeviceDefinitionProperty {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#value_quantity: Vec<Box<super::super::types::Quantity>>,
}
#[derive(Debug, Clone)]
pub struct DeviceDefinitionCapability {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    pub r#jurisdiction: super::super::types::Uri,
    pub r#id: Option<std::string::String>,
    pub r#issuer: super::super::types::Uri,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#device_identifier: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct DeviceDefinitionSpecialization {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#system_type: super::super::types::String,
    pub r#version: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DeviceDefinitionDeviceName {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#name: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct DeviceDefinition {
    pub r#language: Option<super::super::types::Code>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#model_number: Option<super::super::types::String>,
    pub r#online_information: Option<super::super::types::Uri>,
    pub r#shelf_life_storage: Vec<Box<super::super::types::ProductShelfLife>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#manufacturer: Option<DeviceDefinitionManufacturer>,
    pub r#material: Vec<DeviceDefinitionMaterial>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#safety: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#parent_device: Option<Box<super::super::types::Reference>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#owner: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#property: Vec<DeviceDefinitionProperty>,
    pub r#capability: Vec<DeviceDefinitionCapability>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#language_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,
    pub r#specialization: Vec<DeviceDefinitionSpecialization>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#device_name: Vec<DeviceDefinitionDeviceName>,
    pub r#version: Vec<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
}
