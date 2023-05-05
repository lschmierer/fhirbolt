// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MeasureVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "The intended subjects for the measure. If this element is not provided, a Patient subject is assumed, but the subject of the measure can be anything."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MeasureSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The intended subjects for the measure. If this element is not provided, a Patient subject is assumed, but the subject of the measure can be anything."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MeasureGroupSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Provides a description of an individual term used within the measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureTerm {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A codeable representation of the defined term."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Provides a definition for the term as used within the measure."]
    pub r#definition: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureTerm {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#definition: Default::default(),
        }
    }
}
#[doc = "A population criteria for the measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureGroupPopulation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier that is unique within the Measure allowing linkage to the equivalent population in a MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "The type of population criteria."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The human readable description of this population criteria."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "An expression that specifies the criteria for the population, typically the name of an expression in a library."]
    pub r#criteria: Option<Box<super::super::types::Expression>>,
    #[doc = "A Group resource that defines this population as a set of characteristics."]
    pub r#group_definition: Option<Box<super::super::types::Reference>>,
    #[doc = "The id of a population element in this measure that provides the input for this population criteria. In most cases, the scoring structure of the measure implies specific relationships (e.g. the Numerator uses the Denominator as the source in a proportion scoring). In some cases, however, multiple possible choices exist and must be resolved explicitly. For example in a ratio measure with multiple initial populations, the denominator must specify which population should be used as the starting point."]
    pub r#input_population_id: Option<super::super::types::String>,
    #[doc = "Specifies which method should be used to aggregate measure observation values. For most scoring types, this is implied by scoring (e.g. a proportion measure counts members of the populations). For continuous variables, however, this information must be specified to ensure correct calculation."]
    pub r#aggregate_method: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureGroupPopulation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#description: Default::default(),
            r#criteria: Default::default(),
            r#group_definition: Default::default(),
            r#input_population_id: Default::default(),
            r#aggregate_method: Default::default(),
        }
    }
}
#[doc = "A component of the stratifier criteria for the measure report, specified as either the name of a valid CQL expression defined within a referenced library or a valid FHIR Resource Path."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureGroupStratifierComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier that is unique within the Measure allowing linkage to the equivalent item in a MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "Indicates a meaning for the stratifier component. This can be as simple as a unique identifier, or it can establish meaning in a broader context by drawing from a terminology, allowing stratifiers to be correlated across measures."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The human readable description of this stratifier criteria component."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "An expression that specifies the criteria for this component of the stratifier. This is typically the name of an expression defined within a referenced library, but it may also be a path to a stratifier element."]
    pub r#criteria: Option<Box<super::super::types::Expression>>,
    #[doc = "A Group resource that defines this population as a set of characteristics."]
    pub r#group_definition: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureGroupStratifierComponent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#description: Default::default(),
            r#criteria: Default::default(),
            r#group_definition: Default::default(),
        }
    }
}
#[doc = "The stratifier criteria for the measure report, specified as either the name of a valid CQL expression defined within a referenced library or a valid FHIR Resource Path."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureGroupStratifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier that is unique within the Measure allowing linkage to the equivalent item in a MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "Indicates a meaning for the stratifier. This can be as simple as a unique identifier, or it can establish meaning in a broader context by drawing from a terminology, allowing stratifiers to be correlated across measures."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The human readable description of this stratifier criteria."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "An expression that specifies the criteria for the stratifier. This is typically the name of an expression defined within a referenced library, but it may also be a path to a stratifier element."]
    pub r#criteria: Option<Box<super::super::types::Expression>>,
    #[doc = "A Group resource that defines this population as a set of characteristics."]
    pub r#group_definition: Option<Box<super::super::types::Reference>>,
    #[doc = "A component of the stratifier criteria for the measure report, specified as either the name of a valid CQL expression defined within a referenced library or a valid FHIR Resource Path."]
    pub r#component: Vec<MeasureGroupStratifierComponent>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureGroupStratifier {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#description: Default::default(),
            r#criteria: Default::default(),
            r#group_definition: Default::default(),
            r#component: Default::default(),
        }
    }
}
#[doc = "A group of population criteria for the measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureGroup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier that is unique within the Measure allowing linkage to the equivalent item in a MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "Indicates a meaning for the group. This can be as simple as a unique identifier, or it can establish meaning in a broader context by drawing from a terminology, allowing groups to be correlated across measures."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The human readable description of this population group."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Indicates whether the measure is used to examine a process, an outcome over time, a patient-reported outcome, or a structure measure such as utilization."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The intended subjects for the measure. If this element is not provided, a Patient subject is assumed, but the subject of the measure can be anything."]
    pub r#subject: Option<MeasureGroupSubject>,
    #[doc = "The population basis specifies the type of elements in the population. For a subject-based measure, this is boolean (because the subject and the population basis are the same, and the population criteria define yes/no values for each individual in the population). For measures that have a population basis that is different than the subject, this element specifies the type of the population basis. For example, an encounter-based measure has a subject of Patient and a population basis of Encounter, and the population criteria all return lists of Encounters."]
    pub r#basis: Option<super::super::types::Code>,
    #[doc = "Indicates how the calculation is performed for the measure, including proportion, ratio, continuous-variable, and cohort. The value set is extensible, allowing additional measure scoring types to be represented."]
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Defines the expected units of measure for the measure score. This element SHOULD be specified as a UCUM unit."]
    pub r#scoring_unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes how to combine the information calculated, based on logic in each of several populations, into one summarized result."]
    pub r#rate_aggregation: Option<super::super::types::Markdown>,
    #[doc = "Information on whether an increase or decrease in score is the preferred result (e.g., a higher score indicates better quality OR a lower score indicates better quality OR quality is within a range)."]
    pub r#improvement_notation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to a Library resource containing the formal logic used by the measure group."]
    pub r#library: Vec<super::super::types::Canonical>,
    #[doc = "A population criteria for the measure."]
    pub r#population: Vec<MeasureGroupPopulation>,
    #[doc = "The stratifier criteria for the measure report, specified as either the name of a valid CQL expression defined within a referenced library or a valid FHIR Resource Path."]
    pub r#stratifier: Vec<MeasureGroupStratifier>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureGroup {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#description: Default::default(),
            r#type: Default::default(),
            r#subject: Default::default(),
            r#basis: Default::default(),
            r#scoring: Default::default(),
            r#scoring_unit: Default::default(),
            r#rate_aggregation: Default::default(),
            r#improvement_notation: Default::default(),
            r#library: Default::default(),
            r#population: Default::default(),
            r#stratifier: Default::default(),
        }
    }
}
#[doc = "The supplemental data criteria for the measure report, specified as either the name of a valid CQL expression within a referenced library, or a valid FHIR Resource Path."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureSupplementalData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier that is unique within the Measure allowing linkage to the equivalent item in a MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "Indicates a meaning for the supplemental data. This can be as simple as a unique identifier, or it can establish meaning in a broader context by drawing from a terminology, allowing supplemental data to be correlated across measures."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An indicator of the intended usage for the supplemental data element. Supplemental data indicates the data is additional information requested to augment the measure information. Risk adjustment factor indicates the data is additional information used to calculate risk adjustment factors when applying a risk model to the measure calculation."]
    pub r#usage: Vec<super::super::types::CodeableConcept>,
    #[doc = "The human readable description of this supplemental data."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The criteria for the supplemental data. This is typically the name of a valid expression defined within a referenced library, but it may also be a path to a specific data element. The criteria defines the data to be returned for this element."]
    pub r#criteria: Box<super::super::types::Expression>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureSupplementalData {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#usage: Default::default(),
            r#description: Default::default(),
            r#criteria: Box::new(super::super::types::Expression {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "The Measure resource provides the definition of a quality measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct Measure {
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
    #[doc = "An absolute URI that is used to identify this measure when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this measure is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the measure is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this measure when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the measure when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the measure author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<MeasureVersionAlgorithm>,
    #[doc = "A natural language name identifying the measure. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the measure."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the measure giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "The status of this measure. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this measure is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The intended subjects for the measure. If this element is not provided, a Patient subject is assumed, but the subject of the measure can be anything."]
    pub r#subject: Option<MeasureSubject>,
    #[doc = "The population basis specifies the type of elements in the population. For a subject-based measure, this is boolean (because the subject and the population basis are the same, and the population criteria define yes/no values for each individual in the population). For measures that have a population basis that is different than the subject, this element specifies the type of the population basis. For example, an encounter-based measure has a subject of Patient and a population basis of Encounter, and the population criteria all return lists of Encounters."]
    pub r#basis: Option<super::super::types::Code>,
    #[doc = "The date  (and optionally time) when the measure was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the measure changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the measure."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the measure from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate measure instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the measure is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this measure is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A detailed description, from a clinical perspective, of how the measure is used."]
    pub r#usage: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the measure and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the measure."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the measure content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the measure. Topics provide a high-level categorization grouping types of measures that can be useful for filtering and searching."]
    pub r#topic: Vec<super::super::types::CodeableConcept>,
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
    #[doc = "A reference to a Library resource containing the formal logic used by the measure."]
    pub r#library: Vec<super::super::types::Canonical>,
    #[doc = "Notices and disclaimers regarding the use of the measure or related to intellectual property (such as code systems) referenced by the measure."]
    pub r#disclaimer: Option<super::super::types::Markdown>,
    #[doc = "Indicates how the calculation is performed for the measure, including proportion, ratio, continuous-variable, and cohort. The value set is extensible, allowing additional measure scoring types to be represented."]
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Defines the expected units of measure for the measure score. This element SHOULD be specified as a UCUM unit."]
    pub r#scoring_unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this is a composite measure, the scoring method used to combine the component measures to determine the composite score."]
    pub r#composite_scoring: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates whether the measure is used to examine a process, an outcome over time, a patient-reported outcome, or a structure measure such as utilization."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "A description of the risk adjustment factors that may impact the resulting score for the measure and how they may be accounted for when computing and reporting measure results."]
    pub r#risk_adjustment: Option<super::super::types::Markdown>,
    #[doc = "Describes how to combine the information calculated, based on logic in each of several populations, into one summarized result."]
    pub r#rate_aggregation: Option<super::super::types::Markdown>,
    #[doc = "Provides a succinct statement of the need for the measure. Usually includes statements pertaining to importance criterion: impact, gap in care, and evidence."]
    pub r#rationale: Option<super::super::types::Markdown>,
    #[doc = "Provides a summary of relevant clinical guidelines or other clinical recommendations supporting the measure."]
    pub r#clinical_recommendation_statement: Option<super::super::types::Markdown>,
    #[doc = "Information on whether an increase or decrease in score is the preferred result (e.g., a higher score indicates better quality OR a lower score indicates better quality OR quality is within a range)."]
    pub r#improvement_notation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Provides a description of an individual term used within the measure."]
    pub r#term: Vec<MeasureTerm>,
    #[doc = "Additional guidance for the measure including how it can be used in a clinical context, and the intent of the measure."]
    pub r#guidance: Option<super::super::types::Markdown>,
    #[doc = "A group of population criteria for the measure."]
    pub r#group: Vec<MeasureGroup>,
    #[doc = "The supplemental data criteria for the measure report, specified as either the name of a valid CQL expression within a referenced library, or a valid FHIR Resource Path."]
    pub r#supplemental_data: Vec<MeasureSupplementalData>,
}
#[allow(clippy::derivable_impls)]
impl Default for Measure {
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
            r#subtitle: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#subject: Default::default(),
            r#basis: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#purpose: Default::default(),
            r#usage: Default::default(),
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
            r#library: Default::default(),
            r#disclaimer: Default::default(),
            r#scoring: Default::default(),
            r#scoring_unit: Default::default(),
            r#composite_scoring: Default::default(),
            r#type: Default::default(),
            r#risk_adjustment: Default::default(),
            r#rate_aggregation: Default::default(),
            r#rationale: Default::default(),
            r#clinical_recommendation_statement: Default::default(),
            r#improvement_notation: Default::default(),
            r#term: Default::default(),
            r#guidance: Default::default(),
            r#group: Default::default(),
            r#supplemental_data: Default::default(),
        }
    }
}
