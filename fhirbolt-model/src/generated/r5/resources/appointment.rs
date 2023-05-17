// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "List of participants involved in the appointment."]
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Role of participant in the appointment."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Participation period of the actor."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The individual, device, location, or service participating in the appointment."]
    pub r#actor: Option<Box<super::super::types::Reference>>,
    #[doc = "Whether this participant is required to be present at the meeting. If false, the participant is optional."]
    pub r#required: Option<super::super::types::Boolean>,
    #[doc = "Participation status of the actor."]
    pub r#status: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for AppointmentParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#period: Default::default(),
            r#actor: Default::default(),
            r#required: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Information about weekly recurring appointments."]
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentRecurrenceTemplateWeeklyTemplate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates that recurring appointments should occur on Mondays."]
    pub r#monday: Option<super::super::types::Boolean>,
    #[doc = "Indicates that recurring appointments should occur on Tuesdays."]
    pub r#tuesday: Option<super::super::types::Boolean>,
    #[doc = "Indicates that recurring appointments should occur on Wednesdays."]
    pub r#wednesday: Option<super::super::types::Boolean>,
    #[doc = "Indicates that recurring appointments should occur on Thursdays."]
    pub r#thursday: Option<super::super::types::Boolean>,
    #[doc = "Indicates that recurring appointments should occur on Fridays."]
    pub r#friday: Option<super::super::types::Boolean>,
    #[doc = "Indicates that recurring appointments should occur on Saturdays."]
    pub r#saturday: Option<super::super::types::Boolean>,
    #[doc = "Indicates that recurring appointments should occur on Sundays."]
    pub r#sunday: Option<super::super::types::Boolean>,
    #[doc = "The interval defines if the recurrence is every nth week. The default is every week, so it is expected that this value will be 2 or more.\r\re.g. For recurring every second week this interval would be 2, or every third week the interval would be 3."]
    pub r#week_interval: Option<super::super::types::PositiveInt>,
}
#[allow(clippy::derivable_impls)]
impl Default for AppointmentRecurrenceTemplateWeeklyTemplate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#monday: Default::default(),
            r#tuesday: Default::default(),
            r#wednesday: Default::default(),
            r#thursday: Default::default(),
            r#friday: Default::default(),
            r#saturday: Default::default(),
            r#sunday: Default::default(),
            r#week_interval: Default::default(),
        }
    }
}
#[doc = "Information about monthly recurring appointments."]
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentRecurrenceTemplateMonthlyTemplate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates that appointments in the series of recurring appointments should occur on a specific day of the month."]
    pub r#day_of_month: Option<super::super::types::PositiveInt>,
    #[doc = "Indicates which week within a month the appointments in the series of recurring appointments should occur on."]
    pub r#nth_week_of_month: Option<Box<super::super::types::Coding>>,
    #[doc = "Indicates which day of the week the recurring appointments should occur each nth week."]
    pub r#day_of_week: Option<Box<super::super::types::Coding>>,
    #[doc = "Indicates that recurring appointments should occur every nth month."]
    pub r#month_interval: super::super::types::PositiveInt,
}
#[allow(clippy::derivable_impls)]
impl Default for AppointmentRecurrenceTemplateMonthlyTemplate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#day_of_month: Default::default(),
            r#nth_week_of_month: Default::default(),
            r#day_of_week: Default::default(),
            r#month_interval: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Information about yearly recurring appointments."]
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentRecurrenceTemplateYearlyTemplate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Appointment recurs every nth year."]
    pub r#year_interval: super::super::types::PositiveInt,
}
#[allow(clippy::derivable_impls)]
impl Default for AppointmentRecurrenceTemplateYearlyTemplate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#year_interval: super::super::types::PositiveInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The details of the recurrence pattern or template that is used to generate recurring appointments."]
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentRecurrenceTemplate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The timezone of the recurring appointment occurrences."]
    pub r#timezone: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How often the appointment series should recur."]
    pub r#recurrence_type: Box<super::super::types::CodeableConcept>,
    #[doc = "Recurring appointments will not occur after this date."]
    pub r#last_occurrence_date: Option<super::super::types::Date>,
    #[doc = "How many appointments are planned in the recurrence."]
    pub r#occurrence_count: Option<super::super::types::PositiveInt>,
    #[doc = "The list of specific dates that will have appointments generated."]
    pub r#occurrence_date: Vec<super::super::types::Date>,
    #[doc = "Information about weekly recurring appointments."]
    pub r#weekly_template: Option<AppointmentRecurrenceTemplateWeeklyTemplate>,
    #[doc = "Information about monthly recurring appointments."]
    pub r#monthly_template: Option<AppointmentRecurrenceTemplateMonthlyTemplate>,
    #[doc = "Information about yearly recurring appointments."]
    pub r#yearly_template: Option<AppointmentRecurrenceTemplateYearlyTemplate>,
    #[doc = "Any dates, such as holidays, that should be excluded from the recurrence."]
    pub r#excluding_date: Vec<super::super::types::Date>,
    #[doc = "Any dates, such as holidays, that should be excluded from the recurrence."]
    pub r#excluding_recurrence_id: Vec<super::super::types::PositiveInt>,
}
#[allow(clippy::derivable_impls)]
impl Default for AppointmentRecurrenceTemplate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#timezone: Default::default(),
            r#recurrence_type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#last_occurrence_date: Default::default(),
            r#occurrence_count: Default::default(),
            r#occurrence_date: Default::default(),
            r#weekly_template: Default::default(),
            r#monthly_template: Default::default(),
            r#yearly_template: Default::default(),
            r#excluding_date: Default::default(),
            r#excluding_recurrence_id: Default::default(),
        }
    }
}
#[doc = "A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s)."]
#[derive(Debug, Clone, PartialEq)]
pub struct Appointment {
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
    #[doc = "This records identifiers associated with this appointment concern that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate (e.g. in CDA documents, or in written / printed documentation)."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The overall status of the Appointment. Each of the participants has their own participation status which indicates their involvement in the process, however this status indicates the shared status."]
    pub r#status: super::super::types::Code,
    #[doc = "The coded reason for the appointment being cancelled. This is often used in reporting/billing/futher processing to determine if further actions are required, or specific fees apply."]
    pub r#cancellation_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Concepts representing classification of patient encounter such as ambulatory (outpatient), inpatient, emergency, home health or others due to local variations."]
    pub r#class: Vec<super::super::types::CodeableConcept>,
    #[doc = "A broad categorization of the service that is to be performed during this appointment."]
    pub r#service_category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The specific service that is to be performed during this appointment."]
    pub r#service_type: Vec<super::super::types::CodeableReference>,
    #[doc = "The specialty of a practitioner that would be required to perform the service requested in this appointment."]
    pub r#specialty: Vec<super::super::types::CodeableConcept>,
    #[doc = "The style of appointment or patient that has been booked in the slot (not service type)."]
    pub r#appointment_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The reason that this appointment is being scheduled. This is more clinical than administrative. This can be coded, or as specified using information from another resource. When the patient arrives and the encounter begins it may be used as the admission diagnosis. The indication will typically be a Condition (with other resources referenced in the evidence.detail), or a Procedure."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "The priority of the appointment. Can be used to make informed decisions if needing to re-prioritize appointments. (The iCal Standard specifies 0 as undefined, 1 as highest, 9 as lowest priority)."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The brief description of the appointment as would be shown on a subject line in a meeting request, or appointment list. Detailed or expanded information should be put in the note field."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Appointment replaced by this Appointment in cases where there is a cancellation, the details of the cancellation can be found in the cancellationReason property (on the referenced resource)."]
    pub r#replaces: Vec<super::super::types::Reference>,
    #[doc = "Connection details of a virtual service (e.g. conference call)."]
    pub r#virtual_service: Vec<super::super::types::VirtualServiceDetail>,
    #[doc = "Additional information to support the appointment provided when making the appointment."]
    pub r#supporting_information: Vec<super::super::types::Reference>,
    #[doc = "The previous appointment in a series of related appointments."]
    pub r#previous_appointment: Option<Box<super::super::types::Reference>>,
    #[doc = "The originating appointment in a recurring set of related appointments."]
    pub r#originating_appointment: Option<Box<super::super::types::Reference>>,
    #[doc = "Date/Time that the appointment is to take place."]
    pub r#start: Option<super::super::types::Instant>,
    #[doc = "Date/Time that the appointment is to conclude."]
    pub r#end: Option<super::super::types::Instant>,
    #[doc = "Number of minutes that the appointment is to take. This can be less than the duration between the start and end times.  For example, where the actual time of appointment is only an estimate or if a 30 minute appointment is being requested, but any time would work.  Also, if there is, for example, a planned 15 minute break in the middle of a long appointment, the duration may be 15 minutes less than the difference between the start and end."]
    pub r#minutes_duration: Option<super::super::types::PositiveInt>,
    #[doc = "A set of date ranges (potentially including times) that the appointment is preferred to be scheduled within.\n\nThe duration (usually in minutes) could also be provided to indicate the length of the appointment to fill and populate the start/end times for the actual allocated time. However, in other situations the duration may be calculated by the scheduling system."]
    pub r#requested_period: Vec<super::super::types::Period>,
    #[doc = "The slots from the participants' schedules that will be filled by the appointment."]
    pub r#slot: Vec<super::super::types::Reference>,
    #[doc = "The set of accounts that is expected to be used for billing the activities that result from this Appointment."]
    pub r#account: Vec<super::super::types::Reference>,
    #[doc = "The date that this appointment was initially created. This could be different to the meta.lastModified value on the initial entry, as this could have been before the resource was created on the FHIR server, and should remain unchanged over the lifespan of the appointment."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "The date/time describing when the appointment was cancelled."]
    pub r#cancellation_date: Option<super::super::types::DateTime>,
    #[doc = "Additional notes/comments about the appointment."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "While Appointment.note contains information for internal use, Appointment.patientInstructions is used to capture patient facing information about the Appointment (e.g. please bring your referral or fast from 8pm night before)."]
    pub r#patient_instruction: Vec<super::super::types::CodeableReference>,
    #[doc = "The request this appointment is allocated to assess (e.g. incoming referral or procedure request)."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The patient or group associated with the appointment, if they are to be present (usually) then they should also be included in the participant backbone element."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "List of participants involved in the appointment."]
    pub r#participant: Vec<AppointmentParticipant>,
    #[doc = "The sequence number that identifies a specific appointment in a recurring pattern."]
    pub r#recurrence_id: Option<super::super::types::PositiveInt>,
    #[doc = "This appointment varies from the recurring pattern."]
    pub r#occurrence_changed: Option<super::super::types::Boolean>,
    #[doc = "The details of the recurrence pattern or template that is used to generate recurring appointments."]
    pub r#recurrence_template: Vec<AppointmentRecurrenceTemplate>,
}
#[allow(clippy::derivable_impls)]
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#cancellation_reason: Default::default(),
            r#class: Default::default(),
            r#service_category: Default::default(),
            r#service_type: Default::default(),
            r#specialty: Default::default(),
            r#appointment_type: Default::default(),
            r#reason: Default::default(),
            r#priority: Default::default(),
            r#description: Default::default(),
            r#replaces: Default::default(),
            r#virtual_service: Default::default(),
            r#supporting_information: Default::default(),
            r#previous_appointment: Default::default(),
            r#originating_appointment: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
            r#minutes_duration: Default::default(),
            r#requested_period: Default::default(),
            r#slot: Default::default(),
            r#account: Default::default(),
            r#created: Default::default(),
            r#cancellation_date: Default::default(),
            r#note: Default::default(),
            r#patient_instruction: Default::default(),
            r#based_on: Default::default(),
            r#subject: Default::default(),
            r#participant: Default::default(),
            r#recurrence_id: Default::default(),
            r#occurrence_changed: Default::default(),
            r#recurrence_template: Default::default(),
        }
    }
}
