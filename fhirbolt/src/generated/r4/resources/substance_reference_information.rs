// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SubstanceReferenceInformationTargetAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct SubstanceReferenceInformationGene {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#gene: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#gene_sequence_origin: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceReferenceInformationGeneElement {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#element: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceReferenceInformationClassification {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#classification: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct SubstanceReferenceInformationTarget {
    pub r#id: Option<std::string::String>,
    pub r#interaction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#organism_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#target: Option<Box<super::super::types::Identifier>>,
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#organism: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#amount: Option<SubstanceReferenceInformationTargetAmount>,
}
#[derive(Debug, Clone)]
pub struct SubstanceReferenceInformation {
    pub r#gene: Vec<SubstanceReferenceInformationGene>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#gene_element: Vec<SubstanceReferenceInformationGeneElement>,
    pub r#comment: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#classification: Vec<SubstanceReferenceInformationClassification>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#target: Vec<SubstanceReferenceInformationTarget>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
