// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationMoiety {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#molecular_formula: Option<super::super::types::String>,
    pub r#amount: Option<SubstanceSpecificationMoietyAmount>,
    pub r#name: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationPropertyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationPropertyDefiningSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationProperty {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<SubstanceSpecificationPropertyAmount>,
    pub r#parameters: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#defining_substance: Option<SubstanceSpecificationPropertyDefiningSubstance>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationCode {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status_date: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#comment: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationRelationshipAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationRelationshipSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationRelationship {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#amount: Option<SubstanceSpecificationRelationshipAmount>,
    pub r#substance: Option<SubstanceSpecificationRelationshipSubstance>,
    pub r#is_defining: Option<super::super::types::Boolean>,
    pub r#amount_ratio_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructureRepresentation {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#representation: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotope {
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#substitution: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#half_life: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructure {
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#molecular_formula: Option<super::super::types::String>,
    pub r#representation: Vec<SubstanceSpecificationStructureRepresentation>,
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#id: Option<std::string::String>,
    pub r#isotope: Vec<SubstanceSpecificationStructureIsotope>,
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationNameOfficial {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationName {
    pub r#id: Option<std::string::String>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#name: super::super::types::String,
    pub r#synonym: Vec<SubstanceSpecificationName>,
    pub r#official: Vec<SubstanceSpecificationNameOfficial>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#translation: Vec<SubstanceSpecificationName>,
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecification {
    pub r#moiety: Vec<SubstanceSpecificationMoiety>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#polymer: Option<Box<super::super::types::Reference>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#property: Vec<SubstanceSpecificationProperty>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reference_information: Option<Box<super::super::types::Reference>>,
    pub r#nucleic_acid: Option<Box<super::super::types::Reference>>,
    pub r#molecular_weight: Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#protein: Option<Box<super::super::types::Reference>>,
    pub r#description: Option<super::super::types::String>,
    pub r#code: Vec<SubstanceSpecificationCode>,
    pub r#relationship: Vec<SubstanceSpecificationRelationship>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#structure: Option<SubstanceSpecificationStructure>,
    pub r#source_material: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Vec<SubstanceSpecificationName>,
}
