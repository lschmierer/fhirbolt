// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for xhtml Type"]
#[derive(Default, Debug, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Xhtml {
    #[doc = "unique id for the element within a resource (for internal references)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#id: Option<std::string::String>,
    #[doc = "Actual xhtml"]
    pub r#value: std::string::String,
}
