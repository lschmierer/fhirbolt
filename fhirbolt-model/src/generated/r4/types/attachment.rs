// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Base StructureDefinition for Attachment Type: For referring to data content defined in other formats.\n\nMany models need to include data defined in other specifications that is complex and opaque to the healthcare model. This includes documents, media recordings, structured data, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct Attachment {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies the type of the data in the attachment and allows a method to be chosen to interpret or render the data. Includes mime type parameters such as charset where appropriate."]
    pub r#content_type: Option<super::super::types::Code>,
    #[doc = "The human language of the content. The value can be any valid value according to BCP 47."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "The actual data of the attachment - a sequence of bytes, base64 encoded."]
    pub r#data: Option<super::super::types::Base64Binary>,
    #[doc = "A location where the data can be accessed."]
    pub r#url: Option<super::super::types::Url>,
    #[doc = "The number of bytes of data that make up this attachment (before base64 encoding, if that is done)."]
    pub r#size: Option<super::super::types::UnsignedInt>,
    #[doc = "The calculated hash of the data using SHA-1. Represented using base64."]
    pub r#hash: Option<super::super::types::Base64Binary>,
    #[doc = "A label or set of text to display in place of the data."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The date that the attachment was first created."]
    pub r#creation: Option<super::super::types::DateTime>,
}
impl Default for Attachment {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#content_type: Default::default(),
            r#language: Default::default(),
            r#data: Default::default(),
            r#url: Default::default(),
            r#size: Default::default(),
            r#hash: Default::default(),
            r#title: Default::default(),
            r#creation: Default::default(),
        }
    }
}
