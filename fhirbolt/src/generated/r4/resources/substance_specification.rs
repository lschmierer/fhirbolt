// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationPropertyDefiningSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationPropertyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
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
pub enum SubstanceSpecificationMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationProperty {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#defining_substance: Option<SubstanceSpecificationPropertyDefiningSubstance>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#parameters: Option<super::super::types::String>,
    pub r#amount: Option<SubstanceSpecificationPropertyAmount>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationRelationship {
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#is_defining: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount_ratio_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<SubstanceSpecificationRelationshipAmount>,
    pub r#substance: Option<SubstanceSpecificationRelationshipSubstance>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationCode {
    pub r#id: Option<std::string::String>,
    pub r#comment: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status_date: Option<super::super::types::DateTime>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructureRepresentation {
    pub r#id: Option<std::string::String>,
    pub r#representation: Option<super::super::types::String>,
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotope {
    pub r#substitution: Option<Box<super::super::types::CodeableConcept>>,
    pub r#name: Option<Box<super::super::types::CodeableConcept>>,
    pub r#half_life: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationStructure {
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#representation: Vec<SubstanceSpecificationStructureRepresentation>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#molecular_formula: Option<super::super::types::String>,
    pub r#isotope: Vec<SubstanceSpecificationStructureIsotope>,
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationMoiety {
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<SubstanceSpecificationMoietyAmount>,
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#molecular_formula: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationNameOfficial {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecificationName {
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#synonym: Vec<SubstanceSpecificationName>,
    pub r#official: Vec<SubstanceSpecificationNameOfficial>,
    pub r#translation: Vec<SubstanceSpecificationName>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSpecification {
    pub r#description: Option<super::super::types::String>,
    pub r#source_material: Option<Box<super::super::types::Reference>>,
    pub r#property: Vec<SubstanceSpecificationProperty>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#relationship: Vec<SubstanceSpecificationRelationship>,
    pub r#protein: Option<Box<super::super::types::Reference>>,
    pub r#code: Vec<SubstanceSpecificationCode>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#polymer: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#nucleic_acid: Option<Box<super::super::types::Reference>>,
    pub r#structure: Option<SubstanceSpecificationStructure>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#reference_information: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#moiety: Vec<SubstanceSpecificationMoiety>,
    pub r#molecular_weight: Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    pub r#name: Vec<SubstanceSpecificationName>,
}
