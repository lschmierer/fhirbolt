// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
#[doc = "ExtendedContactDetail Type: Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.\n\nNeed to track contact and address information in the same way across multiple resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedContactDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The purpose/type of contact."]
    pub r#purpose: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The name of an individual to contact, some types of contact detail are usually blank."]
    pub r#name: Vec<super::super::types::HumanName>,
    #[doc = "The contact details application for the purpose defined."]
    pub r#telecom: Vec<super::super::types::ContactPoint>,
    #[doc = "Address for the contact."]
    pub r#address: Option<Box<super::super::types::Address>>,
    #[doc = "This contact detail is handled/monitored by a specific organization. If the name is provided in the contact, then it is referring to the named individual within this organization."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Period that this contact was valid for usage."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ExtendedContactDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#purpose: Default::default(),
            r#name: Default::default(),
            r#telecom: Default::default(),
            r#address: Default::default(),
            r#organization: Default::default(),
            r#period: Default::default(),
        }
    }
}
