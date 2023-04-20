// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.\n\nThere are a number of places where content must be signed in healthcare."]
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An indication of the reason that the entity signed this document. This may be explicitly included as part of the signature information and can be used when determining accountability for various actions concerning the document."]
    pub r#type: Vec<Box<super::super::types::Coding>>,
    #[doc = "When the digital signature was signed."]
    pub r#when: Option<super::super::types::Instant>,
    #[doc = "A reference to an application-usable description of the identity that signed  (e.g. the signature used their private key)."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to an application-usable description of the identity that is represented by the signature."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "A mime type that indicates the technical format of the target resources signed by the signature."]
    pub r#target_format: Option<super::super::types::Code>,
    #[doc = "A mime type that indicates the technical format of the signature. Important mime types are application/signature+xml for X ML DigSig, application/jose for JWS, and image/* for a graphical image of a signature, etc."]
    pub r#sig_format: Option<super::super::types::Code>,
    #[doc = "The base64 encoding of the Signature content. When signature is not recorded electronically this element would be empty."]
    pub r#data: Option<super::super::types::Base64Binary>,
}
impl Default for Signature {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#type: Default::default(),
            r#when: Default::default(),
            r#who: Default::default(),
            r#on_behalf_of: Default::default(),
            r#target_format: Default::default(),
            r#sig_format: Default::default(),
            r#data: Default::default(),
        }
    }
}
