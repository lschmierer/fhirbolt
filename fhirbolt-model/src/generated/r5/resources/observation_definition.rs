// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ObservationDefinitionVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "A set of qualified values associated with a context and a set of conditions -  provides a range for quantitative and ordinal observations and a collection of value sets for qualitative observations."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinitionQualifiedValue {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A concept defining the context for this set of qualified values."]
    pub r#context: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target population this  set of qualified values applies to."]
    pub r#applies_to: Vec<super::super::types::CodeableConcept>,
    #[doc = "The gender this  set of qualified values applies to."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The age range this  set of qualified values applies to."]
    pub r#age: Option<Box<super::super::types::Range>>,
    #[doc = "The gestational age this  set of qualified values applies to."]
    pub r#gestational_age: Option<Box<super::super::types::Range>>,
    #[doc = "Text based condition for which the the set of qualified values is valid."]
    pub r#condition: Option<super::super::types::String>,
    #[doc = "The category of range of values for continuous or ordinal observations that match the criteria of this set of qualified values."]
    pub r#range_category: Option<super::super::types::Code>,
    #[doc = "The range of values defined for continuous or ordinal observations that match the criteria of this set of qualified values."]
    pub r#range: Option<Box<super::super::types::Range>>,
    #[doc = "The set of valid coded results for qualitative observations  that match the criteria of this set of qualified values."]
    pub r#valid_coded_value_set: Option<super::super::types::Canonical>,
    #[doc = "The set of normal coded results for qualitative observations  that match the criteria of this set of qualified values."]
    pub r#normal_coded_value_set: Option<super::super::types::Canonical>,
    #[doc = "The set of abnormal coded results for qualitative observations  that match the criteria of this set of qualified values."]
    pub r#abnormal_coded_value_set: Option<super::super::types::Canonical>,
    #[doc = "The set of critical coded results for qualitative observations  that match the criteria of this set of qualified values."]
    pub r#critical_coded_value_set: Option<super::super::types::Canonical>,
}
#[allow(clippy::derivable_impls)]
impl Default for ObservationDefinitionQualifiedValue {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#context: Default::default(),
            r#applies_to: Default::default(),
            r#gender: Default::default(),
            r#age: Default::default(),
            r#gestational_age: Default::default(),
            r#condition: Default::default(),
            r#range_category: Default::default(),
            r#range: Default::default(),
            r#valid_coded_value_set: Default::default(),
            r#normal_coded_value_set: Default::default(),
            r#abnormal_coded_value_set: Default::default(),
            r#critical_coded_value_set: Default::default(),
        }
    }
}
#[doc = "Some observations have multiple component observations, expressed as separate code value pairs."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinitionComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes what will be observed."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The data types allowed for the value element of the instance of this component observations."]
    pub r#permitted_data_type: Vec<super::super::types::Code>,
    #[doc = "Units allowed for the valueQuantity element in the instance observations conforming to this ObservationDefinition."]
    pub r#permitted_unit: Vec<super::super::types::Coding>,
    #[doc = "A set of qualified values associated with a context and a set of conditions -  provides a range for quantitative and ordinal observations and a collection of value sets for qualitative observations."]
    pub r#qualified_value: Vec<ObservationDefinitionQualifiedValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for ObservationDefinitionComponent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#permitted_data_type: Default::default(),
            r#permitted_unit: Default::default(),
            r#qualified_value: Default::default(),
        }
    }
}
#[doc = "Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.\n\nIn a catalog of health-related services that use or produce observations and measurements, this resource describes the expected characteristics of these observation / measurements."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinition {
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
    #[doc = "An absolute URL that is used to identify this ObservationDefinition when it is referenced in a specification, model, design or an instance. This SHALL be a URL, SHOULD be globally unique, and SHOULD be an address at which this ObservationDefinition is (or will be) published. The URL SHOULD include the major version of the ObservationDefinition. For more information see Technical and Business Versions."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "Business identifiers assigned to this ObservationDefinition. by the performer and/or other systems. These identifiers remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the ObservationDefinition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the ObservationDefinition author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions are orderable."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<ObservationDefinitionVersionAlgorithm>,
    #[doc = "A natural language name identifying the ObservationDefinition. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the ObservationDefinition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The current state of the ObservationDefinition."]
    pub r#status: super::super::types::Code,
    #[doc = "A flag to indicate that this ObservationDefinition is authored for testing purposes (or education/evaluation/marketing), and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date (and optionally time) when the ObservationDefinition was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the ObservationDefinition changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Helps establish the \"authority/credibility\" of the ObservationDefinition. May also allow for contact."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the ObservationDefinition from the consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate ObservationDefinition instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A jurisdiction in which the ObservationDefinition is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explains why this ObservationDefinition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "Copyright statement relating to the ObservationDefinition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the ObservationDefinition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The date on which the asset content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the asset content was last reviewed. Review happens periodically after that, but doesn't change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the ObservationDefinition content was or is planned to be effective."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "The canonical URL pointing to another FHIR-defined ObservationDefinition that is adhered to in whole or in part by this definition."]
    pub r#derived_from_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally-defined observation definition, guideline or other definition that is adhered to in whole or in part by this definition."]
    pub r#derived_from_uri: Vec<super::super::types::Uri>,
    #[doc = "A code that describes the intended kind of subject of Observation instances conforming to this ObservationDefinition."]
    pub r#subject: Vec<super::super::types::CodeableConcept>,
    #[doc = "The type of individual/organization/device that is expected to act upon instances of this definition."]
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code that classifies the general type of observation."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Describes what will be observed. Sometimes this is called the observation \"name\"."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The data types allowed for the value element of the instance observations conforming to this ObservationDefinition."]
    pub r#permitted_data_type: Vec<super::super::types::Code>,
    #[doc = "Multiple results allowed for observations conforming to this ObservationDefinition."]
    pub r#multiple_results_allowed: Option<super::super::types::Boolean>,
    #[doc = "The site on the subject's body where the  observation is to be made."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The method or technique used to perform the observation."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The kind of specimen that this type of observation is produced on."]
    pub r#specimen: Vec<super::super::types::Reference>,
    #[doc = "The measurement model of device or actual device used to produce observations of this type."]
    pub r#device: Vec<super::super::types::Reference>,
    #[doc = "The preferred name to be used when reporting the results of observations conforming to this ObservationDefinition."]
    pub r#preferred_report_name: Option<super::super::types::String>,
    #[doc = "Units allowed for the valueQuantity element in the instance observations conforming to this ObservationDefinition."]
    pub r#permitted_unit: Vec<super::super::types::Coding>,
    #[doc = "A set of qualified values associated with a context and a set of conditions -  provides a range for quantitative and ordinal observations and a collection of value sets for qualitative observations."]
    pub r#qualified_value: Vec<ObservationDefinitionQualifiedValue>,
    #[doc = "This ObservationDefinition defines a group  observation (e.g. a battery, a panel of tests, a set of vital sign measurements) that includes the target as a member of the group."]
    pub r#has_member: Vec<super::super::types::Reference>,
    #[doc = "Some observations have multiple component observations, expressed as separate code value pairs."]
    pub r#component: Vec<ObservationDefinitionComponent>,
}
#[allow(clippy::derivable_impls)]
impl Default for ObservationDefinition {
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
            r#derived_from_canonical: Default::default(),
            r#derived_from_uri: Default::default(),
            r#subject: Default::default(),
            r#performer_type: Default::default(),
            r#category: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#permitted_data_type: Default::default(),
            r#multiple_results_allowed: Default::default(),
            r#body_site: Default::default(),
            r#method: Default::default(),
            r#specimen: Default::default(),
            r#device: Default::default(),
            r#preferred_report_name: Default::default(),
            r#permitted_unit: Default::default(),
            r#qualified_value: Default::default(),
            r#has_member: Default::default(),
            r#component: Default::default(),
        }
    }
}
