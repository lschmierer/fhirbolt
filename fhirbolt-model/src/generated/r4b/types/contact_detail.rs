// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Base StructureDefinition for ContactDetail Type: Specifies contact information for a person or organization.\n\nNeed to track contact information in the same way across multiple resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContactDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The name of an individual to contact."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The contact details for the individual (if a name was provided) or the organization."]
    pub r#telecom: Vec<super::super::types::ContactPoint>,
}
#[allow(clippy::derivable_impls)]
impl Default for ContactDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#name: Default::default(),
            r#telecom: Default::default(),
        }
    }
}
