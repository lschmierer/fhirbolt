// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The list of people responsible for providing the service."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Role of participant in encounter."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The period of time that the specified participant participated in the encounter. These can overlap or be sub-sets of the overall encounter's period."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Person involved in the encounter, the patient/group is also included here to indicate that the patient was actually participating in the encounter. Not including the patient here covers use cases such as a case meeting between practitioners about a patient - non contact times."]
    pub r#actor: Option<Box<super::super::types::Reference>>,
}
impl Default for EncounterParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#period: Default::default(),
            r#actor: Default::default(),
        }
    }
}
#[doc = "The list of medical reasons that are expected to be addressed during the episode of care."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterReason {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "What the reason value should be used as e.g. Chief Complaint, Health Concern, Health Maintenance (including screening)."]
    pub r#use: Vec<super::super::types::CodeableConcept>,
    #[doc = "Reason the encounter takes place, expressed as a code or a reference to another resource. For admissions, this can be used for a coded admission diagnosis."]
    pub r#value: Vec<super::super::types::CodeableReference>,
}
impl Default for EncounterReason {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#use: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "The list of diagnosis relevant to this encounter."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The coded diagnosis or a reference to a Condition (with other resources referenced in the evidence.detail), the use property will indicate the purpose of this specific diagnosis."]
    pub r#condition: Vec<super::super::types::CodeableReference>,
    #[doc = "Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)."]
    pub r#use: Vec<super::super::types::CodeableConcept>,
}
impl Default for EncounterDiagnosis {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#condition: Default::default(),
            r#use: Default::default(),
        }
    }
}
#[doc = "Details about the stay during which a healthcare service is provided.\r\rThis does not describe the event of admitting the patient, but rather any information that is relevant from the time of admittance until the time of discharge."]
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterAdmission {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Pre-admission identifier."]
    pub r#pre_admission_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The location/organization from which the patient came before admission."]
    pub r#origin: Option<Box<super::super::types::Reference>>,
    #[doc = "From where patient was admitted (physician referral, transfer)."]
    pub r#admit_source: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates that this encounter is directly related to a prior admission, often because the conditions addressed in the prior admission were not fully addressed."]
    pub r#re_admission: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Location/organization to which the patient is discharged."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Category or kind of location after discharge."]
    pub r#discharge_disposition: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for EncounterAdmission {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#pre_admission_identifier: Default::default(),
            r#origin: Default::default(),
            r#admit_source: Default::default(),
            r#re_admission: Default::default(),
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
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The location where the encounter takes place."]
    pub r#location: Box<super::super::types::Reference>,
    #[doc = "The status of the participants' presence at the specified location during the period specified. If the participant is no longer at the location, then the period will have an end date/time."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "This will be used to specify the required levels (bed/ward/room/etc.) desired to be recorded to simplify either messaging or query."]
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
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
            r#form: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s)."]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier(s) by which this encounter is known."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The current state of the encounter (not the state of the patient within the encounter - that is subjectState)."]
    pub r#status: super::super::types::Code,
    #[doc = "Concepts representing classification of patient encounter such as ambulatory (outpatient), inpatient, emergency, home health or others due to local variations."]
    pub r#class: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates the urgency of the encounter."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled nursing, rehabilitation)."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Broad categorization of the service that is to be provided (e.g. cardiology)."]
    pub r#service_type: Vec<super::super::types::CodeableReference>,
    #[doc = "The patient or group related to this encounter. In some use-cases the patient MAY not be present, such as a case meeting about a patient between several practitioners or a careteam."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The subjectStatus value can be used to track the patient's status within the encounter. It details whether the patient has arrived or departed, has been triaged or is currently in a waiting status."]
    pub r#subject_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Where a specific encounter should be classified as a part of a specific episode(s) of care this field should be used. This association can facilitate grouping of related encounters together for a specific purpose, such as government reporting, issue tracking, association via a common problem.  The association is recorded on the encounter as these are typically created after the episode of care and grouped on entry rather than editing the episode of care to append another encounter to it (the episode of care could span years)."]
    pub r#episode_of_care: Vec<super::super::types::Reference>,
    #[doc = "The request this encounter satisfies (e.g. incoming referral or procedure request)."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The group(s) of individuals, organizations that are allocated to participate in this encounter. The participants backbone will record the actuals of when these individuals participated during the encounter."]
    pub r#care_team: Vec<super::super::types::Reference>,
    #[doc = "Another Encounter of which this encounter is a part of (administratively or in time)."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization that is primarily responsible for this Encounter's services. This MAY be the same as the organization on the Patient record, however it could be different, such as if the actor performing the services was from an external organization (which may be billed seperately) for an external consultation.  Refer to the colonoscopy example on the Encounter examples tab."]
    pub r#service_provider: Option<Box<super::super::types::Reference>>,
    #[doc = "The list of people responsible for providing the service."]
    pub r#participant: Vec<EncounterParticipant>,
    #[doc = "The appointment that scheduled this encounter."]
    pub r#appointment: Vec<super::super::types::Reference>,
    #[doc = "Connection details of a virtual service (e.g. conference call)."]
    pub r#virtual_service: Vec<super::super::types::VirtualServiceDetail>,
    #[doc = "The actual start and end time of the encounter."]
    pub r#actual_period: Option<Box<super::super::types::Period>>,
    #[doc = "The planned start date/time (or admission date) of the encounter."]
    pub r#planned_start_date: Option<super::super::types::DateTime>,
    #[doc = "The planned end date/time (or discharge date) of the encounter."]
    pub r#planned_end_date: Option<super::super::types::DateTime>,
    #[doc = "Actual quantity of time the encounter lasted. This excludes the time during leaves of absence.\r\rWhen missing it is the time in between the start and end values."]
    pub r#length: Option<Box<super::super::types::Duration>>,
    #[doc = "The list of medical reasons that are expected to be addressed during the episode of care."]
    pub r#reason: Vec<EncounterReason>,
    #[doc = "The list of diagnosis relevant to this encounter."]
    pub r#diagnosis: Vec<EncounterDiagnosis>,
    #[doc = "The set of accounts that may be used for billing for this Encounter."]
    pub r#account: Vec<super::super::types::Reference>,
    #[doc = "Diet preferences reported by the patient."]
    pub r#diet_preference: Vec<super::super::types::CodeableConcept>,
    #[doc = "Any special requests that have been made for this encounter, such as the provision of specific equipment or other things."]
    pub r#special_arrangement: Vec<super::super::types::CodeableConcept>,
    #[doc = "Special courtesies that may be provided to the patient during the encounter (VIP, board member, professional courtesy)."]
    pub r#special_courtesy: Vec<super::super::types::CodeableConcept>,
    #[doc = "Details about the stay during which a healthcare service is provided.\r\rThis does not describe the event of admitting the patient, but rather any information that is relevant from the time of admittance until the time of discharge."]
    pub r#admission: Option<EncounterAdmission>,
    #[doc = "List of locations where  the patient has been during this encounter."]
    pub r#location: Vec<EncounterLocation>,
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
            r#class: Default::default(),
            r#priority: Default::default(),
            r#type: Default::default(),
            r#service_type: Default::default(),
            r#subject: Default::default(),
            r#subject_status: Default::default(),
            r#episode_of_care: Default::default(),
            r#based_on: Default::default(),
            r#care_team: Default::default(),
            r#part_of: Default::default(),
            r#service_provider: Default::default(),
            r#participant: Default::default(),
            r#appointment: Default::default(),
            r#virtual_service: Default::default(),
            r#actual_period: Default::default(),
            r#planned_start_date: Default::default(),
            r#planned_end_date: Default::default(),
            r#length: Default::default(),
            r#reason: Default::default(),
            r#diagnosis: Default::default(),
            r#account: Default::default(),
            r#diet_preference: Default::default(),
            r#special_arrangement: Default::default(),
            r#special_courtesy: Default::default(),
            r#admission: Default::default(),
            r#location: Default::default(),
        }
    }
}
