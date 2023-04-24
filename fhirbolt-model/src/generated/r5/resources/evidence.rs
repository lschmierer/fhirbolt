// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "Citation Resource or display of suggested citation for this evidence."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceCiteAs {
    Reference(Box<super::super::types::Reference>),
    Markdown(Box<super::super::types::Markdown>),
    #[default]
    Invalid,
}
#[doc = "Evidence variable such as population, exposure, or outcome."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A text description or summary of the variable."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "population | subpopulation | exposure | referenceExposure | measuredVariable | confounder."]
    pub r#variable_role: Box<super::super::types::CodeableConcept>,
    #[doc = "Definition of the actual variable related to the statistic(s)."]
    pub r#observed: Option<Box<super::super::types::Reference>>,
    #[doc = "Definition of the intended variable related to the Evidence."]
    pub r#intended: Option<Box<super::super::types::Reference>>,
    #[doc = "Indication of quality of match between intended variable to actual variable."]
    pub r#directness_match: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for EvidenceVariableDefinition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#variable_role: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#observed: Default::default(),
            r#intended: Default::default(),
            r#directness_match: Default::default(),
        }
    }
}
#[doc = "Number of samples in the statistic."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceStatisticSampleSize {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Human-readable summary of population sample size."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Footnote or explanatory note about the sample size."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Number of participants in the population."]
    pub r#number_of_studies: Option<super::super::types::UnsignedInt>,
    #[doc = "A human-readable string to clarify or explain concepts about the sample size."]
    pub r#number_of_participants: Option<super::super::types::UnsignedInt>,
    #[doc = "Number of participants with known results for measured variables."]
    pub r#known_data_count: Option<super::super::types::UnsignedInt>,
}
impl Default for EvidenceStatisticSampleSize {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#number_of_studies: Default::default(),
            r#number_of_participants: Default::default(),
            r#known_data_count: Default::default(),
        }
    }
}
#[doc = "A statistical attribute of the statistic such as a measure of heterogeneity."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceStatisticAttributeEstimate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Human-readable summary of the estimate."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Footnote or explanatory note about the estimate."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The type of attribute estimate, e.g., confidence interval or p value."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The singular quantity of the attribute estimate, for attribute estimates represented as single values; also used to report unit of measure."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Use 95 for a 95% confidence interval."]
    pub r#level: Option<super::super::types::Decimal>,
    #[doc = "Lower bound of confidence interval."]
    pub r#range: Option<Box<super::super::types::Range>>,
    #[doc = "A nested attribute estimate; which is the attribute estimate of an attribute estimate."]
    pub r#attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
}
impl Default for EvidenceStatisticAttributeEstimate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#type: Default::default(),
            r#quantity: Default::default(),
            r#level: Default::default(),
            r#range: Default::default(),
            r#attribute_estimate: Default::default(),
        }
    }
}
#[doc = "A variable adjusted for in the adjusted analysis."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceStatisticModelCharacteristicVariable {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Description of the variable."]
    pub r#variable_definition: Box<super::super::types::Reference>,
    #[doc = "How the variable is classified for use in adjusted analysis."]
    pub r#handling: Option<super::super::types::Code>,
    #[doc = "Description for grouping of ordinal or polychotomous variables."]
    pub r#value_category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Discrete value for grouping of ordinal or polychotomous variables."]
    pub r#value_quantity: Vec<super::super::types::Quantity>,
    #[doc = "Range of values for grouping of ordinal or polychotomous variables."]
    pub r#value_range: Vec<super::super::types::Range>,
}
impl Default for EvidenceStatisticModelCharacteristicVariable {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#variable_definition: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#handling: Default::default(),
            r#value_category: Default::default(),
            r#value_quantity: Default::default(),
            r#value_range: Default::default(),
        }
    }
}
#[doc = "A component of the method to generate the statistic."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceStatisticModelCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Description of a component of the method to generate the statistic."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Further specification of the quantified value of the component of the method to generate the statistic."]
    pub r#value: Option<Box<super::super::types::Quantity>>,
    #[doc = "A variable adjusted for in the adjusted analysis."]
    pub r#variable: Vec<EvidenceStatisticModelCharacteristicVariable>,
    #[doc = "An attribute of the statistic used as a model characteristic."]
    pub r#attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
}
impl Default for EvidenceStatisticModelCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#value: Default::default(),
            r#variable: Default::default(),
            r#attribute_estimate: Default::default(),
        }
    }
}
#[doc = "Values and parameters for a single statistic."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceStatistic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A description of the content value of the statistic."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Type of statistic, e.g., relative risk."]
    pub r#statistic_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the measured variable is handled categorically, the category element is used to define which category the statistic is reporting."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Statistic value."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The number of events associated with the statistic, where the unit of analysis is different from numberAffected, sampleSize.knownDataCount and sampleSize.numberOfParticipants."]
    pub r#number_of_events: Option<super::super::types::UnsignedInt>,
    #[doc = "The number of participants affected where the unit of analysis is the same as sampleSize.knownDataCount and sampleSize.numberOfParticipants."]
    pub r#number_affected: Option<super::super::types::UnsignedInt>,
    #[doc = "Number of samples in the statistic."]
    pub r#sample_size: Option<EvidenceStatisticSampleSize>,
    #[doc = "A statistical attribute of the statistic such as a measure of heterogeneity."]
    pub r#attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
    #[doc = "A component of the method to generate the statistic."]
    pub r#model_characteristic: Vec<EvidenceStatisticModelCharacteristic>,
}
impl Default for EvidenceStatistic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#statistic_type: Default::default(),
            r#category: Default::default(),
            r#quantity: Default::default(),
            r#number_of_events: Default::default(),
            r#number_affected: Default::default(),
            r#sample_size: Default::default(),
            r#attribute_estimate: Default::default(),
            r#model_characteristic: Default::default(),
        }
    }
}
#[doc = "Assessment of certainty, confidence in the estimates, or quality of the evidence."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceCertainty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Textual description of certainty."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Aspect of certainty being rated."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Assessment or judgement of the aspect."]
    pub r#rating: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Individual or group who did the rating."]
    pub r#rater: Option<super::super::types::String>,
    #[doc = "A domain or subdomain of certainty."]
    pub r#subcomponent: Vec<EvidenceCertainty>,
}
impl Default for EvidenceCertainty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#type: Default::default(),
            r#rating: Default::default(),
            r#rater: Default::default(),
            r#subcomponent: Default::default(),
        }
    }
}
#[doc = "The Evidence Resource provides a machine-interpretable expression of an evidence concept including the evidence variables (e.g., population, exposures/interventions, comparators, outcomes, measured variables, confounding variables), the statistics, and the certainty of this evidence."]
#[derive(Debug, Clone, PartialEq)]
pub struct Evidence {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this evidence when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this summary is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the summary is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this summary when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the summary when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the summary author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<EvidenceVersionAlgorithm>,
    #[doc = "A natural language name identifying the evidence. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the summary."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Citation Resource or display of suggested citation for this evidence."]
    pub r#cite_as: Option<EvidenceCiteAs>,
    #[doc = "The status of this summary. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this resource is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the summary was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the summary changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the evidence."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate evidence instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "Explanation of why this Evidence is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the Evidence and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the Evidence."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "Link or citation to artifact associated with the summary."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "A free text natural language description of the evidence from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Declarative description of the Evidence."]
    pub r#assertion: Option<super::super::types::Markdown>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Evidence variable such as population, exposure, or outcome."]
    pub r#variable_definition: Vec<EvidenceVariableDefinition>,
    #[doc = "The method to combine studies."]
    pub r#synthesis_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The design of the study that produced this evidence. The design is described with any number of study design characteristics."]
    pub r#study_design: Vec<super::super::types::CodeableConcept>,
    #[doc = "Values and parameters for a single statistic."]
    pub r#statistic: Vec<EvidenceStatistic>,
    #[doc = "Assessment of certainty, confidence in the estimates, or quality of the evidence."]
    pub r#certainty: Vec<EvidenceCertainty>,
}
impl Default for Evidence {
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
            r#cite_as: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#use_context: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#copyright_label: Default::default(),
            r#related_artifact: Default::default(),
            r#description: Default::default(),
            r#assertion: Default::default(),
            r#note: Default::default(),
            r#variable_definition: Default::default(),
            r#synthesis_type: Default::default(),
            r#study_design: Default::default(),
            r#statistic: Default::default(),
            r#certainty: Default::default(),
        }
    }
}
