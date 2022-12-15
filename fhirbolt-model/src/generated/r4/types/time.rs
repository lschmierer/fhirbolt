// Generated on 2022-12-15 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for time Type: A time during the day, with no date specified"]
#[derive(Default, Debug, Clone, serde :: Serialize)]
pub struct Time {
    #[doc = "unique id for the element within a resource (for internal references)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual value"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#value: Option<std::string::String>,
}
impl crate::AnyResource for Time {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
