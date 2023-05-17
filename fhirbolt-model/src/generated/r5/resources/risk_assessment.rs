// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The date (and possibly time) the risk assessment was performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RiskAssessmentOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Indicates how likely the outcome is (in the specified timeframe)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RiskAssessmentPredictionProbability {
    Decimal(Box<super::super::types::Decimal>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "Indicates the period of time or age range of the subject to which the specified probability applies."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RiskAssessmentPredictionWhen {
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "Describes the expected outcome for the subject."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskAssessmentPrediction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "One of the potential outcomes for the patient (e.g. remission, death,  a particular condition)."]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates how likely the outcome is (in the specified timeframe)."]
    pub r#probability: Option<RiskAssessmentPredictionProbability>,
    #[doc = "Indicates how likely the outcome is (in the specified timeframe), expressed as a qualitative value (e.g. low, medium, or high)."]
    pub r#qualitative_risk: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the risk for this particular subject (with their specific characteristics) divided by the risk of the population in general.  (Numbers greater than 1 = higher risk than the population, numbers less than 1 = lower risk.)."]
    pub r#relative_risk: Option<super::super::types::Decimal>,
    #[doc = "Indicates the period of time or age range of the subject to which the specified probability applies."]
    pub r#when: Option<RiskAssessmentPredictionWhen>,
    #[doc = "Additional information explaining the basis for the prediction."]
    pub r#rationale: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskAssessmentPrediction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#outcome: Default::default(),
            r#probability: Default::default(),
            r#qualitative_risk: Default::default(),
            r#relative_risk: Default::default(),
            r#when: Default::default(),
            r#rationale: Default::default(),
        }
    }
}
#[doc = "An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskAssessment {
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
    #[doc = "Business identifier assigned to the risk assessment."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A reference to the request that is fulfilled by this risk assessment."]
    pub r#based_on: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to a resource that this risk assessment is part of, such as a Procedure."]
    pub r#parent: Option<Box<super::super::types::Reference>>,
    #[doc = "The status of the RiskAssessment, using the same statuses as an Observation."]
    pub r#status: super::super::types::Code,
    #[doc = "The algorithm, process or mechanism used to evaluate the risk."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of the risk assessment performed."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient or group the risk assessment applies to."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The encounter where the assessment was performed."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and possibly time) the risk assessment was performed."]
    pub r#occurrence: Option<RiskAssessmentOccurrence>,
    #[doc = "For assessments or prognosis specific to a particular condition, indicates the condition being assessed."]
    pub r#condition: Option<Box<super::super::types::Reference>>,
    #[doc = "The provider, patient, related person, or software application that performed the assessment."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason the risk assessment was performed."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "Indicates the source data considered as part of the assessment (for example, FamilyHistory, Observations, Procedures, Conditions, etc.)."]
    pub r#basis: Vec<super::super::types::Reference>,
    #[doc = "Describes the expected outcome for the subject."]
    pub r#prediction: Vec<RiskAssessmentPrediction>,
    #[doc = "A description of the steps that might be taken to reduce the identified risk(s)."]
    pub r#mitigation: Option<super::super::types::String>,
    #[doc = "Additional comments about the risk assessment."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskAssessment {
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
            r#based_on: Default::default(),
            r#parent: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#method: Default::default(),
            r#code: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#condition: Default::default(),
            r#performer: Default::default(),
            r#reason: Default::default(),
            r#basis: Default::default(),
            r#prediction: Default::default(),
            r#mitigation: Default::default(),
            r#note: Default::default(),
        }
    }
}
