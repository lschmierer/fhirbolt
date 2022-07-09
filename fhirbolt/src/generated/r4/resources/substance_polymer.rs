// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    pub r#is_defining: Option<super::super::types::Boolean>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerMonomerSet {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#starting_material: Vec<SubstancePolymerMonomerSetStartingMaterial>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#ratio_type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    pub r#id: Option<std::string::String>,
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#representation: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    pub r#id: Option<std::string::String>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#degree: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnit {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#structural_representation: Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#degree_of_polymerisation: Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,
    pub r#orientation_of_polymerisation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#repeat_unit: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeat {
    pub r#number_of_units: Option<super::super::types::Integer>,
    pub r#average_molecular_formula: Option<super::super::types::String>,
    pub r#repeat_unit_amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#repeat_unit: Vec<SubstancePolymerRepeatRepeatUnit>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymer {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#geometry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#monomer_set: Vec<SubstancePolymerMonomerSet>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modification: Vec<super::super::types::String>,
    pub r#copolymer_connectivity: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#repeat: Vec<SubstancePolymerRepeat>,
    pub r#id: Option<std::string::String>,
}
