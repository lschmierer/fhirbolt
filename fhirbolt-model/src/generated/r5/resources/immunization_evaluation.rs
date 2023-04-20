// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "Describes a comparison of an immunization event against published recommendations to determine if the administration is \"valid\" in relation to those  recommendations."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImmunizationEvaluation {
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
    #[doc = "A unique identifier assigned to this immunization evaluation record."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the current status of the evaluation of the vaccination administration event."]
    pub r#status: super::super::types::Code,
    #[doc = "The individual for whom the evaluation is being done."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date the evaluation of the vaccine administration event was performed."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Indicates the authority who published the protocol (e.g. ACIP)."]
    pub r#authority: Option<Box<super::super::types::Reference>>,
    #[doc = "The vaccine preventable disease the dose is being evaluated against."]
    pub r#target_disease: Box<super::super::types::CodeableConcept>,
    #[doc = "The vaccine administration event being evaluated."]
    pub r#immunization_event: Box<super::super::types::Reference>,
    #[doc = "Indicates if the dose is valid or not valid with respect to the published recommendations."]
    pub r#dose_status: Box<super::super::types::CodeableConcept>,
    #[doc = "Provides an explanation as to why the vaccine administration event is valid or not relative to the published recommendations."]
    pub r#dose_status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional information about the evaluation."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "One possible path to achieve presumed immunity against a disease - within the context of an authority."]
    pub r#series: Option<super::super::types::String>,
    #[doc = "Nominal position in a series as determined by the outcome of the evaluation process."]
    pub r#dose_number: Option<super::super::types::String>,
    #[doc = "The recommended number of doses to achieve immunity as determined by the outcome of the evaluation process."]
    pub r#series_doses: Option<super::super::types::String>,
}
