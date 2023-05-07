// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "Base StructureDefinition for xhtml Type"]
#[derive(Debug, Clone, PartialEq)]
pub struct Xhtml {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "Actual xhtml"]
    pub r#value: std::string::String,
}
#[allow(clippy::derivable_impls)]
impl Default for Xhtml {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#value: Default::default(),
        }
    }
}
