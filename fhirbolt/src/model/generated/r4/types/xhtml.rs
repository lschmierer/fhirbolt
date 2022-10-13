// Generated on 2022-10-11 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for xhtml Type"]
#[derive(Default, Debug, Clone)]
pub struct Xhtml {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "Actual xhtml"]
    pub r#value: std::string::String,
}
impl crate::model::ResourceOrElement for Xhtml {}
