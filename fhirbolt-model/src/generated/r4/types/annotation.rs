// Generated on 2023-04-16 by fhirbolt-codegen v0.2.0
#[doc = "The individual responsible for making the annotation."]
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationAuthor {
    Reference(Box<super::super::types::Reference>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for AnnotationAuthor {
    fn default() -> AnnotationAuthor {
        AnnotationAuthor::Invalid
    }
}
#[doc = "Base StructureDefinition for Annotation Type: A  text note which also  contains information about who made the statement and when."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Annotation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The individual responsible for making the annotation."]
    pub r#author: Option<AnnotationAuthor>,
    #[doc = "Indicates when this particular annotation was made."]
    pub r#time: Option<super::super::types::DateTime>,
    #[doc = "The text of the annotation in markdown format."]
    pub r#text: super::super::types::Markdown,
}
