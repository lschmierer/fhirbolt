// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "What address or number needs to be used for a user to connect to the virtual service to join. The channelType informs as to which datatype is appropriate to use (requires knowledge of the specific type)."]
#[derive(Debug, Clone, PartialEq)]
pub enum VirtualServiceDetailAddress {
    Url(Box<super::super::types::Url>),
    String(Box<super::super::types::String>),
    ContactPoint(Box<super::super::types::ContactPoint>),
    ExtendedContactDetail(Box<super::super::types::ExtendedContactDetail>),
    Invalid,
}
impl Default for VirtualServiceDetailAddress {
    fn default() -> VirtualServiceDetailAddress {
        VirtualServiceDetailAddress::Invalid
    }
}
#[doc = "VirtualServiceDetail Type: Virtual Service Contact Details."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VirtualServiceDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of virtual service to connect to (i.e. Teams, Zoom, Specific VMR technology, WhatsApp)."]
    pub r#channel_type: Option<Box<super::super::types::Coding>>,
    #[doc = "What address or number needs to be used for a user to connect to the virtual service to join. The channelType informs as to which datatype is appropriate to use (requires knowledge of the specific type)."]
    pub r#address: Option<VirtualServiceDetailAddress>,
    #[doc = "Address to see alternative connection details."]
    pub r#additional_info: Vec<super::super::types::Url>,
    #[doc = "Maximum number of participants supported by the virtual service."]
    pub r#max_participants: Option<super::super::types::PositiveInt>,
    #[doc = "Session Key required by the virtual service."]
    pub r#session_key: Option<super::super::types::String>,
}
