// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for markdown type: A string that may contain Github Flavored Markdown syntax for optional processing by a mark down presentation engine"]
#[derive(Default, Debug, Clone)]
pub struct Markdown {
    #[doc = "unique id for the element within a resource (for internal references)"]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Primitive value for markdown"]
    pub r#value: Option<std::string::String>,
}
impl crate::AnyResource for Markdown {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
