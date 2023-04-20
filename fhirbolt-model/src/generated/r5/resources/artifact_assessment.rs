// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Display of or reference to the bibliographic citation of the comment, classifier, or rating."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ArtifactAssessmentCiteAs {
    Reference(Box<super::super::types::Reference>),
    Markdown(Box<super::super::types::Markdown>),
    #[default]
    Invalid,
}
#[doc = "A reference to a resource, canonical resource, or non-FHIR resource which the comment or assessment is about."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ArtifactAssessmentArtifact {
    Reference(Box<super::super::types::Reference>),
    Canonical(Box<super::super::types::Canonical>),
    Uri(Box<super::super::types::Uri>),
    #[default]
    Invalid,
}
#[doc = "A component comment, classifier, or rating of the artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct ArtifactAssessmentContent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of information this component of the content represents."]
    pub r#information_type: Option<super::super::types::Code>,
    #[doc = "A brief summary of the content of this component."]
    pub r#summary: Option<super::super::types::Markdown>,
    #[doc = "Indicates what type of content this component represents."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Represents a rating, classifier, or assessment of the artifact."]
    pub r#classifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A quantitative rating of the artifact."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Indicates who or what authored the content."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "A URI that points to what the comment is about, such as a line of text in the CQL, or a specific element in a resource."]
    pub r#path: Vec<super::super::types::Uri>,
    #[doc = "Additional related artifacts that provide supporting documentation, additional evidence, or further information related to the content."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "Acceptable to publicly share the comment, classifier or rating."]
    pub r#free_to_share: Option<super::super::types::Boolean>,
    #[doc = "If the informationType is container, the components of the content."]
    pub r#component: Vec<ArtifactAssessmentContent>,
}
impl Default for ArtifactAssessmentContent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#information_type: Default::default(),
            r#summary: Default::default(),
            r#type: Default::default(),
            r#classifier: Default::default(),
            r#quantity: Default::default(),
            r#author: Default::default(),
            r#path: Default::default(),
            r#related_artifact: Default::default(),
            r#free_to_share: Default::default(),
            r#component: Default::default(),
        }
    }
}
#[doc = "This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content."]
#[derive(Debug, Clone, PartialEq)]
pub struct ArtifactAssessment {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A formal identifier that is used to identify this artifact assessment when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A short title for the assessment for use in displaying and selecting."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Display of or reference to the bibliographic citation of the comment, classifier, or rating."]
    pub r#cite_as: Option<ArtifactAssessmentCiteAs>,
    #[doc = "The date  (and optionally time) when the artifact assessment was published. The date must change when the disposition changes and it must change if the workflow status code changes. In addition, it should change when the substantive content of the artifact assessment changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "A copyright statement relating to the artifact assessment and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the artifact assessment."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "A reference to a resource, canonical resource, or non-FHIR resource which the comment or assessment is about."]
    pub r#artifact: ArtifactAssessmentArtifact,
    #[doc = "A component comment, classifier, or rating of the artifact."]
    pub r#content: Vec<ArtifactAssessmentContent>,
    #[doc = "Indicates the workflow status of the comment or change request."]
    pub r#workflow_status: Option<super::super::types::Code>,
    #[doc = "Indicates the disposition of the responsible party to the comment or change request."]
    pub r#disposition: Option<super::super::types::Code>,
}
impl Default for ArtifactAssessment {
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
            r#title: Default::default(),
            r#cite_as: Default::default(),
            r#date: Default::default(),
            r#copyright: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#artifact: Default::default(),
            r#content: Default::default(),
            r#workflow_status: Default::default(),
            r#disposition: Default::default(),
        }
    }
}
