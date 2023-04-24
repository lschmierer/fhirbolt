// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Additional names for the study."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyLabel {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Kind of name."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The name."]
    pub r#value: Option<super::super::types::String>,
}
impl Default for ResearchStudyLabel {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "Sponsors, collaborators, and other parties."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyAssociatedParty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name of associated party."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Type of association."]
    pub r#role: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the start date and the end date of the associated party in the role."]
    pub r#period: Vec<Box<super::super::types::Period>>,
    #[doc = "A categorization other than role for the associated party."]
    pub r#classifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Individual or organization associated with study (use practitionerRole to specify their organisation)."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
impl Default for ResearchStudyAssociatedParty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#role: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: Default::default(),
            r#classifier: Default::default(),
            r#party: Default::default(),
        }
    }
}
#[doc = "Status of study with time for that status."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyProgressStatus {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Label for status or state (e.g. recruitment status)."]
    pub r#state: Box<super::super::types::CodeableConcept>,
    #[doc = "An indication of whether or not the date is a known date when the state changed or will change. A value of true indicates a known date. A value of false indicates an estimated date."]
    pub r#actual: Option<super::super::types::Boolean>,
    #[doc = "Date range."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl Default for ResearchStudyProgressStatus {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#state: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#actual: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "Target or actual group of participants enrolled in study."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyRecruitment {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Estimated total number of participants to be enrolled."]
    pub r#target_number: Option<super::super::types::UnsignedInt>,
    #[doc = "Actual total number of participants enrolled in study."]
    pub r#actual_number: Option<super::super::types::UnsignedInt>,
    #[doc = "Inclusion and exclusion criteria."]
    pub r#eligibility: Option<Box<super::super::types::Reference>>,
    #[doc = "Group of participants who were enrolled in study."]
    pub r#actual_group: Option<Box<super::super::types::Reference>>,
}
impl Default for ResearchStudyRecruitment {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#target_number: Default::default(),
            r#actual_number: Default::default(),
            r#eligibility: Default::default(),
            r#actual_group: Default::default(),
        }
    }
}
#[doc = "Describes an expected event or sequence of events for one of the subjects of a study. E.g. for a living subject: exposure to drug A, wash-out, exposure to drug B, wash-out, follow-up. E.g. for a stability study: {store sample from lot A at 25 degrees for 1 month}, {store sample from lot A at 40 degrees for 1 month}."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyComparisonGroup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Allows the comparisonGroup for the study and the comparisonGroup for the subject to be linked easily."]
    pub r#link_id: Option<super::super::types::Id>,
    #[doc = "Unique, human-readable label for this comparisonGroup of the study."]
    pub r#name: super::super::types::String,
    #[doc = "Categorization of study comparisonGroup, e.g. experimental, active comparator, placebo comparater."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A succinct description of the path through the study that would be followed by a subject adhering to this comparisonGroup."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Interventions or exposures in this comparisonGroup or cohort."]
    pub r#intended_exposure: Vec<Box<super::super::types::Reference>>,
    #[doc = "Group of participants who were enrolled in study comparisonGroup."]
    pub r#observed_group: Option<Box<super::super::types::Reference>>,
}
impl Default for ResearchStudyComparisonGroup {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#name: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#type: Default::default(),
            r#description: Default::default(),
            r#intended_exposure: Default::default(),
            r#observed_group: Default::default(),
        }
    }
}
#[doc = "A goal that the study is aiming to achieve in terms of a scientific question to be answered by the analysis of data collected during the study."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyObjective {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique, human-readable label for this objective of the study."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The kind of study objective."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Free text description of the objective of the study.  This is what the study is trying to achieve rather than how it is going to achieve it (see ResearchStudy.description)."]
    pub r#description: Option<super::super::types::Markdown>,
}
impl Default for ResearchStudyObjective {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#type: Default::default(),
            r#description: Default::default(),
        }
    }
}
#[doc = "An \"outcome measure\", \"endpoint\", \"effect measure\" or \"measure of effect\" is a specific measurement or observation used to quantify the effect of experimental variables on the participants in a study, or for observational studies, to describe patterns of diseases or traits or associations with exposures, risk factors or treatment."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyOutcomeMeasure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Label for the outcome."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The parameter or characteristic being assessed as one of the values by which the study is assessed."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Description of the outcome."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Structured outcome definition."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
}
impl Default for ResearchStudyOutcomeMeasure {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#type: Default::default(),
            r#description: Default::default(),
            r#reference: Default::default(),
        }
    }
}
#[doc = "A scientific study of nature that sometimes includes processes involved in health and disease. For example, clinical trials are research studies that involve people. These studies may be related to new ways to screen, prevent, diagnose, and treat disease. They may also study certain outcomes and certain groups of people by looking at data collected in the past or future."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudy {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Canonical identifier for this study resource, represented as a globally unique URI."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "Identifiers assigned to this research study by the sponsor or other systems."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The business version for the study record."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Name for this study (computer friendly)."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The human readable name of the research study."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Additional names for the study."]
    pub r#label: Vec<ResearchStudyLabel>,
    #[doc = "The set of steps expected to be performed as part of the execution of the study."]
    pub r#protocol: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger research study of which this particular study is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "Citations, references, URLs and other related documents.  When using relatedArtifact to share URLs, the relatedArtifact.type will often be set to one of \"documentation\" or \"supported-with\" and the URL value will often be in relatedArtifact.document.url but another possible location is relatedArtifact.resource when it is a canonical URL."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "The date (and optionally time) when the ResearchStudy Resource was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the ResearchStudy Resource changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The publication state of the resource (not of the study)."]
    pub r#status: super::super::types::Code,
    #[doc = "The type of study based upon the intent of the study activities. A classification of the intent of the study."]
    pub r#primary_purpose_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The stage in the progression of a therapy from initial experimental use in humans in clinical trials to post-market evaluation."]
    pub r#phase: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Codes categorizing the type of study such as investigational vs. observational, type of blinding, type of randomization, safety vs. efficacy, etc."]
    pub r#study_design: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The medication(s), food(s), therapy(ies), device(s) or other concerns or interventions that the study is seeking to gain more information about."]
    pub r#focus: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The condition that is the focus of the study.  For example, In a study to examine risk factors for Lupus, might have as an inclusion criterion \"healthy volunteer\", but the target condition code would be a Lupus SNOMED code."]
    pub r#condition: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Key terms to aid in searching for or filtering the study."]
    pub r#keyword: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A country, state or other area where the study is taking place rather than its precise geographic location or address."]
    pub r#region: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A brief text for explaining the study."]
    pub r#description_summary: Option<super::super::types::Markdown>,
    #[doc = "A detailed and human-readable narrative of the study. E.g., study abstract."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Identifies the start date and the expected (or actual, depending on status) end date for the study."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "A facility in which study activities are conducted."]
    pub r#site: Vec<Box<super::super::types::Reference>>,
    #[doc = "Comments made about the study by the performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Additional grouping mechanism or categorization of a research study. Example: FDA regulated device, FDA regulated drug, MPG Paragraph 23b (a German legal requirement), IRB-exempt, etc. Implementation Note: do not use the classifier element to support existing semantics that are already supported thru explicit elements in the resource."]
    pub r#classifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Sponsors, collaborators, and other parties."]
    pub r#associated_party: Vec<ResearchStudyAssociatedParty>,
    #[doc = "Status of study with time for that status."]
    pub r#progress_status: Vec<ResearchStudyProgressStatus>,
    #[doc = "A description and/or code explaining the premature termination of the study."]
    pub r#why_stopped: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Target or actual group of participants enrolled in study."]
    pub r#recruitment: Option<ResearchStudyRecruitment>,
    #[doc = "Describes an expected event or sequence of events for one of the subjects of a study. E.g. for a living subject: exposure to drug A, wash-out, exposure to drug B, wash-out, follow-up. E.g. for a stability study: {store sample from lot A at 25 degrees for 1 month}, {store sample from lot A at 40 degrees for 1 month}."]
    pub r#comparison_group: Vec<ResearchStudyComparisonGroup>,
    #[doc = "A goal that the study is aiming to achieve in terms of a scientific question to be answered by the analysis of data collected during the study."]
    pub r#objective: Vec<ResearchStudyObjective>,
    #[doc = "An \"outcome measure\", \"endpoint\", \"effect measure\" or \"measure of effect\" is a specific measurement or observation used to quantify the effect of experimental variables on the participants in a study, or for observational studies, to describe patterns of diseases or traits or associations with exposures, risk factors or treatment."]
    pub r#outcome_measure: Vec<ResearchStudyOutcomeMeasure>,
    #[doc = "Link to one or more sets of results generated by the study.  Could also link to a research registry holding the results such as ClinicalTrials.gov."]
    pub r#result: Vec<Box<super::super::types::Reference>>,
}
impl Default for ResearchStudy {
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
            r#name: Default::default(),
            r#title: Default::default(),
            r#label: Default::default(),
            r#protocol: Default::default(),
            r#part_of: Default::default(),
            r#related_artifact: Default::default(),
            r#date: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#primary_purpose_type: Default::default(),
            r#phase: Default::default(),
            r#study_design: Default::default(),
            r#focus: Default::default(),
            r#condition: Default::default(),
            r#keyword: Default::default(),
            r#region: Default::default(),
            r#description_summary: Default::default(),
            r#description: Default::default(),
            r#period: Default::default(),
            r#site: Default::default(),
            r#note: Default::default(),
            r#classifier: Default::default(),
            r#associated_party: Default::default(),
            r#progress_status: Default::default(),
            r#why_stopped: Default::default(),
            r#recruitment: Default::default(),
            r#comparison_group: Default::default(),
            r#objective: Default::default(),
            r#outcome_measure: Default::default(),
            r#result: Default::default(),
        }
    }
}
