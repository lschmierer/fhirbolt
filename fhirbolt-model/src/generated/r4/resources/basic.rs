// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Basic is used for handling concepts not yet defined in FHIR, narrative-only resources that don't map to an existing resource, and custom resources not appropriate for inclusion in the FHIR specification.\n\nNeed some way to safely (without breaking interoperability) allow implementers to exchange content not supported by the initial set of declared resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct Basic {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier assigned to the resource for business purposes, outside the context of FHIR."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Identifies the 'type' of resource - equivalent to the resource name for other resources."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the patient, practitioner, device or any other resource that is the \"focus\" of this resource."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies when the resource was first created."]
    pub r#created: Option<super::super::types::Date>,
    #[doc = "Indicates who was responsible for creating the resource instance."]
    pub r#author: Option<Box<super::super::types::Reference>>,
}
impl Default for Basic {
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
            r#code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#subject: Default::default(),
            r#created: Default::default(),
            r#author: Default::default(),
        }
    }
}
