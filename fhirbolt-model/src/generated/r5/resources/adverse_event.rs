// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "The date (and perhaps time) when the adverse event occurred."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AdverseEventOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "Identifies the actual instance of what caused the adverse event.  May be a substance, medication, medication administration, medication statement or a device."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AdverseEventSuspectEntityInstance {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The item that is suspected to have increased the probability or severity of the adverse event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AdverseEventContributingFactorItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "The action that contributed to avoiding the adverse event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AdverseEventPreventiveActionItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "The ameliorating action taken after the adverse event occured in order to reduce the extent of harm."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AdverseEventMitigatingActionItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Relevant past history for the subject. In a clinical care context, an example being a patient had an adverse event following a pencillin administration and the patient had a previously documented penicillin allergy. In a clinical trials context, an example is a bunion or rash that was present prior to the study. Additionally, the supporting item can be a document that is relevant to this instance of the adverse event that is not part of the subject's medical history. For example, a clinical note, staff list, or material safety data sheet (MSDS).  Supporting information is not a contributing factor, preventive action, or mitigating action."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AdverseEventSupportingInfoItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Indicates who or what participated in the adverse event and how they were involved."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Distinguishes the type of involvement of the actor in the adverse event, such as contributor or informant."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what participated in the event."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl Default for AdverseEventParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "Information on the possible cause of the event."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventSuspectEntityCausality {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The method of evaluating the relatedness of the suspected entity to the event."]
    pub r#assessment_method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The result of the assessment regarding the relatedness of the suspected entity to the event."]
    pub r#entity_relatedness: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The author of the information on the possible cause of the event."]
    pub r#author: Option<Box<super::super::types::Reference>>,
}
impl Default for AdverseEventSuspectEntityCausality {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#assessment_method: Default::default(),
            r#entity_relatedness: Default::default(),
            r#author: Default::default(),
        }
    }
}
#[doc = "Describes the entity that is suspected to have caused the adverse event."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventSuspectEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the actual instance of what caused the adverse event.  May be a substance, medication, medication administration, medication statement or a device."]
    pub r#instance: AdverseEventSuspectEntityInstance,
    #[doc = "Information on the possible cause of the event."]
    pub r#causality: Option<AdverseEventSuspectEntityCausality>,
}
impl Default for AdverseEventSuspectEntity {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#instance: Default::default(),
            r#causality: Default::default(),
        }
    }
}
#[doc = "The contributing factors suspected to have increased the probability or severity of the adverse event."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventContributingFactor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The item that is suspected to have increased the probability or severity of the adverse event."]
    pub r#item: AdverseEventContributingFactorItem,
}
impl Default for AdverseEventContributingFactor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "Preventive actions that contributed to avoiding the adverse event."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventPreventiveAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The action that contributed to avoiding the adverse event."]
    pub r#item: AdverseEventPreventiveActionItem,
}
impl Default for AdverseEventPreventiveAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "The ameliorating action taken after the adverse event occured in order to reduce the extent of harm."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventMitigatingAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The ameliorating action taken after the adverse event occured in order to reduce the extent of harm."]
    pub r#item: AdverseEventMitigatingActionItem,
}
impl Default for AdverseEventMitigatingAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "Supporting information relevant to the event."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Relevant past history for the subject. In a clinical care context, an example being a patient had an adverse event following a pencillin administration and the patient had a previously documented penicillin allergy. In a clinical trials context, an example is a bunion or rash that was present prior to the study. Additionally, the supporting item can be a document that is relevant to this instance of the adverse event that is not part of the subject's medical history. For example, a clinical note, staff list, or material safety data sheet (MSDS).  Supporting information is not a contributing factor, preventive action, or mitigating action."]
    pub r#item: AdverseEventSupportingInfoItem,
}
impl Default for AdverseEventSupportingInfo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "An event (i.e. any change to current patient status) that may be related to unintended effects on a patient or research participant. The unintended effects may require additional monitoring, treatment, hospitalization, or may result in death. The AdverseEvent resource also extends to potential or avoided events that could have had such effects. There are two major domains where the AdverseEvent resource is expected to be used. One is in clinical care reported adverse events and the other is in reporting adverse events in clinical  research trial management.  Adverse events can be reported by healthcare providers, patients, caregivers or by medical products manufacturers.  Given the differences between these two concepts, we recommend consulting the domain specific implementation guides when implementing the AdverseEvent Resource. The implementation guides include specific extensions, value sets and constraints."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEvent {
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
    #[doc = "Business identifiers assigned to this adverse event by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The current state of the adverse event or potential adverse event."]
    pub r#status: super::super::types::Code,
    #[doc = "Whether the event actually happened or was a near miss. Note that this is independent of whether anyone was affected or harmed or how severely."]
    pub r#actuality: super::super::types::Code,
    #[doc = "The overall type of event, intended for search and filtering purposes."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific event that occurred or that was averted, such as patient fall, wrong organ removed, or wrong blood transfused."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This subject or group impacted by the event."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter associated with the start of the AdverseEvent."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and perhaps time) when the adverse event occurred."]
    pub r#occurrence: Option<AdverseEventOccurrence>,
    #[doc = "Estimated or actual date the AdverseEvent began, in the opinion of the reporter."]
    pub r#detected: Option<super::super::types::DateTime>,
    #[doc = "The date on which the existence of the AdverseEvent was first recorded."]
    pub r#recorded_date: Option<super::super::types::DateTime>,
    #[doc = "Information about the condition that occurred as a result of the adverse event, such as hives due to the exposure to a substance (for example, a drug or a chemical) or a broken leg as a result of the fall."]
    pub r#resulting_effect: Vec<Box<super::super::types::Reference>>,
    #[doc = "The information about where the adverse event occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Assessment whether this event, or averted event, was of clinical importance."]
    pub r#seriousness: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the type of outcome from the adverse event, such as resolved, recovering, ongoing, resolved-with-sequelae, or fatal."]
    pub r#outcome: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information on who recorded the adverse event.  May be the patient or a practitioner."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates who or what participated in the adverse event and how they were involved."]
    pub r#participant: Vec<AdverseEventParticipant>,
    #[doc = "The research study that the subject is enrolled in."]
    pub r#study: Vec<Box<super::super::types::Reference>>,
    #[doc = "Considered likely or probable or anticipated in the research study.  Whether the reported event matches any of the outcomes for the patient that are considered by the study as known or likely."]
    pub r#expected_in_research_study: Option<super::super::types::Boolean>,
    #[doc = "Describes the entity that is suspected to have caused the adverse event."]
    pub r#suspect_entity: Vec<AdverseEventSuspectEntity>,
    #[doc = "The contributing factors suspected to have increased the probability or severity of the adverse event."]
    pub r#contributing_factor: Vec<AdverseEventContributingFactor>,
    #[doc = "Preventive actions that contributed to avoiding the adverse event."]
    pub r#preventive_action: Vec<AdverseEventPreventiveAction>,
    #[doc = "The ameliorating action taken after the adverse event occured in order to reduce the extent of harm."]
    pub r#mitigating_action: Vec<AdverseEventMitigatingAction>,
    #[doc = "Supporting information relevant to the event."]
    pub r#supporting_info: Vec<AdverseEventSupportingInfo>,
    #[doc = "Comments made about the adverse event by the performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl Default for AdverseEvent {
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
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#actuality: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#category: Default::default(),
            r#code: Default::default(),
            r#subject: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#detected: Default::default(),
            r#recorded_date: Default::default(),
            r#resulting_effect: Default::default(),
            r#location: Default::default(),
            r#seriousness: Default::default(),
            r#outcome: Default::default(),
            r#recorder: Default::default(),
            r#participant: Default::default(),
            r#study: Default::default(),
            r#expected_in_research_study: Default::default(),
            r#suspect_entity: Default::default(),
            r#contributing_factor: Default::default(),
            r#preventive_action: Default::default(),
            r#mitigating_action: Default::default(),
            r#supporting_info: Default::default(),
            r#note: Default::default(),
        }
    }
}
