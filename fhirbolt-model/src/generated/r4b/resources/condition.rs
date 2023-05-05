// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Estimated or actual date or date-time  the condition began, in the opinion of the clinician."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConditionOnset {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "The date or estimated date that the condition resolved or went into remission. This is called \"abatement\" because of the many overloaded connotations associated with \"remission\" or \"resolution\" - Conditions are never really resolved, but they can abate."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConditionAbatement {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Clinical stage or grade of a condition. May include formal severity assessments."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionStage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A simple summary of the stage such as \"Stage 3\". The determination of the stage is disease-specific."]
    pub r#summary: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reference to a formal record of the evidence on which the staging assessment is based."]
    pub r#assessment: Vec<super::super::types::Reference>,
    #[doc = "The kind of staging, such as pathological or clinical staging."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConditionStage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#summary: Default::default(),
            r#assessment: Default::default(),
            r#type: Default::default(),
        }
    }
}
#[doc = "Supporting evidence / manifestations that are the basis of the Condition's verification status, such as evidence that confirmed or refuted the condition."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionEvidence {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A manifestation or symptom that led to the recording of this condition."]
    pub r#code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Links to other relevant information, including pathology reports."]
    pub r#detail: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConditionEvidence {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern."]
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
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
    #[doc = "Business identifiers assigned to this condition by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The clinical status of the condition."]
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The verification status to support the clinical status of the condition."]
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A category assigned to the condition."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "A subjective assessment of the severity of the condition as evaluated by the clinician."]
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identification of the condition, problem or diagnosis."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The anatomical location where this condition manifests itself."]
    pub r#body_site: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates the patient or group who the condition record is associated with."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this Condition was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Estimated or actual date or date-time  the condition began, in the opinion of the clinician."]
    pub r#onset: Option<ConditionOnset>,
    #[doc = "The date or estimated date that the condition resolved or went into remission. This is called \"abatement\" because of the many overloaded connotations associated with \"remission\" or \"resolution\" - Conditions are never really resolved, but they can abate."]
    pub r#abatement: Option<ConditionAbatement>,
    #[doc = "The recordedDate represents when this particular Condition record was created in the system, which is often a system-generated date."]
    pub r#recorded_date: Option<super::super::types::DateTime>,
    #[doc = "Individual who recorded the record and takes responsibility for its content."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Individual who is making the condition statement."]
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    #[doc = "Clinical stage or grade of a condition. May include formal severity assessments."]
    pub r#stage: Vec<ConditionStage>,
    #[doc = "Supporting evidence / manifestations that are the basis of the Condition's verification status, such as evidence that confirmed or refuted the condition."]
    pub r#evidence: Vec<ConditionEvidence>,
    #[doc = "Additional information about the Condition. This is a general notes/comments entry  for description of the Condition, its diagnosis and prognosis."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for Condition {
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
            r#clinical_status: Default::default(),
            r#verification_status: Default::default(),
            r#category: Default::default(),
            r#severity: Default::default(),
            r#code: Default::default(),
            r#body_site: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#onset: Default::default(),
            r#abatement: Default::default(),
            r#recorded_date: Default::default(),
            r#recorder: Default::default(),
            r#asserter: Default::default(),
            r#stage: Default::default(),
            r#evidence: Default::default(),
            r#note: Default::default(),
        }
    }
}
