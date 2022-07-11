// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialPartDescription {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#part_location: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialFractionDescription {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#fraction: Option<super::super::types::String>,
    pub r#material_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismAuthor {
    pub r#id: Option<std::string::String>,
    pub r#author_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#author_description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#order: Option<Box<super::super::types::CodeableConcept>>,
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#kingdom: Option<Box<super::super::types::CodeableConcept>>,
    pub r#phylum: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismHybrid {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#hybrid_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#paternal_organism_name: Option<super::super::types::String>,
    pub r#maternal_organism_id: Option<super::super::types::String>,
    pub r#paternal_organism_id: Option<super::super::types::String>,
    pub r#maternal_organism_name: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganism {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#intraspecific_description: Option<super::super::types::String>,
    pub r#author: Vec<SubstanceSourceMaterialOrganismAuthor>,
    pub r#organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
    pub r#hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    pub r#intraspecific_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#family: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterial {
    pub r#development_stage: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source_material_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#parent_substance_id: Vec<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source_material_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#geographical_location: Vec<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#country_of_origin: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#part_description: Vec<SubstanceSourceMaterialPartDescription>,
    pub r#source_material_class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#fraction_description: Vec<SubstanceSourceMaterialFractionDescription>,
    pub r#organism: Option<SubstanceSourceMaterialOrganism>,
    pub r#organism_name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#organism_id: Option<Box<super::super::types::Identifier>>,
    pub r#parent_substance_name: Vec<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
