// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "The location of the patient at this point in the encounter, the multiple cardinality permits de-normalizing the levels of the location hierarchy, such as site/ward/room/bed."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterHistoryLocation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The location where the encounter takes place."]
    pub r#location: Box<super::super::types::Reference>,
    #[doc = "This will be used to specify the required levels (bed/ward/room/etc.) desired to be recorded to simplify either messaging or query."]
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for EncounterHistoryLocation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#location: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#form: Default::default(),
        }
    }
}
#[doc = "A record of significant events/milestones key data throughout the history of an Encounter"]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterHistory {
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
    #[doc = "The Encounter associated with this set of historic values."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifier(s) by which this encounter is known."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown."]
    pub r#status: super::super::types::Code,
    #[doc = "Concepts representing classification of patient encounter such as ambulatory (outpatient), inpatient, emergency, home health or others due to local variations."]
    pub r#class: Box<super::super::types::CodeableConcept>,
    #[doc = "Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled nursing, rehabilitation)."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Broad categorization of the service that is to be provided (e.g. cardiology)."]
    pub r#service_type: Vec<super::super::types::CodeableReference>,
    #[doc = "The patient or group related to this encounter. In some use-cases the patient MAY not be present, such as a case meeting about a patient between several practitioners or a careteam."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The subjectStatus value can be used to track the patient's status within the encounter. It details whether the patient has arrived or departed, has been triaged or is currently in a waiting status."]
    pub r#subject_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The start and end time associated with this set of values associated with the encounter, may be different to the planned times for various reasons."]
    pub r#actual_period: Option<Box<super::super::types::Period>>,
    #[doc = "The planned start date/time (or admission date) of the encounter."]
    pub r#planned_start_date: Option<super::super::types::DateTime>,
    #[doc = "The planned end date/time (or discharge date) of the encounter."]
    pub r#planned_end_date: Option<super::super::types::DateTime>,
    #[doc = "Actual quantity of time the encounter lasted. This excludes the time during leaves of absence.\r\rWhen missing it is the time in between the start and end values."]
    pub r#length: Option<Box<super::super::types::Duration>>,
    #[doc = "The location of the patient at this point in the encounter, the multiple cardinality permits de-normalizing the levels of the location hierarchy, such as site/ward/room/bed."]
    pub r#location: Vec<EncounterHistoryLocation>,
}
#[allow(clippy::derivable_impls)]
impl Default for EncounterHistory {
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
            r#encounter: Default::default(),
            r#identifier: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#class: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#type: Default::default(),
            r#service_type: Default::default(),
            r#subject: Default::default(),
            r#subject_status: Default::default(),
            r#actual_period: Default::default(),
            r#planned_start_date: Default::default(),
            r#planned_end_date: Default::default(),
            r#length: Default::default(),
            r#location: Default::default(),
        }
    }
}
