// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Base StructureDefinition for Period Type: A time period defined by a start and end date and optionally time."]
#[derive(Debug, Clone, PartialEq)]
pub struct Period {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The start of the period. The boundary is inclusive."]
    pub r#start: Option<super::super::types::DateTime>,
    #[doc = "The end of the period. If the end of the period is missing, it means no end was known or planned at the time the instance was created. The start may be in the past, and the end date in the future, which means that period is expected/planned to end at that time."]
    pub r#end: Option<super::super::types::DateTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for Period {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
        }
    }
}
