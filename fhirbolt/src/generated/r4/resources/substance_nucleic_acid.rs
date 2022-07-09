// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct SubstanceNucleicAcidSubunitSugar {
    pub r#residue_site: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceNucleicAcidSubunitLinkage {
    pub r#connectivity: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#residue_site: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#name: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct SubstanceNucleicAcidSubunit {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#length: Option<super::super::types::Integer>,
    pub r#sequence: Option<super::super::types::String>,
    pub r#five_prime: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sugar: Vec<SubstanceNucleicAcidSubunitSugar>,
    pub r#linkage: Vec<SubstanceNucleicAcidSubunitLinkage>,
    pub r#id: Option<std::string::String>,
    pub r#three_prime: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subunit: Option<super::super::types::Integer>,
    pub r#sequence_attachment: Option<Box<super::super::types::Attachment>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceNucleicAcid {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number_of_subunits: Option<super::super::types::Integer>,
    pub r#area_of_hybridisation: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#subunit: Vec<SubstanceNucleicAcidSubunit>,
    pub r#language: Option<super::super::types::Code>,
    pub r#oligo_nucleotide_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sequence_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
}
