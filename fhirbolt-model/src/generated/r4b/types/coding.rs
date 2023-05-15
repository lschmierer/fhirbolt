// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "Base StructureDefinition for Coding Type: A reference to a code defined by a terminology system.\n\nReferences to codes are very common in healthcare models."]
#[derive(Debug, Clone, PartialEq)]
pub struct Coding {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The identification of the code system that defines the meaning of the symbol in the code."]
    pub r#system: Option<super::super::types::Uri>,
    #[doc = "The version of the code system which was used when choosing this code. Note that a well-maintained code system does not need the version reported, because the meaning of codes is consistent across versions. However this cannot consistently be assured, and when the meaning is not guaranteed to be consistent, the version SHOULD be exchanged."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A symbol in syntax defined by the system. The symbol may be a predefined code or an expression in a syntax defined by the coding system (e.g. post-coordination)."]
    pub r#code: Option<super::super::types::Code>,
    #[doc = "A representation of the meaning of the code in the system, following the rules of the system."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "Indicates that this coding was chosen by a user directly - e.g. off a pick list of available items (codes or displays)."]
    pub r#user_selected: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for Coding {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#system: Default::default(),
            r#version: Default::default(),
            r#code: Default::default(),
            r#display: Default::default(),
            r#user_selected: Default::default(),
        }
    }
}
