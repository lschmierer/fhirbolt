// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Base StructureDefinition for unsignedInt type: An integer with a value that is not negative (e.g. >= 0)"]
#[derive(Debug, Clone, PartialEq)]
pub struct UnsignedInt {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Primitive value for unsignedInt"]
    pub r#value: Option<u32>,
}
#[allow(clippy::derivable_impls)]
impl Default for UnsignedInt {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#value: Default::default(),
        }
    }
}
impl<I: Into<u32>> From<I> for UnsignedInt {
    fn from(v: I) -> Self {
        UnsignedInt {
            value: Some(v.into()),
            ..Default::default()
        }
    }
}
