// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Estimated or actual date, date-time, period, or age when the procedure was performed.  Allows a period to support complex procedures that span more than one date, and also allows for the length of the procedure to be captured."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ProcedurePerformed {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "Limited to \"real\" people rather than equipment."]
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedurePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Distinguishes the type of involvement of the performer in the procedure. For example, surgeon, anaesthetist, endoscopist."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The practitioner who was involved in the procedure."]
    pub r#actor: Box<super::super::types::Reference>,
    #[doc = "The organization the device or practitioner was acting on behalf of."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
}
impl Default for ProcedurePerformer {
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
            r#on_behalf_of: Default::default(),
        }
    }
}
#[doc = "A device that is implanted, removed or otherwise manipulated (calibration, battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a focal portion of the Procedure."]
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureFocalDevice {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of change that happened to the device during the procedure."]
    pub r#action: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device that was manipulated (changed) during the procedure."]
    pub r#manipulated: Box<super::super::types::Reference>,
}
impl Default for ProcedureFocalDevice {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#action: Default::default(),
            r#manipulated: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy."]
#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
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
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific procedure that is performed. Use text if the exact nature of the procedure cannot be coded (e.g. \"Laparoscopic Appendectomy\")."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person, animal or group on which the procedure was performed."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this Procedure was created or performed or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Estimated or actual date, date-time, period, or age when the procedure was performed.  Allows a period to support complex procedures that span more than one date, and also allows for the length of the procedure to be captured."]
    pub r#performed: Option<ProcedurePerformed>,
    #[doc = "Individual who recorded the record and takes responsibility for its content."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Individual who is making the procedure statement."]
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    #[doc = "Limited to \"real\" people rather than equipment."]
    pub r#performer: Vec<ProcedurePerformer>,
    #[doc = "The location where the procedure actually happened.  E.g. a newborn at home, a tracheostomy at a restaurant."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The coded reason why the procedure was performed. This may be a coded entity of some type, or may simply be present as text."]
    pub r#reason_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "The justification of why the procedure was performed."]
    pub r#reason_reference: Vec<super::super::types::Reference>,
    #[doc = "Detailed and structured anatomical location information. Multiple locations are allowed - e.g. multiple punch biopsies of a lesion."]
    pub r#body_site: Vec<super::super::types::CodeableConcept>,
    #[doc = "The outcome of the procedure - did it resolve the reasons for the procedure being performed?"]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This could be a histology result, pathology report, surgical report, etc."]
    pub r#report: Vec<super::super::types::Reference>,
    #[doc = "Any complications that occurred during the procedure, or in the immediate post-performance period. These are generally tracked separately from the notes, which will typically describe the procedure itself rather than any 'post procedure' issues."]
    pub r#complication: Vec<super::super::types::CodeableConcept>,
    #[doc = "Any complications that occurred during the procedure, or in the immediate post-performance period."]
    pub r#complication_detail: Vec<super::super::types::Reference>,
    #[doc = "If the procedure required specific follow up - e.g. removal of sutures. The follow up may be represented as a simple note or could potentially be more complex, in which case the CarePlan resource can be used."]
    pub r#follow_up: Vec<super::super::types::CodeableConcept>,
    #[doc = "Any other notes and comments about the procedure."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "A device that is implanted, removed or otherwise manipulated (calibration, battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a focal portion of the Procedure."]
    pub r#focal_device: Vec<ProcedureFocalDevice>,
    #[doc = "Identifies medications, devices and any other substance used as part of the procedure."]
    pub r#used_reference: Vec<super::super::types::Reference>,
    #[doc = "Identifies coded items that were used as part of the procedure."]
    pub r#used_code: Vec<super::super::types::CodeableConcept>,
}
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
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status_reason: Default::default(),
            r#category: Default::default(),
            r#code: Default::default(),
            r#subject: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#encounter: Default::default(),
            r#performed: Default::default(),
            r#recorder: Default::default(),
            r#asserter: Default::default(),
            r#performer: Default::default(),
            r#location: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#body_site: Default::default(),
            r#outcome: Default::default(),
            r#report: Default::default(),
            r#complication: Default::default(),
            r#complication_detail: Default::default(),
            r#follow_up: Default::default(),
            r#note: Default::default(),
            r#focal_device: Default::default(),
            r#used_reference: Default::default(),
            r#used_code: Default::default(),
        }
    }
}
