// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#is_defining: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerMonomerSet {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#ratio_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#starting_material: Vec<SubstancePolymerMonomerSetStartingMaterial>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#degree: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
    pub r#representation: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnit {
    pub r#degree_of_polymerisation: Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    pub r#orientation_of_polymerisation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#structural_representation: Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#repeat_unit: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymerRepeat {
    pub r#repeat_unit_amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#repeat_unit: Vec<SubstancePolymerRepeatRepeatUnit>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number_of_units: Option<super::super::types::Integer>,
    pub r#average_molecular_formula: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct SubstancePolymer {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#copolymer_connectivity: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#geometry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modification: Vec<super::super::types::String>,
    pub r#monomer_set: Vec<SubstancePolymerMonomerSet>,
    pub r#repeat: Vec<SubstancePolymerRepeat>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
