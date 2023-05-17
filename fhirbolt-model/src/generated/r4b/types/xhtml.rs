// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
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
impl<I: Into<std::string::String>> From<I> for Xhtml {
    fn from(v: I) -> Self {
        Xhtml {
            value: v.into(),
            ..Default::default()
        }
    }
}
