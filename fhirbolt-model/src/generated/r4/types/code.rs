// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Base StructureDefinition for code type: A string which has at least one character and no leading or trailing whitespace and where there is no whitespace other than single spaces in the contents"]
#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Primitive value for code"]
    pub r#value: Option<std::string::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for Code {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#value: Default::default(),
        }
    }
}
impl<I: Into<std::string::String>> From<I> for Code {
    fn from(v: I) -> Self {
        Code {
            value: Some(v.into()),
            ..Default::default()
        }
    }
}
