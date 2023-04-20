// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RequirementsVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "The actual statement of requirement, in markdown format."]
#[derive(Debug, Clone, PartialEq)]
pub struct RequirementsStatement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Key that identifies this statement (unique within this resource)."]
    pub r#key: super::super::types::Id,
    #[doc = "A short human usable label for this statement."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "A short human usable label for this statement."]
    pub r#conformance: Vec<super::super::types::Code>,
    #[doc = "This boolean flag is set to true of the text of the requirement is conditional on something e.g. it includes lanauage like 'if x then y'. This conditionality flag is introduced for purposes of filtering and colour highlighting etc."]
    pub r#conditionality: Option<super::super::types::Boolean>,
    #[doc = "The actual requirement for human consumption."]
    pub r#requirement: super::super::types::Markdown,
    #[doc = "Another statement on one of the requirements that this requirement clarifies or restricts."]
    pub r#derived_from: Option<super::super::types::String>,
    #[doc = "A larger requirement that this requirement helps to refine and enable."]
    pub r#parent: Option<super::super::types::String>,
    #[doc = "A reference to another artifact that satisfies this requirement. This could be a Profile, extension, or an element in one of those, or a CapabilityStatement, OperationDefinition, SearchParameter, CodeSystem(/code), ValueSet, Libary etc."]
    pub r#satisfied_by: Vec<super::super::types::Url>,
    #[doc = "A reference to another artifact that created this requirement. This could be a Profile, etc., or external regulation, or business requirements expressed elsewhere."]
    pub r#reference: Vec<super::super::types::Url>,
    #[doc = "Who asked for this statement to be a requirement. By default, it's assumed that the publisher knows who it is if it matters."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl Default for RequirementsStatement {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#key: {
                let mut default: super::super::types::Id = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#label: Default::default(),
            r#conformance: Default::default(),
            r#conditionality: Default::default(),
            r#requirement: {
                let mut default: super::super::types::Markdown = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#derived_from: Default::default(),
            r#parent: Default::default(),
            r#satisfied_by: Default::default(),
            r#reference: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "The Requirements resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays."]
#[derive(Debug, Clone, PartialEq)]
pub struct Requirements {
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
    #[doc = "An absolute URI that is used to identify this Requirements when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this Requirements is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the Requirements is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this Requirements when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the Requirements when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the Requirements author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<RequirementsVersionAlgorithm>,
    #[doc = "A natural language name identifying the Requirements. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the Requirements."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this Requirements. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this Requirements is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the Requirements was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the Requirements changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the Requirements."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the requirements."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate Requirements instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the Requirements is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this Requirements is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the Requirements and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the Requirements."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "Another set of Requirements that this set of Requirements builds on and updates."]
    pub r#derived_from: Vec<super::super::types::Canonical>,
    #[doc = "A reference to another artifact that created this set of requirements. This could be a Profile, etc., or external regulation, or business requirements expressed elsewhere."]
    pub r#reference: Vec<super::super::types::Url>,
    #[doc = "An actor these requirements are in regard to."]
    pub r#actor: Vec<super::super::types::Canonical>,
    #[doc = "The actual statement of requirement, in markdown format."]
    pub r#statement: Vec<RequirementsStatement>,
}
impl Default for Requirements {
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
            r#version_algorithm: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#copyright_label: Default::default(),
            r#derived_from: Default::default(),
            r#reference: Default::default(),
            r#actor: Default::default(),
            r#statement: Default::default(),
        }
    }
}
