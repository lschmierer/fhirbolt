// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Record details about an anatomical structure.  This resource may be used when a coded concept does not provide the necessary detail needed for the use case."]
#[derive(Debug, Clone, PartialEq)]
pub struct BodyStructure {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifier for this instance of the anatomical structure."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this body site is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The kind of structure being represented by the body structure at `BodyStructure.location`.  This can define both normal and abnormal morphologies."]
    pub r#morphology: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The anatomical location or region of the specimen, lesion, or body structure."]
    pub r#location: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Qualifier to refine the anatomical location.  These include qualifiers for laterality, relative location, directionality, number, and plane."]
    pub r#location_qualifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A summary, characterization or explanation of the body structure."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Image or images used to identify a location."]
    pub r#image: Vec<Box<super::super::types::Attachment>>,
    #[doc = "The person to which the body site belongs."]
    pub r#patient: Box<super::super::types::Reference>,
}
impl Default for BodyStructure {
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
            r#active: Default::default(),
            r#morphology: Default::default(),
            r#location: Default::default(),
            r#location_qualifier: Default::default(),
            r#description: Default::default(),
            r#image: Default::default(),
            r#patient: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
