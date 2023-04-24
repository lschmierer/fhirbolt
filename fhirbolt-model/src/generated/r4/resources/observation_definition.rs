// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Characteristics for quantitative results of this observation."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinitionQuantitativeDetails {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Customary unit used to report quantitative results of observations conforming to this ObservationDefinition."]
    pub r#customary_unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "SI unit used to report quantitative results of observations conforming to this ObservationDefinition."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Factor for converting value expressed with SI unit to value expressed with customary unit."]
    pub r#conversion_factor: Option<super::super::types::Decimal>,
    #[doc = "Number of digits after decimal separator when the results of such observations are of type Quantity."]
    pub r#decimal_precision: Option<super::super::types::Integer>,
}
impl Default for ObservationDefinitionQuantitativeDetails {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#customary_unit: Default::default(),
            r#unit: Default::default(),
            r#conversion_factor: Default::default(),
            r#decimal_precision: Default::default(),
        }
    }
}
#[doc = "Multiple  ranges of results qualified by different contexts for ordinal or continuous observations conforming to this ObservationDefinition."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinitionQualifiedInterval {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The category of interval of values for continuous or ordinal observations conforming to this ObservationDefinition."]
    pub r#category: Option<super::super::types::Code>,
    #[doc = "The low and high values determining the interval. There may be only one of the two."]
    pub r#range: Option<Box<super::super::types::Range>>,
    #[doc = "Codes to indicate the health context the range applies to. For example, the normal or therapeutic range."]
    pub r#context: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Codes to indicate the target population this reference range applies to."]
    pub r#applies_to: Vec<super::super::types::CodeableConcept>,
    #[doc = "Sex of the population the range applies to."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The age at which this reference range is applicable. This is a neonatal age (e.g. number of weeks at term) if the meaning says so."]
    pub r#age: Option<Box<super::super::types::Range>>,
    #[doc = "The gestational age to which this reference range is applicable, in the context of pregnancy."]
    pub r#gestational_age: Option<Box<super::super::types::Range>>,
    #[doc = "Text based condition for which the reference range is valid."]
    pub r#condition: Option<super::super::types::String>,
}
impl Default for ObservationDefinitionQualifiedInterval {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Default::default(),
            r#range: Default::default(),
            r#context: Default::default(),
            r#applies_to: Default::default(),
            r#gender: Default::default(),
            r#age: Default::default(),
            r#gestational_age: Default::default(),
            r#condition: Default::default(),
        }
    }
}
#[doc = "Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.\n\nIn a catalog of health-related services that use or produce observations and measurements, this resource describes the expected characteristics of these observation / measurements."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinition {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code that classifies the general type of observation."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Describes what will be observed. Sometimes this is called the observation \"name\"."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A unique identifier assigned to this ObservationDefinition artifact."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The data types allowed for the value element of the instance observations conforming to this ObservationDefinition."]
    pub r#permitted_data_type: Vec<super::super::types::Code>,
    #[doc = "Multiple results allowed for observations conforming to this ObservationDefinition."]
    pub r#multiple_results_allowed: Option<super::super::types::Boolean>,
    #[doc = "The method or technique used to perform the observation."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The preferred name to be used when reporting the results of observations conforming to this ObservationDefinition."]
    pub r#preferred_report_name: Option<super::super::types::String>,
    #[doc = "Characteristics for quantitative results of this observation."]
    pub r#quantitative_details: Option<ObservationDefinitionQuantitativeDetails>,
    #[doc = "Multiple  ranges of results qualified by different contexts for ordinal or continuous observations conforming to this ObservationDefinition."]
    pub r#qualified_interval: Vec<ObservationDefinitionQualifiedInterval>,
    #[doc = "The set of valid coded results for the observations  conforming to this ObservationDefinition."]
    pub r#valid_coded_value_set: Option<Box<super::super::types::Reference>>,
    #[doc = "The set of normal coded results for the observations conforming to this ObservationDefinition."]
    pub r#normal_coded_value_set: Option<Box<super::super::types::Reference>>,
    #[doc = "The set of abnormal coded results for the observation conforming to this ObservationDefinition."]
    pub r#abnormal_coded_value_set: Option<Box<super::super::types::Reference>>,
    #[doc = "The set of critical coded results for the observation conforming to this ObservationDefinition."]
    pub r#critical_coded_value_set: Option<Box<super::super::types::Reference>>,
}
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
            r#category: Default::default(),
            r#code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#identifier: Default::default(),
            r#permitted_data_type: Default::default(),
            r#multiple_results_allowed: Default::default(),
            r#method: Default::default(),
            r#preferred_report_name: Default::default(),
            r#quantitative_details: Default::default(),
            r#qualified_interval: Default::default(),
            r#valid_coded_value_set: Default::default(),
            r#normal_coded_value_set: Default::default(),
            r#abnormal_coded_value_set: Default::default(),
            r#critical_coded_value_set: Default::default(),
        }
    }
}
