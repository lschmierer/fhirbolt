// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "Defines the characteristic when paired with characteristic.type."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicDefinitionByTypeAndValueValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
    Id(Box<super::super::types::Id>),
    #[default]
    Invalid,
}
#[doc = "Number of occurrences meeting the characteristic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicInstances {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "Length of time in which the characteristic is met."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicDuration {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "The event used as a base point (reference point) in time."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicTimeFromEventEvent {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    DateTime(Box<super::super::types::DateTime>),
    Id(Box<super::super::types::Id>),
    #[default]
    Invalid,
}
#[doc = "Definition of the grouping."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVariableCategoryValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "Defines the characteristic using both a type and value\\[x\\] elements."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCharacteristicDefinitionByTypeAndValue {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Used to express the type of characteristic."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Method for how the characteristic value was determined."]
    pub r#method: Vec<super::super::types::CodeableConcept>,
    #[doc = "Device used for determining characteristic."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Defines the characteristic when paired with characteristic.type."]
    pub r#value: EvidenceVariableCharacteristicDefinitionByTypeAndValueValue,
    #[doc = "Defines the reference point for comparison when valueQuantity or valueRange is not compared to zero."]
    pub r#offset: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceVariableCharacteristicDefinitionByTypeAndValue {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#method: Default::default(),
            r#device: Default::default(),
            r#value: Default::default(),
            r#offset: Default::default(),
        }
    }
}
#[doc = "Defines the characteristic as a combination of two or more characteristics."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCharacteristicDefinitionByCombination {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Used to specify if two or more characteristics are combined with OR or AND."]
    pub r#code: super::super::types::Code,
    #[doc = "Provides the value of \"n\" when \"at-least\" or \"at-most\" codes are used."]
    pub r#threshold: Option<super::super::types::PositiveInt>,
    #[doc = "A defining factor of the characteristic."]
    pub r#characteristic: Vec<EvidenceVariableCharacteristic>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceVariableCharacteristicDefinitionByCombination {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#threshold: Default::default(),
            r#characteristic: Default::default(),
        }
    }
}
#[doc = "Timing in which the characteristic is determined."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCharacteristicTimeFromEvent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Human readable description."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the timeFromEvent."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The event used as a base point (reference point) in time."]
    pub r#event: Option<EvidenceVariableCharacteristicTimeFromEventEvent>,
    #[doc = "Used to express the observation at a defined amount of time before or after the event."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Used to express the observation within a period before and/or after the event."]
    pub r#range: Option<Box<super::super::types::Range>>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceVariableCharacteristicTimeFromEvent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#event: Default::default(),
            r#quantity: Default::default(),
            r#range: Default::default(),
        }
    }
}
#[doc = "A defining factor of the EvidenceVariable. Multiple characteristics are applied with \"and\" semantics."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Label used for when a characteristic refers to another characteristic."]
    pub r#link_id: Option<super::super::types::Id>,
    #[doc = "A short, natural language description of the characteristic that could be used to communicate the criteria to an end-user."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the characteristic."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "When true, this characteristic is an exclusion criterion. In other words, not matching this characteristic definition is equivalent to meeting this criterion."]
    pub r#exclude: Option<super::super::types::Boolean>,
    #[doc = "Defines the characteristic using a Reference."]
    pub r#definition_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Defines the characteristic using Canonical."]
    pub r#definition_canonical: Option<super::super::types::Canonical>,
    #[doc = "Defines the characteristic using CodeableConcept."]
    pub r#definition_codeable_concept: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Defines the characteristic using Expression."]
    pub r#definition_expression: Option<Box<super::super::types::Expression>>,
    #[doc = "Defines the characteristic using id."]
    pub r#definition_id: Option<super::super::types::Id>,
    #[doc = "Defines the characteristic using both a type and value\\[x\\] elements."]
    pub r#definition_by_type_and_value:
        Option<EvidenceVariableCharacteristicDefinitionByTypeAndValue>,
    #[doc = "Defines the characteristic as a combination of two or more characteristics."]
    pub r#definition_by_combination: Option<EvidenceVariableCharacteristicDefinitionByCombination>,
    #[doc = "Number of occurrences meeting the characteristic."]
    pub r#instances: Option<EvidenceVariableCharacteristicInstances>,
    #[doc = "Length of time in which the characteristic is met."]
    pub r#duration: Option<EvidenceVariableCharacteristicDuration>,
    #[doc = "Timing in which the characteristic is determined."]
    pub r#time_from_event: Vec<EvidenceVariableCharacteristicTimeFromEvent>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceVariableCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#exclude: Default::default(),
            r#definition_reference: Default::default(),
            r#definition_canonical: Default::default(),
            r#definition_codeable_concept: Default::default(),
            r#definition_expression: Default::default(),
            r#definition_id: Default::default(),
            r#definition_by_type_and_value: Default::default(),
            r#definition_by_combination: Default::default(),
            r#instances: Default::default(),
            r#duration: Default::default(),
            r#time_from_event: Default::default(),
        }
    }
}
#[doc = "A grouping for ordinal or polychotomous variables."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCategory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Description of the grouping."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Definition of the grouping."]
    pub r#value: Option<EvidenceVariableCategoryValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceVariableCategory {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "The EvidenceVariable resource describes an element that knowledge (Evidence) is about.\n\nNeed to be able to define and reuse the definition of individual elements of a research question."]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this evidence variable when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this evidence variable is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the evidence variable is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this evidence variable when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the evidence variable when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the evidence variable author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<EvidenceVariableVersionAlgorithm>,
    #[doc = "A natural language name identifying the evidence variable. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the evidence variable."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The short title provides an alternate title for use in informal descriptive contexts where the full, formal title is not necessary."]
    pub r#short_title: Option<super::super::types::String>,
    #[doc = "The status of this evidence variable. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this resource is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the evidence variable was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the evidence variable changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the evidence variable."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the evidence variable from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate evidence variable instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "Explanation of why this EvidenceVariable is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the resource and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the resource."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage.\n\nSee guidance around (not) making local changes to elements [here](canonicalresource.html#localization)."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the resource content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization asserted by the publisher to be primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization asserted by the publisher to be responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "Related artifacts such as additional documentation, justification, or bibliographic references."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "True if the actual variable measured, false if a conceptual representation of the intended variable."]
    pub r#actual: Option<super::super::types::Boolean>,
    #[doc = "A defining factor of the EvidenceVariable. Multiple characteristics are applied with \"and\" semantics."]
    pub r#characteristic: Vec<EvidenceVariableCharacteristic>,
    #[doc = "The method of handling in statistical analysis."]
    pub r#handling: Option<super::super::types::Code>,
    #[doc = "A grouping for ordinal or polychotomous variables."]
    pub r#category: Vec<EvidenceVariableCategory>,
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
            r#version_algorithm: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#short_title: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#use_context: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#copyright_label: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#related_artifact: Default::default(),
            r#actual: Default::default(),
            r#characteristic: Default::default(),
            r#handling: Default::default(),
            r#category: Default::default(),
        }
    }
}
