// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Indicates the mechanism used to compare versions to determine which CodeSystem is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CodeSystemVersionAlgorithm {
    String(super::super::types::String),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "The value of this property."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CodeSystemConceptPropertyValue {
    Code(super::super::types::Code),
    Coding(Box<super::super::types::Coding>),
    String(super::super::types::String),
    Integer(super::super::types::Integer),
    Boolean(super::super::types::Boolean),
    DateTime(super::super::types::DateTime),
    Decimal(super::super::types::Decimal),
    #[default]
    Invalid,
}
#[doc = "A filter that can be used in a value set compose statement when selecting concepts using a filter."]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSystemFilter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The code that identifies this filter when it is used as a filter in [ValueSet](valueset.html#).compose.include.filter."]
    pub r#code: super::super::types::Code,
    #[doc = "A description of how or why the filter is used."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A list of operators that can be used with the filter."]
    pub r#operator: Vec<super::super::types::Code>,
    #[doc = "A description of what the value for the filter should be."]
    pub r#value: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for CodeSystemFilter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
            r#operator: Default::default(),
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "A property defines an additional slot through which additional information can be provided about a concept."]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSystemProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code that is used to identify the property. The code is used internally (in CodeSystem.concept.property.code) and also externally, such as in property filters."]
    pub r#code: super::super::types::Code,
    #[doc = "Reference to the formal meaning of the property. One possible source of meaning is the [Concept Properties](https://hl7.org/FHIR/codesystem-concept-properties.html)) code system."]
    pub r#uri: Option<super::super::types::Uri>,
    #[doc = "A description of the property- why it is defined, and how its value might be used."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The type of the property value. Properties of type \"code\" contain a code defined by the code system (e.g. a reference to another defined concept)."]
    pub r#type: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for CodeSystemProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#uri: Default::default(),
            r#description: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Additional representations for the concept - other languages, aliases, specialized purposes, used for particular purposes, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSystemConceptDesignation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The language this designation is defined for."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A code that details how this designation would be used."]
    pub r#use: Option<Box<super::super::types::Coding>>,
    #[doc = "Additional codes that detail how this designation would be used, if there is more than one use."]
    pub r#additional_use: Vec<super::super::types::Coding>,
    #[doc = "The text value for this designation."]
    pub r#value: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for CodeSystemConceptDesignation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#language: Default::default(),
            r#use: Default::default(),
            r#additional_use: Default::default(),
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "A property value for this concept."]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSystemConceptProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code that is a reference to CodeSystem.property.code."]
    pub r#code: super::super::types::Code,
    #[doc = "The value of this property."]
    pub r#value: CodeSystemConceptPropertyValue,
}
#[allow(clippy::derivable_impls)]
impl Default for CodeSystemConceptProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: Default::default(),
        }
    }
}
#[doc = "Concepts that are in the code system. The concept definitions are inherently hierarchical, but the definitions must be consulted to determine what the meanings of the hierarchical relationships are."]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSystemConcept {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code - a text symbol - that uniquely identifies the concept within the code system."]
    pub r#code: super::super::types::Code,
    #[doc = "A human readable string that is the recommended default way to present this concept to a user."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "The formal definition of the concept. The code system resource does not make formal definitions required, because of the prevalence of legacy systems. However, they are highly recommended, as without them there is no formal meaning associated with the concept."]
    pub r#definition: Option<super::super::types::String>,
    #[doc = "Additional representations for the concept - other languages, aliases, specialized purposes, used for particular purposes, etc."]
    pub r#designation: Vec<CodeSystemConceptDesignation>,
    #[doc = "A property value for this concept."]
    pub r#property: Vec<CodeSystemConceptProperty>,
    #[doc = "Defines children of a concept to produce a hierarchy of concepts. The nature of the relationships is variable (is-a/contains/categorizes) - see hierarchyMeaning."]
    pub r#concept: Vec<CodeSystemConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for CodeSystemConcept {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#display: Default::default(),
            r#definition: Default::default(),
            r#designation: Default::default(),
            r#property: Default::default(),
            r#concept: Default::default(),
        }
    }
}
#[doc = "The CodeSystem resource is used to declare the existence of and describe a code system or code system supplement and its key properties, and optionally define a part or all of its content."]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSystem {
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
    #[doc = "An absolute URI that is used to identify this code system when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this code system is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the code system is stored on different servers. This is used in [Coding](datatypes.html#Coding).system."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this code system when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the code system when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the code system author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. This is used in [Coding](datatypes.html#Coding).version."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which CodeSystem is more current."]
    pub r#version_algorithm: Option<CodeSystemVersionAlgorithm>,
    #[doc = "A natural language name identifying the code system. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the code system."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this code system. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this code system is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the code system was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the code system changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the code system."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the code system from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate code system instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the code system is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this code system is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the code system and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the code system."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the CodeSystem content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptions related to the content of the CodeSystem. Topics provide a high-level categorization as well as keywords for the CodeSystem that can be useful for filtering and searching."]
    pub r#topic: Vec<super::super::types::CodeableConcept>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the CodeSystem."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the CodeSystem."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization asserted by the publisher to be primarily responsible for review of some aspect of the CodeSystem."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization asserted by the publisher to be responsible for officially endorsing the CodeSystem for use in some setting."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "Related artifacts such as additional documentation, justification, dependencies, bibliographic references, and predecessor and successor artifacts."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "If code comparison is case sensitive when codes within this system are compared to each other."]
    pub r#case_sensitive: Option<super::super::types::Boolean>,
    #[doc = "Canonical reference to the value set that contains all codes in the code system independent of code status."]
    pub r#value_set: Option<super::super::types::Canonical>,
    #[doc = "The meaning of the hierarchy of concepts as represented in this resource."]
    pub r#hierarchy_meaning: Option<super::super::types::Code>,
    #[doc = "The code system defines a compositional (post-coordination) grammar."]
    pub r#compositional: Option<super::super::types::Boolean>,
    #[doc = "This flag is used to signify that the code system does not commit to concept permanence across versions. If true, a version must be specified when referencing this code system."]
    pub r#version_needed: Option<super::super::types::Boolean>,
    #[doc = "The extent of the content of the code system (the concepts and codes it defines) are represented in this resource instance."]
    pub r#content: super::super::types::Code,
    #[doc = "The canonical URL of the code system that this code system supplement is adding designations and properties to."]
    pub r#supplements: Option<super::super::types::Canonical>,
    #[doc = "The total number of concepts defined by the code system. Where the code system has a compositional grammar, the basis of this count is defined by the system steward."]
    pub r#count: Option<super::super::types::UnsignedInt>,
    #[doc = "A filter that can be used in a value set compose statement when selecting concepts using a filter."]
    pub r#filter: Vec<CodeSystemFilter>,
    #[doc = "A property defines an additional slot through which additional information can be provided about a concept."]
    pub r#property: Vec<CodeSystemProperty>,
    #[doc = "Concepts that are in the code system. The concept definitions are inherently hierarchical, but the definitions must be consulted to determine what the meanings of the hierarchical relationships are."]
    pub r#concept: Vec<CodeSystemConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for CodeSystem {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
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
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#topic: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#related_artifact: Default::default(),
            r#case_sensitive: Default::default(),
            r#value_set: Default::default(),
            r#hierarchy_meaning: Default::default(),
            r#compositional: Default::default(),
            r#version_needed: Default::default(),
            r#content: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#supplements: Default::default(),
            r#count: Default::default(),
            r#filter: Default::default(),
            r#property: Default::default(),
            r#concept: Default::default(),
        }
    }
}
