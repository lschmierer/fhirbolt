// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "Define members of the evidence element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicDefinition {
    Reference(Box<super::super::types::Reference>),
    Canonical(Box<super::super::types::Canonical>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Expression(Box<super::super::types::Expression>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
    #[default]
    Invalid,
}
#[doc = "Indicates what effective period the study covers."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicParticipantEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "A characteristic that defines the members of the evidence element. Multiple characteristics are applied with \"and\" semantics."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A short, natural language description of the characteristic that could be used to communicate the criteria to an end-user."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Define members of the evidence element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
    pub r#definition: EvidenceVariableCharacteristicDefinition,
    #[doc = "Use UsageContext to define the members of the population, such as Age Ranges, Genders, Settings."]
    pub r#usage_context: Vec<super::super::types::UsageContext>,
    #[doc = "When true, members with this characteristic are excluded from the element."]
    pub r#exclude: Option<super::super::types::Boolean>,
    #[doc = "Indicates what effective period the study covers."]
    pub r#participant_effective: Option<EvidenceVariableCharacteristicParticipantEffective>,
    #[doc = "Indicates duration from the participant's study entry."]
    pub r#time_from_start: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates how elements are aggregated within the study effective period."]
    pub r#group_measure: Option<super::super::types::Code>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceVariableCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#definition: Default::default(),
            r#usage_context: Default::default(),
            r#exclude: Default::default(),
            r#participant_effective: Default::default(),
            r#time_from_start: Default::default(),
            r#group_measure: Default::default(),
        }
    }
}
#[doc = "The EvidenceVariable resource describes a \"PICO\" element that knowledge (evidence, assertion, recommendation) is about.\n\nNeed to be able to define and reuse the definition of individual elements of a research question."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariable {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this evidence variable when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this evidence variable is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the evidence variable is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this evidence variable when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the evidence variable when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the evidence variable author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the evidence variable. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the evidence variable."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The short title provides an alternate title for use in informal descriptive contexts where the full, formal title is not necessary."]
    pub r#short_title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the EvidenceVariable giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "The status of this evidence variable. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The date  (and optionally time) when the evidence variable was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the evidence variable changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the evidence variable."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the evidence variable from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate evidence variable instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the evidence variable is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "A copyright statement relating to the evidence variable and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the evidence variable."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the evidence variable content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the EvidenceVariable. Topics provide a high-level categorization grouping types of EvidenceVariables that can be useful for filtering and searching."]
    pub r#topic: Vec<super::super::types::CodeableConcept>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "Related artifacts such as additional documentation, justification, or bibliographic references."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "The type of evidence element, a population, an exposure, or an outcome."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "A characteristic that defines the members of the evidence element. Multiple characteristics are applied with \"and\" semantics."]
    pub r#characteristic: Vec<EvidenceVariableCharacteristic>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceVariable {
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
            r#url: Default::default(),
            r#identifier: Default::default(),
            r#version: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#short_title: Default::default(),
            r#subtitle: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#copyright: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#topic: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#related_artifact: Default::default(),
            r#type: Default::default(),
            r#characteristic: Default::default(),
        }
    }
}
