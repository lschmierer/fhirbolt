// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "This resource provides the insurance enrollment details to the insurer regarding a specified coverage."]
#[derive(Debug, Clone, PartialEq)]
pub struct EnrollmentRequest {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
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
    #[doc = "The Response business identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of the resource instance."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The date when this resource was created."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "The Insurer who is target  of the request."]
    pub r#insurer: Option<Box<super::super::types::Reference>>,
    #[doc = "The practitioner who is responsible for the services rendered to the patient."]
    pub r#provider: Option<Box<super::super::types::Reference>>,
    #[doc = "Patient Resource."]
    pub r#candidate: Option<Box<super::super::types::Reference>>,
    #[doc = "Reference to the program or plan identification, underwriter or payor."]
    pub r#coverage: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for EnrollmentRequest {
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
            r#status: Default::default(),
            r#created: Default::default(),
            r#insurer: Default::default(),
            r#provider: Default::default(),
            r#candidate: Default::default(),
            r#coverage: Default::default(),
        }
    }
}
