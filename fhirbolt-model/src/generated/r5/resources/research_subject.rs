// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "The current state (status) of the subject and resons for status change where appropriate."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchSubjectProgress {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies the aspect of the subject's journey that the state refers to."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The current state of the subject."]
    pub r#subject_state: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The milestones the subject has passed through."]
    pub r#milestone: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The reason for the state change.  If coded it should follow the formal subject state model."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date when the new status started."]
    pub r#start_date: Option<super::super::types::DateTime>,
    #[doc = "The date when the state ended."]
    pub r#end_date: Option<super::super::types::DateTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for ResearchSubjectProgress {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#subject_state: Default::default(),
            r#milestone: Default::default(),
            r#reason: Default::default(),
            r#start_date: Default::default(),
            r#end_date: Default::default(),
        }
    }
}
#[doc = "A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study."]
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchSubject {
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
    #[doc = "Identifiers assigned to this research subject for a study."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The publication state of the resource (not of the subject)."]
    pub r#status: super::super::types::Code,
    #[doc = "The current state (status) of the subject and resons for status change where appropriate."]
    pub r#progress: Vec<ResearchSubjectProgress>,
    #[doc = "The dates the subject began and ended their participation in the study."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Reference to the study the subject is participating in."]
    pub r#study: Box<super::super::types::Reference>,
    #[doc = "The record of the person, animal or other entity involved in the study."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The name of the arm in the study the subject is expected to follow as part of this study."]
    pub r#assigned_comparison_group: Option<super::super::types::Id>,
    #[doc = "The name of the arm in the study the subject actually followed as part of this study."]
    pub r#actual_comparison_group: Option<super::super::types::Id>,
    #[doc = "A record of the patient's informed agreement to participate in the study."]
    pub r#consent: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ResearchSubject {
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
            r#progress: Default::default(),
            r#period: Default::default(),
            r#study: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#assigned_comparison_group: Default::default(),
            r#actual_comparison_group: Default::default(),
            r#consent: Default::default(),
        }
    }
}
