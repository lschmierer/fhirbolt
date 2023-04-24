// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The status history permits the encounter resource to contain the status history without needing to read through the historical versions of the resource, or even have the server store them."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterStatusHistory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "planned | arrived | triaged | in-progress | onleave | finished | cancelled +."]
    pub r#status: super::super::types::Code,
    #[doc = "The time that the episode was in the specified status."]
    pub r#period: Box<super::super::types::Period>,
}
impl Default for EncounterStatusHistory {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: {
                let mut default: Box<super::super::types::Period> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "The class history permits the tracking of the encounters transitions without needing to go  through the resource history.  This would be used for a case where an admission starts of as an emergency encounter, then transitions into an inpatient scenario. Doing this and not restarting a new encounter ensures that any lab/diagnostic results can more easily follow the patient and not require re-processing and not get lost or cancelled during a kind of discharge from emergency to inpatient."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterClassHistory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "inpatient | outpatient | ambulatory | emergency +."]
    pub r#class: Box<super::super::types::Coding>,
    #[doc = "The time that the episode was in the specified class."]
    pub r#period: Box<super::super::types::Period>,
}
impl Default for EncounterClassHistory {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#class: {
                let mut default: Box<super::super::types::Coding> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: {
                let mut default: Box<super::super::types::Period> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "The list of people responsible for providing the service."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role of participant in encounter."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The period of time that the specified participant participated in the encounter. These can overlap or be sub-sets of the overall encounter's period."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Persons involved in the encounter other than the patient."]
    pub r#individual: Option<Box<super::super::types::Reference>>,
}
impl Default for EncounterParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#period: Default::default(),
            r#individual: Default::default(),
        }
    }
}
#[doc = "The list of diagnosis relevant to this encounter."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reason the encounter takes place, as specified using information from another resource. For admissions, this is the admission diagnosis. The indication will typically be a Condition (with other resources referenced in the evidence.detail), or a Procedure."]
    pub r#condition: Box<super::super::types::Reference>,
    #[doc = "Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)."]
    pub r#use: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Ranking of the diagnosis (for each role type)."]
    pub r#rank: Option<super::super::types::PositiveInt>,
}
impl Default for EncounterDiagnosis {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#condition: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#use: Default::default(),
            r#rank: Default::default(),
        }
    }
}
#[doc = "Details about the admission to a healthcare service."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterHospitalization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Pre-admission identifier."]
    pub r#pre_admission_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The location/organization from which the patient came before admission."]
    pub r#origin: Option<Box<super::super::types::Reference>>,
    #[doc = "From where patient was admitted (physician referral, transfer)."]
    pub r#admit_source: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether this hospitalization is a readmission and why if known."]
    pub r#re_admission: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Diet preferences reported by the patient."]
    pub r#diet_preference: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Special courtesies (VIP, board member)."]
    pub r#special_courtesy: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Any special requests that have been made for this hospitalization encounter, such as the provision of specific equipment or other things."]
    pub r#special_arrangement: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Location/organization to which the patient is discharged."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Category or kind of location after discharge."]
    pub r#discharge_disposition: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for EncounterHospitalization {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#pre_admission_identifier: Default::default(),
            r#origin: Default::default(),
            r#admit_source: Default::default(),
            r#re_admission: Default::default(),
            r#diet_preference: Default::default(),
            r#special_courtesy: Default::default(),
            r#special_arrangement: Default::default(),
            r#destination: Default::default(),
            r#discharge_disposition: Default::default(),
        }
    }
}
#[doc = "List of locations where  the patient has been during this encounter."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterLocation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The location where the encounter takes place."]
    pub r#location: Box<super::super::types::Reference>,
    #[doc = "The status of the participants' presence at the specified location during the period specified. If the participant is no longer at the location, then the period will have an end date/time."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "This will be used to specify the required levels (bed/ward/room/etc.) desired to be recorded to simplify either messaging or query."]
    pub r#physical_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Time period during which the patient was present at the location."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl Default for EncounterLocation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#location: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status: Default::default(),
            r#physical_type: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "An interaction between a patient and healthcare provider(s) for the purpose of providing healthcare service(s) or assessing the health status of a patient."]
