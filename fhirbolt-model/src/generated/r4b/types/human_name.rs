// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "Base StructureDefinition for HumanName Type: A human's name with the ability to identify parts and usage.\n\nNeed to be able to record names, along with notes about their use."]
#[derive(Debug, Clone, PartialEq)]
pub struct HumanName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies the purpose for this name."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "Specifies the entire name as it should be displayed e.g. on an application UI. This may be provided instead of or as well as the specific parts."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The part of a name that links to the genealogy. In some cultures (e.g. Eritrea) the family name of a son is the first name of his father."]
    pub r#family: Option<super::super::types::String>,
    #[doc = "Given name."]
    pub r#given: Vec<super::super::types::String>,
    #[doc = "Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the start of the name."]
    pub r#prefix: Vec<super::super::types::String>,
    #[doc = "Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the end of the name."]
    pub r#suffix: Vec<super::super::types::String>,
    #[doc = "Indicates the period of time when this name was valid for the named person."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for HumanName {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#use: Default::default(),
            r#text: Default::default(),
            r#family: Default::default(),
            r#given: Default::default(),
            r#prefix: Default::default(),
            r#suffix: Default::default(),
            r#period: Default::default(),
        }
    }
}
