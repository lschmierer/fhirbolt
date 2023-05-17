// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "Nominal position of the recommended dose in a series (e.g. dose 2 is the next recommended dose)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImmunizationRecommendationRecommendationDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "The recommended number of doses to achieve immunity."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImmunizationRecommendationRecommendationSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Vaccine date recommendations.  For example, earliest date to administer, latest date to administer, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Date classification of recommendation.  For example, earliest date to give, latest date to give, etc."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The date whose meaning is specified by dateCriterion.code."]
    pub r#value: super::super::types::DateTime,
}
#[allow(clippy::derivable_impls)]
impl Default for ImmunizationRecommendationRecommendationDateCriterion {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Vaccine administration recommendations."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendationRecommendation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Vaccine(s) or vaccine group that pertain to the recommendation."]
    pub r#vaccine_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "The targeted disease for the recommendation."]
    pub r#target_disease: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Vaccine(s) which should not be used to fulfill the recommendation."]
    pub r#contraindicated_vaccine_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates the patient status with respect to the path to immunity for the target disease."]
    pub r#forecast_status: Box<super::super::types::CodeableConcept>,
    #[doc = "The reason for the assigned forecast status."]
    pub r#forecast_reason: Vec<super::super::types::CodeableConcept>,
    #[doc = "Vaccine date recommendations.  For example, earliest date to administer, latest date to administer, etc."]
    pub r#date_criterion: Vec<ImmunizationRecommendationRecommendationDateCriterion>,
    #[doc = "Contains the description about the protocol under which the vaccine was administered."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "One possible path to achieve presumed immunity against a disease - within the context of an authority."]
    pub r#series: Option<super::super::types::String>,
    #[doc = "Nominal position of the recommended dose in a series (e.g. dose 2 is the next recommended dose)."]
    pub r#dose_number: Option<ImmunizationRecommendationRecommendationDoseNumber>,
    #[doc = "The recommended number of doses to achieve immunity."]
    pub r#series_doses: Option<ImmunizationRecommendationRecommendationSeriesDoses>,
    #[doc = "Immunization event history and/or evaluation that supports the status and recommendation."]
    pub r#supporting_immunization: Vec<super::super::types::Reference>,
    #[doc = "Patient Information that supports the status and recommendation.  This includes patient observations, adverse reactions and allergy/intolerance information."]
    pub r#supporting_patient_information: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImmunizationRecommendationRecommendation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#vaccine_code: Default::default(),
            r#target_disease: Default::default(),
            r#contraindicated_vaccine_code: Default::default(),
            r#forecast_status: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#forecast_reason: Default::default(),
            r#date_criterion: Default::default(),
            r#description: Default::default(),
            r#series: Default::default(),
            r#dose_number: Default::default(),
            r#series_doses: Default::default(),
            r#supporting_immunization: Default::default(),
            r#supporting_patient_information: Default::default(),
        }
    }
}
#[doc = "A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendation {
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
    #[doc = "A unique identifier assigned to this particular recommendation record."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The patient the recommendation(s) are for."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date the immunization recommendation(s) were created."]
    pub r#date: super::super::types::DateTime,
    #[doc = "Indicates the authority who published the protocol (e.g. ACIP)."]
    pub r#authority: Option<Box<super::super::types::Reference>>,
    #[doc = "Vaccine administration recommendations."]
    pub r#recommendation: Vec<ImmunizationRecommendationRecommendation>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImmunizationRecommendation {
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
            r#identifier: Default::default(),
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#date: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#authority: Default::default(),
            r#recommendation: Default::default(),
        }
    }
}
