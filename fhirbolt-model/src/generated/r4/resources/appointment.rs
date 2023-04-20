// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "List of participants involved in the appointment."]
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role of participant in the appointment."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A Person, Location/HealthcareService or Device that is participating in the appointment."]
    pub r#actor: Option<Box<super::super::types::Reference>>,
    #[doc = "Whether this participant is required to be present at the meeting. This covers a use-case where two doctors need to meet to discuss the results for a specific patient, and the patient is not required to be present."]
    pub r#required: Option<super::super::types::Code>,
    #[doc = "Participation status of the actor."]
    pub r#status: super::super::types::Code,
    #[doc = "Participation period of the actor."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl Default for AppointmentParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#actor: Default::default(),
            r#required: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: Default::default(),
        }
    }
}
#[doc = "A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s)."]
#[derive(Debug, Clone, PartialEq)]
pub struct Appointment {
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
    #[doc = "This records identifiers associated with this appointment concern that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate (e.g. in CDA documents, or in written / printed documentation)."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The overall status of the Appointment. Each of the participants has their own participation status which indicates their involvement in the process, however this status indicates the shared status."]
    pub r#status: super::super::types::Code,
    #[doc = "The coded reason for the appointment being cancelled. This is often used in reporting/billing/futher processing to determine if further actions are required, or specific fees apply."]
    pub r#cancelation_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A broad categorization of the service that is to be performed during this appointment."]
    pub r#service_category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific service that is to be performed during this appointment."]
    pub r#service_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specialty of a practitioner that would be required to perform the service requested in this appointment."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The style of appointment or patient that has been booked in the slot (not service type)."]
    pub r#appointment_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The coded reason that this appointment is being scheduled. This is more clinical than administrative."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reason the appointment has been scheduled to take place, as specified using information from another resource. When the patient arrives and the encounter begins it may be used as the admission diagnosis. The indication will typically be a Condition (with other resources referenced in the evidence.detail), or a Procedure."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "The priority of the appointment. Can be used to make informed decisions if needing to re-prioritize appointments. (The iCal Standard specifies 0 as undefined, 1 as highest, 9 as lowest priority)."]
    pub r#priority: Option<super::super::types::UnsignedInt>,
    #[doc = "The brief description of the appointment as would be shown on a subject line in a meeting request, or appointment list. Detailed or expanded information should be put in the comment field."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Additional information to support the appointment provided when making the appointment."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    #[doc = "Date/Time that the appointment is to take place."]
    pub r#start: Option<super::super::types::Instant>,
    #[doc = "Date/Time that the appointment is to conclude."]
    pub r#end: Option<super::super::types::Instant>,
    #[doc = "Number of minutes that the appointment is to take. This can be less than the duration between the start and end times.  For example, where the actual time of appointment is only an estimate or if a 30 minute appointment is being requested, but any time would work.  Also, if there is, for example, a planned 15 minute break in the middle of a long appointment, the duration may be 15 minutes less than the difference between the start and end."]
    pub r#minutes_duration: Option<super::super::types::PositiveInt>,
    #[doc = "The slots from the participants' schedules that will be filled by the appointment."]
    pub r#slot: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date that this appointment was initially created. This could be different to the meta.lastModified value on the initial entry, as this could have been before the resource was created on the FHIR server, and should remain unchanged over the lifespan of the appointment."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "Additional comments about the appointment."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "While Appointment.comment contains information for internal use, Appointment.patientInstructions is used to capture patient facing information about the Appointment (e.g. please bring your referral or fast from 8pm night before)."]
    pub r#patient_instruction: Option<super::super::types::String>,
    #[doc = "The service request this appointment is allocated to assess (e.g. incoming referral or procedure request)."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "List of participants involved in the appointment."]
    pub r#participant: Vec<AppointmentParticipant>,
    #[doc = "A set of date ranges (potentially including times) that the appointment is preferred to be scheduled within.\n\nThe duration (usually in minutes) could also be provided to indicate the length of the appointment to fill and populate the start/end times for the actual allocated time. However, in other situations the duration may be calculated by the scheduling system."]
    pub r#requested_period: Vec<Box<super::super::types::Period>>,
}
impl Default for Appointment {
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
            r#cancelation_reason: Default::default(),
            r#service_category: Default::default(),
            r#service_type: Default::default(),
            r#specialty: Default::default(),
            r#appointment_type: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#priority: Default::default(),
            r#description: Default::default(),
            r#supporting_information: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
            r#minutes_duration: Default::default(),
            r#slot: Default::default(),
            r#created: Default::default(),
            r#comment: Default::default(),
            r#patient_instruction: Default::default(),
            r#based_on: Default::default(),
            r#participant: Default::default(),
            r#requested_period: Default::default(),
        }
    }
}
