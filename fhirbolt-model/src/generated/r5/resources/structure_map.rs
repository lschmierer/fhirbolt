// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum StructureMapVersionAlgorithm {
    String(super::super::types::String),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "Parameter value - variable or literal."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum StructureMapGroupRuleTargetParameterValue {
    Id(super::super::types::Id),
    String(super::super::types::String),
    Boolean(super::super::types::Boolean),
    Integer(super::super::types::Integer),
    Decimal(super::super::types::Decimal),
    Date(super::super::types::Date),
    Time(super::super::types::Time),
    DateTime(super::super::types::DateTime),
    #[default]
    Invalid,
}
#[doc = "A structure definition used by this map. The structure definition may describe instances that are converted, or the instances that are produced."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapStructure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The canonical reference to the structure."]
    pub r#url: super::super::types::Canonical,
    #[doc = "How the referenced structure is used in this mapping."]
    pub r#mode: super::super::types::Code,
    #[doc = "The name used for this type in the map."]
    pub r#alias: Option<super::super::types::String>,
    #[doc = "Documentation that describes how the structure is used in the mapping."]
    pub r#documentation: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapStructure {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#url: super::super::types::Canonical {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#mode: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#alias: Default::default(),
            r#documentation: Default::default(),
        }
    }
}
#[doc = "Definition of a constant value used in the map rules."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapConst {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Other maps used by this map (canonical URLs)."]
    pub r#name: Option<super::super::types::Id>,
    #[doc = "A FHIRPath expression that is the value of this variable."]
    pub r#value: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapConst {
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
#[doc = "A name assigned to an instance of data. The instance must be provided when the mapping is invoked."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroupInput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Name for this instance of data."]
    pub r#name: super::super::types::Id,
    #[doc = "Type for this instance of data."]
    pub r#type: Option<super::super::types::String>,
    #[doc = "Mode for this instance of data."]
    pub r#mode: super::super::types::Code,
    #[doc = "Documentation for this instance of data."]
    pub r#documentation: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapGroupInput {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Default::default(),
            r#mode: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#documentation: Default::default(),
        }
    }
}
#[doc = "Source inputs to the mapping."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Type or variable this rule applies to."]
    pub r#context: super::super::types::Id,
    #[doc = "Specified minimum cardinality for the element. This is optional; if present, it acts an implicit check on the input content."]
    pub r#min: Option<super::super::types::Integer>,
    #[doc = "Specified maximum cardinality for the element - a number or a \"*\". This is optional; if present, it acts an implicit check on the input content (* just serves as documentation; it's the default value)."]
    pub r#max: Option<super::super::types::String>,
    #[doc = "Specified type for the element. This works as a condition on the mapping - use for polymorphic elements."]
    pub r#type: Option<super::super::types::String>,
    #[doc = "A value to use if there is no existing value in the source object."]
    pub r#default_value: Option<super::super::types::String>,
    #[doc = "Optional field for this source."]
    pub r#element: Option<super::super::types::String>,
    #[doc = "How to handle the list mode for this element."]
    pub r#list_mode: Option<super::super::types::Code>,
    #[doc = "Named context for field, if a field is specified."]
    pub r#variable: Option<super::super::types::Id>,
    #[doc = "FHIRPath expression  - must be true or the rule does not apply."]
    pub r#condition: Option<super::super::types::String>,
    #[doc = "FHIRPath expression  - must be true or the mapping engine throws an error instead of completing."]
    pub r#check: Option<super::super::types::String>,
    #[doc = "A FHIRPath expression which specifies a message to put in the transform log when content matching the source rule is found."]
    pub r#log_message: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapGroupRuleSource {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#context: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#min: Default::default(),
            r#max: Default::default(),
            r#type: Default::default(),
            r#default_value: Default::default(),
            r#element: Default::default(),
            r#list_mode: Default::default(),
            r#variable: Default::default(),
            r#condition: Default::default(),
            r#check: Default::default(),
            r#log_message: Default::default(),
        }
    }
}
#[doc = "Parameters to the transform."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleTargetParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Parameter value - variable or literal."]
    pub r#value: StructureMapGroupRuleTargetParameterValue,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapGroupRuleTargetParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "Content to create because of this mapping rule."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Variable this rule applies to."]
    pub r#context: Option<super::super::types::String>,
    #[doc = "Field to create in the context."]
    pub r#element: Option<super::super::types::String>,
    #[doc = "Named context for field, if desired, and a field is specified."]
    pub r#variable: Option<super::super::types::Id>,
    #[doc = "If field is a list, how to manage the list."]
    pub r#list_mode: Vec<super::super::types::Code>,
    #[doc = "Internal rule reference for shared list items."]
    pub r#list_rule_id: Option<super::super::types::Id>,
    #[doc = "How the data is copied / created."]
    pub r#transform: Option<super::super::types::Code>,
    #[doc = "Parameters to the transform."]
    pub r#parameter: Vec<StructureMapGroupRuleTargetParameter>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapGroupRuleTarget {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#context: Default::default(),
            r#element: Default::default(),
            r#variable: Default::default(),
            r#list_mode: Default::default(),
            r#list_rule_id: Default::default(),
            r#transform: Default::default(),
            r#parameter: Default::default(),
        }
    }
}
#[doc = "Which other rules to apply in the context of this rule."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleDependent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Name of a rule or group to apply."]
    pub r#name: super::super::types::Id,
    #[doc = "Parameter to pass to the rule or group."]
    pub r#parameter: Vec<StructureMapGroupRuleTargetParameter>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapGroupRuleDependent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#parameter: Default::default(),
        }
    }
}
#[doc = "Transform Rule from source to target."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroupRule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Name of the rule for internal references."]
    pub r#name: Option<super::super::types::Id>,
    #[doc = "Source inputs to the mapping."]
    pub r#source: Vec<StructureMapGroupRuleSource>,
    #[doc = "Content to create because of this mapping rule."]
    pub r#target: Vec<StructureMapGroupRuleTarget>,
    #[doc = "Rules contained in this rule."]
    pub r#rule: Vec<StructureMapGroupRule>,
    #[doc = "Which other rules to apply in the context of this rule."]
    pub r#dependent: Vec<StructureMapGroupRuleDependent>,
    #[doc = "Documentation for this instance of data."]
    pub r#documentation: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapGroupRule {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#source: Default::default(),
            r#target: Default::default(),
            r#rule: Default::default(),
            r#dependent: Default::default(),
            r#documentation: Default::default(),
        }
    }
}
#[doc = "Organizes the mapping into managable chunks for human review/ease of maintenance."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique name for the group for the convenience of human readers."]
    pub r#name: super::super::types::Id,
    #[doc = "Another group that this group adds rules to."]
    pub r#extends: Option<super::super::types::Id>,
    #[doc = "If this is the default rule set to apply for the source type or this combination of types."]
    pub r#type_mode: Option<super::super::types::Code>,
    #[doc = "Additional supporting documentation that explains the purpose of the group and the types of mappings within it."]
    pub r#documentation: Option<super::super::types::String>,
    #[doc = "A name assigned to an instance of data. The instance must be provided when the mapping is invoked."]
    pub r#input: Vec<StructureMapGroupInput>,
    #[doc = "Transform Rule from source to target."]
    pub r#rule: Vec<StructureMapGroupRule>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMapGroup {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#extends: Default::default(),
            r#type_mode: Default::default(),
            r#documentation: Default::default(),
            r#input: Default::default(),
            r#rule: Default::default(),
        }
    }
}
#[doc = "A Map of relationships between 2 structures that can be used to transform data."]
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMap {
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
    #[doc = "An absolute URI that is used to identify this structure map when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this structure map is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the structure map is stored on different servers."]
    pub r#url: super::super::types::Uri,
    #[doc = "A formal identifier that is used to identify this structure map when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the structure map when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the structure map author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<StructureMapVersionAlgorithm>,
    #[doc = "A natural language name identifying the structure map. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "A short, descriptive, user-friendly title for the structure map."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this structure map. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this structure map is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the structure map was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the structure map changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the structure map."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the structure map from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate structure map instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the structure map is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this structure map is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the structure map and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the structure map."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "A structure definition used by this map. The structure definition may describe instances that are converted, or the instances that are produced."]
    pub r#structure: Vec<StructureMapStructure>,
    #[doc = "Other maps used by this map (canonical URLs)."]
    pub r#import: Vec<super::super::types::Canonical>,
    #[doc = "Definition of a constant value used in the map rules."]
    pub r#const: Vec<StructureMapConst>,
    #[doc = "Organizes the mapping into managable chunks for human review/ease of maintenance."]
    pub r#group: Vec<StructureMapGroup>,
}
#[allow(clippy::derivable_impls)]
impl Default for StructureMap {
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
            r#url: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#identifier: Default::default(),
            r#version: Default::default(),
            r#version_algorithm: Default::default(),
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
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
            r#structure: Default::default(),
            r#import: Default::default(),
            r#const: Default::default(),
            r#group: Default::default(),
        }
    }
}
