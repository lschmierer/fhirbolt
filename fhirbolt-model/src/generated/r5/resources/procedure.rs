// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Estimated or actual date, date-time, period, or age when the procedure did occur or is occurring.  Allows a period to support complex procedures that span more than one date, and also allows for the length of the procedure to be captured."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ProcedureOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "Indicates if this record was captured as a secondary 'reported' record rather than as an original primary source-of-truth record.  It may also indicate the source of the report."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ProcedureReported {
    Boolean(Box<super::super::types::Boolean>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Indicates who or what performed the procedure and how they were involved."]
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedurePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Distinguishes the type of involvement of the performer in the procedure. For example, surgeon, anaesthetist, endoscopist."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what performed the procedure."]
    pub r#actor: Box<super::super::types::Reference>,
    #[doc = "The Organization the Patient, RelatedPerson, Device, CareTeam, and HealthcareService was acting on behalf of."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "Time period during which the performer performed the procedure."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ProcedurePerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#on_behalf_of: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "A device that is implanted, removed or otherwise manipulated (calibration, battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a focal portion of the Procedure."]
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureFocalDevice {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of change that happened to the device during the procedure."]
    pub r#action: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device that was manipulated (changed) during the procedure."]
    pub r#manipulated: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ProcedureFocalDevice {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#action: Default::default(),
            r#manipulated: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "An action that is or was performed on or for a patient, practitioner, device, organization, or location. For example, this can be a physical intervention on a patient like an operation, or less invasive like long term services, counseling, or hypnotherapy.  This can be a quality or safety inspection for a location, organization, or device.  This can be an accreditation procedure on a practitioner for licensing."]
#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
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
    #[doc = "Business identifiers assigned to this procedure by the performer or other systems which remain constant as the resource is updated and is propagated from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, order set or other definition that is adhered to in whole or in part by this Procedure."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, order set or other definition that is adhered to in whole or in part by this Procedure."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A reference to a resource that contains details of the request for this procedure."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event of which this particular procedure is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "A code specifying the state of the procedure. Generally, this will be the in-progress or completed state."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the procedure."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code that classifies the procedure for searching, sorting and display purposes (e.g. \"Surgical Procedure\")."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The specific procedure that is performed. Use text if the exact nature of the procedure cannot be coded (e.g. \"Laparoscopic Appendectomy\")."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "On whom or on what the procedure was performed. This is usually an individual human, but can also be performed on animals, groups of humans or animals, organizations or practitioners (for licensing), locations or devices (for safety inspections or regulatory authorizations).  If the actual focus of the procedure is different from the subject, the focus element specifies the actual focus of the procedure."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "Who is the target of the procedure when it is not the subject of record only.  If focus is not present, then subject is the focus.  If focus is present and the subject is one of the targets of the procedure, include subject as a focus as well. If focus is present and the subject is not included in focus, it implies that the procedure was only targeted on the focus. For example, when a caregiver is given education for a patient, the caregiver would be the focus and the procedure record is associated with the subject (e.g. patient).  For example, use focus when recording the target of the education, training, or counseling is the parent or relative of a patient."]
    pub r#focus: Option<Box<super::super::types::Reference>>,
    #[doc = "The Encounter during which this Procedure was created or performed or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Estimated or actual date, date-time, period, or age when the procedure did occur or is occurring.  Allows a period to support complex procedures that span more than one date, and also allows for the length of the procedure to be captured."]
    pub r#occurrence: Option<ProcedureOccurrence>,
    #[doc = "The date the occurrence of the procedure was first captured in the record regardless of Procedure.status (potentially after the occurrence of the event)."]
    pub r#recorded: Option<super::super::types::DateTime>,
    #[doc = "Individual who recorded the record and takes responsibility for its content."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates if this record was captured as a secondary 'reported' record rather than as an original primary source-of-truth record.  It may also indicate the source of the report."]
    pub r#reported: Option<ProcedureReported>,
    #[doc = "Indicates who or what performed the procedure and how they were involved."]
    pub r#performer: Vec<ProcedurePerformer>,
    #[doc = "The location where the procedure actually happened.  E.g. a newborn at home, a tracheostomy at a restaurant."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The coded reason or reference why the procedure was performed. This may be a coded entity of some type, be present as text, or be a reference to one of several resources that justify the procedure."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "Detailed and structured anatomical location information. Multiple locations are allowed - e.g. multiple punch biopsies of a lesion."]
    pub r#body_site: Vec<super::super::types::CodeableConcept>,
    #[doc = "The outcome of the procedure - did it resolve the reasons for the procedure being performed?"]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This could be a histology result, pathology report, surgical report, etc."]
    pub r#report: Vec<super::super::types::Reference>,
    #[doc = "Any complications that occurred during the procedure, or in the immediate post-performance period. These are generally tracked separately from the notes, which will typically describe the procedure itself rather than any 'post procedure' issues."]
    pub r#complication: Vec<super::super::types::CodeableReference>,
    #[doc = "If the procedure required specific follow up - e.g. removal of sutures. The follow up may be represented as a simple note or could potentially be more complex, in which case the CarePlan resource can be used."]
    pub r#follow_up: Vec<super::super::types::CodeableConcept>,
    #[doc = "Any other notes and comments about the procedure."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "A device that is implanted, removed or otherwise manipulated (calibration, battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a focal portion of the Procedure."]
    pub r#focal_device: Vec<ProcedureFocalDevice>,
    #[doc = "Identifies medications, devices and any other substance used as part of the procedure."]
    pub r#used: Vec<super::super::types::CodeableReference>,
    #[doc = "Other resources from the patient record that may be relevant to the procedure.  The information from these resources was either used to create the instance or is provided to help with its interpretation. This extension should not be used if more specific inline elements or extensions are available."]
    pub r#supporting_info: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for Procedure {
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
            r#instantiates_canonical: Default::default(),
            r#instantiates_uri: Default::default(),
            r#based_on: Default::default(),
            r#part_of: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#category: Default::default(),
            r#code: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#focus: Default::default(),
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#recorded: Default::default(),
            r#recorder: Default::default(),
            r#reported: Default::default(),
            r#performer: Default::default(),
            r#location: Default::default(),
            r#reason: Default::default(),
            r#body_site: Default::default(),
            r#outcome: Default::default(),
            r#report: Default::default(),
            r#complication: Default::default(),
            r#follow_up: Default::default(),
            r#note: Default::default(),
            r#focal_device: Default::default(),
            r#used: Default::default(),
            r#supporting_info: Default::default(),
        }
    }
}
