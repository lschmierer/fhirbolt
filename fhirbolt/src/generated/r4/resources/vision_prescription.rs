// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct VisionPrescriptionLensSpecificationPrism {
    pub r#amount: super::super::types::Decimal,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#base: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct VisionPrescriptionLensSpecification {
    pub r#axis: Option<super::super::types::Integer>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    pub r#sphere: Option<super::super::types::Decimal>,
    pub r#color: Option<super::super::types::String>,
    pub r#diameter: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#brand: Option<super::super::types::String>,
    pub r#back_curve: Option<super::super::types::Decimal>,
    pub r#product: Box<super::super::types::CodeableConcept>,
    pub r#eye: super::super::types::Code,
    pub r#prism: Vec<VisionPrescriptionLensSpecificationPrism>,
    pub r#power: Option<super::super::types::Decimal>,
    pub r#id: Option<std::string::String>,
    pub r#cylinder: Option<super::super::types::Decimal>,
    pub r#add: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct VisionPrescription {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#lens_specification: Vec<VisionPrescriptionLensSpecification>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#created: super::super::types::DateTime,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#date_written: super::super::types::DateTime,
    pub r#prescriber: Box<super::super::types::Reference>,
}
