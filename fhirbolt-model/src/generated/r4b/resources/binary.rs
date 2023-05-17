// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "A resource that represents the data of a single raw artifact as digital content accessible in its native format.  A Binary resource can contain any content, whether text, image, pdf, zip archive, etc.\n\nThere are situations where it is useful or required to handle pure binary content using the same framework as other resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "MimeType of the binary content represented as a standard MimeType (BCP 13)."]
    pub r#content_type: super::super::types::Code,
    #[doc = "This element identifies another resource that can be used as a proxy of the security sensitivity to use when deciding and enforcing access control rules for the Binary resource. Given that the Binary resource contains very few elements that can be used to determine the sensitivity of the data and relationships to individuals, the referenced resource stands in as a proxy equivalent for this purpose. This referenced resource may be related to the Binary (e.g. Media, DocumentReference), or may be some non-related Resource purely as a security proxy. E.g. to identify that the binary resource relates to a patient, and access should only be granted to applications that have access to the patient."]
    pub r#security_context: Option<Box<super::super::types::Reference>>,
    #[doc = "The actual content, base64 encoded."]
    pub r#data: Option<super::super::types::Base64Binary>,
}
#[allow(clippy::derivable_impls)]
impl Default for Binary {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#content_type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#security_context: Default::default(),
            r#data: Default::default(),
        }
    }
}