#[derive(Debug, Clone, PartialEq)]
pub struct Encounter {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifier(s) by which this encounter is known."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "planned | arrived | triaged | in-progress | onleave | finished | cancelled +."]
    pub r#status: super::super::types::Code,
    #[doc = "The status history permits the encounter resource to contain the status history without needing to read through the historical versions of the resource, or even have the server store them."]
    pub r#status_history: Vec<EncounterStatusHistory>,
    #[doc = "Concepts representing classification of patient encounter such as ambulatory (outpatient), inpatient, emergency, home health or others due to local variations."]
    pub r#class: Box<super::super::types::Coding>,
    #[doc = "The class history permits the tracking of the encounters transitions without needing to go  through the resource history.  This would be used for a case where an admission starts of as an emergency encounter, then transitions into an inpatient scenario. Doing this and not restarting a new encounter ensures that any lab/diagnostic results can more easily follow the patient and not require re-processing and not get lost or cancelled during a kind of discharge from emergency to inpatient."]
    pub r#class_history: Vec<EncounterClassHistory>,
    #[doc = "Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled nursing, rehabilitation)."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Broad categorization of the service that is to be provided (e.g. cardiology)."]
    pub r#service_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the urgency of the encounter."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient or group present at the encounter."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Where a specific encounter should be classified as a part of a specific episode(s) of care this field should be used. This association can facilitate grouping of related encounters together for a specific purpose, such as government reporting, issue tracking, association via a common problem.  The association is recorded on the encounter as these are typically created after the episode of care and grouped on entry rather than editing the episode of care to append another encounter to it (the episode of care could span years)."]
    pub r#episode_of_care: Vec<Box<super::super::types::Reference>>,
    #[doc = "The request this encounter satisfies (e.g. incoming referral or procedure request)."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The list of people responsible for providing the service."]
    pub r#participant: Vec<EncounterParticipant>,
    #[doc = "The appointment that scheduled this encounter."]
    pub r#appointment: Vec<Box<super::super::types::Reference>>,
    #[doc = "The start and end time of the encounter."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Quantity of time the encounter lasted. This excludes the time during leaves of absence."]
    pub r#length: Option<Box<super::super::types::Duration>>,
    #[doc = "Reason the encounter takes place, expressed as a code. For admissions, this can be used for a coded admission diagnosis."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reason the encounter takes place, expressed as a code. For admissions, this can be used for a coded admission diagnosis."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "The list of diagnosis relevant to this encounter."]
    pub r#diagnosis: Vec<EncounterDiagnosis>,
    #[doc = "The set of accounts that may be used for billing for this Encounter."]
    pub r#account: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details about the admission to a healthcare service."]
    pub r#hospitalization: Option<EncounterHospitalization>,
    #[doc = "List of locations where  the patient has been during this encounter."]
    pub r#location: Vec<EncounterLocation>,
    #[doc = "The organization that is primarily responsible for this Encounter's services. This MAY be the same as the organization on the Patient record, however it could be different, such as if the actor performing the services was from an external organization (which may be billed seperately) for an external consultation.  Refer to the example bundle showing an abbreviated set of Encounters for a colonoscopy."]
    pub r#service_provider: Option<Box<super::super::types::Reference>>,
    #[doc = "Another Encounter of which this encounter is a part of (administratively or in time)."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
}
impl Default for Encounter {
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
            r#status_history: Default::default(),
            r#class: {
                let mut default: Box<super::super::types::Coding> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#class_history: Default::default(),
            r#type: Default::default(),
            r#service_type: Default::default(),
            r#priority: Default::default(),
            r#subject: Default::default(),
            r#episode_of_care: Default::default(),
            r#based_on: Default::default(),
            r#participant: Default::default(),
            r#appointment: Default::default(),
            r#period: Default::default(),
            r#length: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#diagnosis: Default::default(),
            r#account: Default::default(),
            r#hospitalization: Default::default(),
            r#location: Default::default(),
            r#service_provider: Default::default(),
            r#part_of: Default::default(),
        }
    }
}
