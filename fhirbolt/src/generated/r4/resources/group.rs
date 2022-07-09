// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum GroupCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct GroupCharacteristic {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#value: GroupCharacteristicValue,
    pub r#exclude: super::super::types::Boolean,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct GroupMember {
    pub r#entity: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#inactive: Option<super::super::types::Boolean>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct Group {
    pub r#actual: super::super::types::Boolean,
    pub r#managing_entity: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Option<super::super::types::UnsignedInt>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type: super::super::types::Code,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#characteristic: Vec<GroupCharacteristic>,
    pub r#member: Vec<GroupMember>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#name: Option<super::super::types::String>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#active: Option<super::super::types::Boolean>,
}
