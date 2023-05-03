// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "The point in time or period over which the subject was assessed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ClinicalImpressionEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "One or more sets of investigations (signs, symptoms, etc.). The actual grouping of investigations varies greatly depending on the type and context of the assessment. These investigations may include data generated during the assessment process, or data previously generated and recorded that is pertinent to the outcomes."]
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalImpressionInvestigation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A name/code for the group (\"set\") of investigations. Typically, this will be something like \"signs\", \"symptoms\", \"clinical\", \"diagnostic\", but the list is not constrained, and others such groups such as (exposure|family|travel|nutritional) history may be used."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A record of a specific investigation that was undertaken."]
    pub r#item: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ClinicalImpressionInvestigation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#item: Default::default(),
        }
    }
}
#[doc = "Specific findings or diagnoses that were considered likely or relevant to ongoing treatment."]
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalImpressionFinding {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Specific text or code for finding or diagnosis, which may include ruled-out or resolved conditions."]
    pub r#item_codeable_concept: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific reference for finding or diagnosis, which may include ruled-out or resolved conditions."]
    pub r#item_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Which investigations support finding or diagnosis."]
    pub r#basis: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ClinicalImpressionFinding {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item_codeable_concept: Default::default(),
            r#item_reference: Default::default(),
            r#basis: Default::default(),
        }
    }
}
#[doc = "A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condition. Assessments are often 1:1 with a clinical consultation / encounter,  but this varies greatly depending on the clinical workflow. This resource is called \"ClinicalImpression\" rather than \"ClinicalAssessment\" to avoid confusion with the recording of assessment tools such as Apgar score."]
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalImpression {
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
    #[doc = "Business identifiers assigned to this clinical impression by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Identifies the workflow status of the assessment."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the ClinicalImpression."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Categorizes the type of clinical assessment performed."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A summary of the context and/or cause of the assessment - why / where it was performed, and what patient events/status prompted it."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The patient or group of individuals assessed as part of this record."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this ClinicalImpression was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The point in time or period over which the subject was assessed."]
    pub r#effective: Option<ClinicalImpressionEffective>,
    #[doc = "Indicates when the documentation of the assessment was complete."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The clinician performing the assessment."]
    pub r#assessor: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to the last assessment that was conducted on this patient. Assessments are often/usually ongoing in nature; a care provider (practitioner or team) will make new assessments on an ongoing basis as new data arises or the patient's conditions changes."]
    pub r#previous: Option<Box<super::super::types::Reference>>,
    #[doc = "A list of the relevant problems/conditions for a patient."]
    pub r#problem: Vec<super::super::types::Reference>,
    #[doc = "One or more sets of investigations (signs, symptoms, etc.). The actual grouping of investigations varies greatly depending on the type and context of the assessment. These investigations may include data generated during the assessment process, or data previously generated and recorded that is pertinent to the outcomes."]
    pub r#investigation: Vec<ClinicalImpressionInvestigation>,
    #[doc = "Reference to a specific published clinical protocol that was followed during this assessment, and/or that provides evidence in support of the diagnosis."]
    pub r#protocol: Vec<super::super::types::Uri>,
    #[doc = "A text summary of the investigations and the diagnosis."]
    pub r#summary: Option<super::super::types::String>,
    #[doc = "Specific findings or diagnoses that were considered likely or relevant to ongoing treatment."]
    pub r#finding: Vec<ClinicalImpressionFinding>,
    #[doc = "Estimate of likely outcome."]
    pub r#prognosis_codeable_concept: Vec<super::super::types::CodeableConcept>,
    #[doc = "RiskAssessment expressing likely outcome."]
    pub r#prognosis_reference: Vec<super::super::types::Reference>,
    #[doc = "Information supporting the clinical impression."]
    pub r#supporting_info: Vec<super::super::types::Reference>,
    #[doc = "Commentary about the impression, typically recorded after the impression itself was made, though supplemental notes by the original author could also appear."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for ClinicalImpression {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#code: Default::default(),
            r#description: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#effective: Default::default(),
            r#date: Default::default(),
            r#assessor: Default::default(),
            r#previous: Default::default(),
            r#problem: Default::default(),
            r#investigation: Default::default(),
            r#protocol: Default::default(),
            r#summary: Default::default(),
            r#finding: Default::default(),
            r#prognosis_codeable_concept: Default::default(),
            r#prognosis_reference: Default::default(),
            r#supporting_info: Default::default(),
            r#note: Default::default(),
        }
    }
}
