// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "xhtml Type definition"]
#[derive(Debug, Clone, PartialEq)]
pub struct Xhtml {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "Actual xhtml"]
    pub r#value: std::string::String,
}
impl Default for Xhtml {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#value: Default::default(),
        }
    }
}
