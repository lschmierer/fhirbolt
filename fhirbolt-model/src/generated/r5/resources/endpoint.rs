// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The set of payloads that are provided/available at this endpoint."]
#[derive(Debug, Clone, PartialEq)]
pub struct EndpointPayload {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The payload type describes the acceptable content that can be communicated on the endpoint."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The mime type to send the payload in - e.g. application/fhir+xml, application/fhir+json. If the mime type is not specified, then the sender could send any content (including no content depending on the connectionType)."]
    pub r#mime_type: Vec<super::super::types::Code>,
}
#[allow(clippy::derivable_impls)]
impl Default for EndpointPayload {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#mime_type: Default::default(),
        }
    }
}
#[doc = "The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b, a REST endpoint for another FHIR server, or a s/Mime email address. This may include any security context information."]
#[derive(Debug, Clone, PartialEq)]
pub struct Endpoint {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier for the organization that is used to identify the endpoint across multiple disparate systems."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The endpoint status represents the general expected availability of an endpoint."]
    pub r#status: super::super::types::Code,
    #[doc = "A coded value that represents the technical details of the usage of this endpoint, such as what WSDLs should be used in what way. (e.g. XDS.b/DICOM/cds-hook)."]
    pub r#connection_type: Vec<super::super::types::CodeableConcept>,
    #[doc = "A friendly name that this endpoint can be referred to with."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The description of the endpoint and what it is for (typically used as supplemental information in an endpoint directory describing its usage/purpose)."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The type of environment(s) exposed at this endpoint (dev, prod, test, etc.)."]
    pub r#environment_type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The organization that manages this endpoint (even if technically another organization is hosting this in the cloud, it is the organization associated with the data)."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Contact details for a human to contact about the endpoint. The primary use of this for system administrator troubleshooting."]
    pub r#contact: Vec<super::super::types::ContactPoint>,
    #[doc = "The interval during which the endpoint is expected to be operational."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The set of payloads that are provided/available at this endpoint."]
    pub r#payload: Vec<EndpointPayload>,
    #[doc = "The uri that describes the actual end-point to connect to."]
    pub r#address: super::super::types::Url,
    #[doc = "Additional headers / information to send as part of the notification."]
    pub r#header: Vec<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for Endpoint {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#connection_type: Default::default(),
            r#name: Default::default(),
            r#description: Default::default(),
            r#environment_type: Default::default(),
            r#managing_organization: Default::default(),
            r#contact: Default::default(),
            r#period: Default::default(),
            r#payload: Default::default(),
            r#address: super::super::types::Url {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#header: Default::default(),
        }
    }
}
