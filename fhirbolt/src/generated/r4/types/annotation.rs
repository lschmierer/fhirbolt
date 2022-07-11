// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum AnnotationAuthor {
    Reference(Box<super::super::types::Reference>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct Annotation {
    pub r#text: super::super::types::Markdown,
    pub r#id: Option<std::string::String>,
    pub r#time: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#author: Option<AnnotationAuthor>,
}
