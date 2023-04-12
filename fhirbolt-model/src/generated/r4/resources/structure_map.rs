// Generated on 2023-04-13 by fhirbolt-codegen v0.1.0
#[doc = "A value to use if there is no existing value in the source object."]
#[derive(Debug, Clone, PartialEq)]
pub enum StructureMapGroupRuleSourceDefaultValue {
    Base64Binary(Box<super::super::types::Base64Binary>),
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
    Code(Box<super::super::types::Code>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Decimal(Box<super::super::types::Decimal>),
    Id(Box<super::super::types::Id>),
    Instant(Box<super::super::types::Instant>),
    Integer(Box<super::super::types::Integer>),
    Markdown(Box<super::super::types::Markdown>),
    Oid(Box<super::super::types::Oid>),
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Time(Box<super::super::types::Time>),
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Uri(Box<super::super::types::Uri>),
    Url(Box<super::super::types::Url>),
    Uuid(Box<super::super::types::Uuid>),
    Address(Box<super::super::types::Address>),
    Age(Box<super::super::types::Age>),
    Annotation(Box<super::super::types::Annotation>),
    Attachment(Box<super::super::types::Attachment>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Coding(Box<super::super::types::Coding>),
    ContactPoint(Box<super::super::types::ContactPoint>),
    Count(Box<super::super::types::Count>),
    Distance(Box<super::super::types::Distance>),
    Duration(Box<super::super::types::Duration>),
    HumanName(Box<super::super::types::HumanName>),
    Identifier(Box<super::super::types::Identifier>),
    Money(Box<super::super::types::Money>),
    Period(Box<super::super::types::Period>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    Reference(Box<super::super::types::Reference>),
    SampledData(Box<super::super::types::SampledData>),
    Signature(Box<super::super::types::Signature>),
    Timing(Box<super::super::types::Timing>),
    ContactDetail(Box<super::super::types::ContactDetail>),
    Contributor(Box<super::super::types::Contributor>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    Expression(Box<super::super::types::Expression>),
    ParameterDefinition(Box<super::super::types::ParameterDefinition>),
    RelatedArtifact(Box<super::super::types::RelatedArtifact>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
    UsageContext(Box<super::super::types::UsageContext>),
    Dosage(Box<super::super::types::Dosage>),
    Meta(Box<super::super::types::Meta>),
    Invalid,
}
impl Default for StructureMapGroupRuleSourceDefaultValue {
    fn default() -> StructureMapGroupRuleSourceDefaultValue {
        StructureMapGroupRuleSourceDefaultValue::Invalid
    }
}
#[doc = "Parameter value - variable or literal."]
#[derive(Debug, Clone, PartialEq)]
pub enum StructureMapGroupRuleTargetParameterValue {
    Id(Box<super::super::types::Id>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Decimal(Box<super::super::types::Decimal>),
    Invalid,
}
impl Default for StructureMapGroupRuleTargetParameterValue {
    fn default() -> StructureMapGroupRuleTargetParameterValue {
        StructureMapGroupRuleTargetParameterValue::Invalid
    }
}
#[doc = "A structure definition used by this map. The structure definition may describe instances that are converted, or the instances that are produced."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapStructure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The canonical reference to the structure."]
    pub r#url: super::super::types::Canonical,
    #[doc = "How the referenced structure is used in this mapping."]
    pub r#mode: super::super::types::Code,
    #[doc = "The name used for this type in the map."]
    pub r#alias: Option<super::super::types::String>,
    #[doc = "Documentation that describes how the structure is used in the mapping."]
    pub r#documentation: Option<super::super::types::String>,
}
#[doc = "A name assigned to an instance of data. The instance must be provided when the mapping is invoked."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapGroupInput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name for this instance of data."]
    pub r#name: super::super::types::Id,
    #[doc = "Type for this instance of data."]
    pub r#type: Option<super::super::types::String>,
    #[doc = "Mode for this instance of data."]
    pub r#mode: super::super::types::Code,
    #[doc = "Documentation for this instance of data."]
    pub r#documentation: Option<super::super::types::String>,
}
#[doc = "Source inputs to the mapping."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type or variable this rule applies to."]
    pub r#context: super::super::types::Id,
    #[doc = "Specified minimum cardinality for the element. This is optional; if present, it acts an implicit check on the input content."]
    pub r#min: Option<super::super::types::Integer>,
    #[doc = "Specified maximum cardinality for the element - a number or a \"*\". This is optional; if present, it acts an implicit check on the input content (* just serves as documentation; it's the default value)."]
    pub r#max: Option<super::super::types::String>,
    #[doc = "Specified type for the element. This works as a condition on the mapping - use for polymorphic elements."]
    pub r#type: Option<super::super::types::String>,
    #[doc = "A value to use if there is no existing value in the source object."]
    pub r#default_value: Option<StructureMapGroupRuleSourceDefaultValue>,
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
#[doc = "Parameters to the transform."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleTargetParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Parameter value - variable or literal."]
    pub r#value: StructureMapGroupRuleTargetParameterValue,
}
#[doc = "Content to create because of this mapping rule."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type or variable this rule applies to."]
    pub r#context: Option<super::super::types::Id>,
    #[doc = "How to interpret the context."]
    pub r#context_type: Option<super::super::types::Code>,
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
#[doc = "Which other rules to apply in the context of this rule."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapGroupRuleDependent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name of a rule or group to apply."]
    pub r#name: super::super::types::Id,
    #[doc = "Variable to pass to the rule or group."]
    pub r#variable: Vec<super::super::types::String>,
}
#[doc = "Transform Rule from source to target."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapGroupRule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name of the rule for internal references."]
    pub r#name: super::super::types::Id,
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
#[doc = "Organizes the mapping into manageable chunks for human review/ease of maintenance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMapGroup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A unique name for the group for the convenience of human readers."]
    pub r#name: super::super::types::Id,
    #[doc = "Another group that this group adds rules to."]
    pub r#extends: Option<super::super::types::Id>,
    #[doc = "If this is the default rule set to apply for the source type or this combination of types."]
    pub r#type_mode: super::super::types::Code,
    #[doc = "Additional supporting documentation that explains the purpose of the group and the types of mappings within it."]
    pub r#documentation: Option<super::super::types::String>,
    #[doc = "A name assigned to an instance of data. The instance must be provided when the mapping is invoked."]
    pub r#input: Vec<StructureMapGroupInput>,
    #[doc = "Transform Rule from source to target."]
    pub r#rule: Vec<StructureMapGroupRule>,
}
#[doc = "A Map of relationships between 2 structures that can be used to transform data."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StructureMap {
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
    #[doc = "An absolute URI that is used to identify this structure map when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this structure map is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the structure map is stored on different servers."]
    pub r#url: super::super::types::Uri,
    #[doc = "A formal identifier that is used to identify this structure map when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the structure map when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the structure map author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the structure map. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "A short, descriptive, user-friendly title for the structure map."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this structure map. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this structure map is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the structure map was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the structure map changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the structure map."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the structure map from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate structure map instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the structure map is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this structure map is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the structure map and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the structure map."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A structure definition used by this map. The structure definition may describe instances that are converted, or the instances that are produced."]
    pub r#structure: Vec<StructureMapStructure>,
    #[doc = "Other maps used by this map (canonical URLs)."]
    pub r#import: Vec<super::super::types::Canonical>,
    #[doc = "Organizes the mapping into manageable chunks for human review/ease of maintenance."]
    pub r#group: Vec<StructureMapGroup>,
}
