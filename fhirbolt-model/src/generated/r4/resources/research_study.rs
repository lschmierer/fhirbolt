// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Describes an expected sequence of events for one of the participants of a study.  E.g. Exposure to drug A, wash-out, exposure to drug B, wash-out, follow-up."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyArm {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique, human-readable label for this arm of the study."]
    pub r#name: super::super::types::String,
    #[doc = "Categorization of study arm, e.g. experimental, active comparator, placebo comparater."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A succinct description of the path through the study that would be followed by a subject adhering to this arm."]
    pub r#description: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ResearchStudyArm {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Default::default(),
            r#description: Default::default(),
        }
    }
}
#[doc = "A goal that the study is aiming to achieve in terms of a scientific question to be answered by the analysis of data collected during the study."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudyObjective {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique, human-readable label for this objective of the study."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The kind of study objective."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ResearchStudyObjective {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#type: Default::default(),
        }
    }
}
#[doc = "A process where a researcher or organization plans and then executes a series of steps intended to increase the field of healthcare-related knowledge.  This includes studies of safety, efficacy, comparative effectiveness and other information about medications, devices, therapies and other interventional and investigative techniques.  A ResearchStudy involves the gathering of information about human or animal subjects."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchStudy {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifiers assigned to this research study by the sponsor or other systems."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A short, descriptive user-friendly label for the study."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The set of steps expected to be performed as part of the execution of the study."]
    pub r#protocol: Vec<super::super::types::Reference>,
    #[doc = "A larger research study of which this particular study is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "The current state of the study."]
    pub r#status: super::super::types::Code,
    #[doc = "The type of study based upon the intent of the study's activities. A classification of the intent of the study."]
    pub r#primary_purpose_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The stage in the progression of a therapy from initial experimental use in humans in clinical trials to post-market evaluation."]
    pub r#phase: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Codes categorizing the type of study such as investigational vs. observational, type of blinding, type of randomization, safety vs. efficacy, etc."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The medication(s), food(s), therapy(ies), device(s) or other concerns or interventions that the study is seeking to gain more information about."]
    pub r#focus: Vec<super::super::types::CodeableConcept>,
    #[doc = "The condition that is the focus of the study.  For example, In a study to examine risk factors for Lupus, might have as an inclusion criterion \"healthy volunteer\", but the target condition code would be a Lupus SNOMED code."]
    pub r#condition: Vec<super::super::types::CodeableConcept>,
    #[doc = "Contact details to assist a user in learning more about or engaging with the study."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "Citations, references and other related documents."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "Key terms to aid in searching for or filtering the study."]
    pub r#keyword: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates a country, state or other region where the study is taking place."]
    pub r#location: Vec<super::super::types::CodeableConcept>,
    #[doc = "A full description of how the study is being conducted."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Reference to a Group that defines the criteria for and quantity of subjects participating in the study.  E.g. \" 200 female Europeans between the ages of 20 and 45 with early onset diabetes\"."]
    pub r#enrollment: Vec<super::super::types::Reference>,
    #[doc = "Identifies the start date and the expected (or actual, depending on status) end date for the study."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "An organization that initiates the investigation and is legally responsible for the study."]
    pub r#sponsor: Option<Box<super::super::types::Reference>>,
    #[doc = "A researcher in a study who oversees multiple aspects of the study, such as concept development, protocol writing, protocol submission for IRB approval, participant recruitment, informed consent, data collection, analysis, interpretation and presentation."]
    pub r#principal_investigator: Option<Box<super::super::types::Reference>>,
    #[doc = "A facility in which study activities are conducted."]
    pub r#site: Vec<super::super::types::Reference>,
    #[doc = "A description and/or code explaining the premature termination of the study."]
    pub r#reason_stopped: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Comments made about the study by the performer, subject or other participants."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Describes an expected sequence of events for one of the participants of a study.  E.g. Exposure to drug A, wash-out, exposure to drug B, wash-out, follow-up."]
    pub r#arm: Vec<ResearchStudyArm>,
    #[doc = "A goal that the study is aiming to achieve in terms of a scientific question to be answered by the analysis of data collected during the study."]
    pub r#objective: Vec<ResearchStudyObjective>,
}
#[allow(clippy::derivable_impls)]
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
            r#identifier: Default::default(),
            r#title: Default::default(),
            r#protocol: Default::default(),
            r#part_of: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#primary_purpose_type: Default::default(),
            r#phase: Default::default(),
            r#category: Default::default(),
            r#focus: Default::default(),
            r#condition: Default::default(),
            r#contact: Default::default(),
            r#related_artifact: Default::default(),
            r#keyword: Default::default(),
            r#location: Default::default(),
            r#description: Default::default(),
            r#enrollment: Default::default(),
            r#period: Default::default(),
            r#sponsor: Default::default(),
            r#principal_investigator: Default::default(),
            r#site: Default::default(),
            r#reason_stopped: Default::default(),
            r#note: Default::default(),
            r#arm: Default::default(),
            r#objective: Default::default(),
        }
    }
}
