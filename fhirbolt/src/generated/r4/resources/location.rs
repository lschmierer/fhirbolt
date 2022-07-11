// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct LocationPosition {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#longitude: super::super::types::Decimal,
    pub r#latitude: super::super::types::Decimal,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#altitude: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct LocationHoursOfOperation {
    pub r#opening_time: Option<super::super::types::Time>,
    pub r#closing_time: Option<super::super::types::Time>,
    pub r#all_day: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#days_of_week: Vec<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Location {
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#description: Option<super::super::types::String>,
    pub r#operational_status: Option<Box<super::super::types::Coding>>,
    pub r#mode: Option<super::super::types::Code>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    pub r#availability_exceptions: Option<super::super::types::String>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#name: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#physical_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#position: Option<LocationPosition>,
    pub r#hours_of_operation: Vec<LocationHoursOfOperation>,
    pub r#address: Option<Box<super::super::types::Address>>,
}
