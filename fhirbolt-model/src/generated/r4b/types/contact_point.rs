// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Base StructureDefinition for ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.\n\nNeed to track phone, fax, mobile, sms numbers, email addresses, twitter tags, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContactPoint {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Telecommunications form for contact point - what communications system is required to make use of the contact."]
    pub r#system: Option<super::super::types::Code>,
    #[doc = "The actual contact point details, in a form that is meaningful to the designated communication system (i.e. phone number or email address)."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "Identifies the purpose for the contact point."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "Specifies a preferred order in which to use a set of contacts. ContactPoints with lower rank values are more preferred than those with higher rank values."]
    pub r#rank: Option<super::super::types::PositiveInt>,
    #[doc = "Time period when the contact point was/is in use."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl Default for ContactPoint {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#system: Default::default(),
            r#value: Default::default(),
            r#use: Default::default(),
            r#rank: Default::default(),
            r#period: Default::default(),
        }
    }
}
