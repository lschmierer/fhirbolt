// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection."]
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentResponse {
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
    #[doc = "This records identifiers associated with this appointment response concern that are defined by business processes and/ or used to refer to it when a direct URL reference to the resource itself is not appropriate."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Appointment that this response is replying to."]
    pub r#appointment: Box<super::super::types::Reference>,
    #[doc = "Indicates that the response is proposing a different time that was initially requested.  The new proposed time will be indicated in the start and end properties."]
    pub r#proposed_new_time: Option<super::super::types::Boolean>,
    #[doc = "Date/Time that the appointment is to take place, or requested new start time."]
    pub r#start: Option<super::super::types::Instant>,
    #[doc = "This may be either the same as the appointment request to confirm the details of the appointment, or alternately a new time to request a re-negotiation of the end time."]
    pub r#end: Option<super::super::types::Instant>,
    #[doc = "Role of participant in the appointment."]
    pub r#participant_type: Vec<super::super::types::CodeableConcept>,
    #[doc = "A Person, Location, HealthcareService, or Device that is participating in the appointment."]
    pub r#actor: Option<Box<super::super::types::Reference>>,
    #[doc = "Participation status of the participant. When the status is declined or tentative if the start/end times are different to the appointment, then these times should be interpreted as a requested time change. When the status is accepted, the times can either be the time of the appointment (as a confirmation of the time) or can be empty."]
    pub r#participant_status: super::super::types::Code,
    #[doc = "Additional comments about the appointment."]
    pub r#comment: Option<super::super::types::Markdown>,
    #[doc = "Indicates that this AppointmentResponse applies to all occurrences in a recurring request."]
    pub r#recurring: Option<super::super::types::Boolean>,
    #[doc = "The original date within a recurring request. This could be used in place of the recurrenceId to be more direct (or where the template is provided through the simple list of dates in `Appointment.occurrenceDate`)."]
    pub r#occurrence_date: Option<super::super::types::Date>,
    #[doc = "The recurrence ID (sequence number) of the specific appointment when responding to a recurring request."]
    pub r#recurrence_id: Option<super::super::types::PositiveInt>,
}
#[allow(clippy::derivable_impls)]
impl Default for AppointmentResponse {
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
            r#appointment: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#proposed_new_time: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
            r#participant_type: Default::default(),
            r#actor: Default::default(),
            r#participant_status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#comment: Default::default(),
            r#recurring: Default::default(),
            r#occurrence_date: Default::default(),
            r#recurrence_id: Default::default(),
        }
    }
}
