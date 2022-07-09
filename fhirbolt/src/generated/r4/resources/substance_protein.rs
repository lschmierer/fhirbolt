// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct SubstanceProteinSubunit {
    pub r#n_terminal_modification: Option<super::super::types::String>,
    pub r#sequence_attachment: Option<Box<super::super::types::Attachment>>,
    pub r#c_terminal_modification: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#n_terminal_modification_id: Option<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subunit: Option<super::super::types::Integer>,
    pub r#sequence: Option<super::super::types::String>,
    pub r#c_terminal_modification_id: Option<Box<super::super::types::Identifier>>,
    pub r#length: Option<super::super::types::Integer>,
}
#[derive(Debug, Clone)]
pub struct SubstanceProtein {
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#number_of_subunits: Option<super::super::types::Integer>,
    pub r#disulfide_linkage: Vec<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#subunit: Vec<SubstanceProteinSubunit>,
}
