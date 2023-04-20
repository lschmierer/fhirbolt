// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.\n\nNeed to track contributor information in the same way across multiple resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct Contributor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of contributor."]
    pub r#type: super::super::types::Code,
    #[doc = "The name of the individual or organization responsible for the contribution."]
    pub r#name: super::super::types::String,
    #[doc = "Contact details to assist a user in finding and communicating with the contributor."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
}
impl Default for Contributor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#type: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#name: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#contact: Default::default(),
        }
    }
}