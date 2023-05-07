// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "True if the prescriber allows a different drug to be dispensed from what was prescribed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationRequestSubstitutionAllowed {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Indicates the quantity or duration for the first dispense of the medication."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequestDispenseRequestInitialFill {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The amount or quantity to provide as part of the first dispense."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The length of time that the first dispense is expected to last."]
    pub r#duration: Option<Box<super::super::types::Duration>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationRequestDispenseRequestInitialFill {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#quantity: Default::default(),
            r#duration: Default::default(),
        }
    }
}
#[doc = "Indicates the specific details for the dispense or medication supply part of a medication request (also known as a Medication Prescription or Medication Order).  Note that this information is not always sent with the order.  There may be in some settings (e.g. hospitals) institutional or system support for completing the dispense details in the pharmacy department."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequestDispenseRequest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates the quantity or duration for the first dispense of the medication."]
    pub r#initial_fill: Option<MedicationRequestDispenseRequestInitialFill>,
    #[doc = "The minimum period of time that must occur between dispenses of the medication."]
    pub r#dispense_interval: Option<Box<super::super::types::Duration>>,
    #[doc = "This indicates the validity period of a prescription (stale dating the Prescription)."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "An integer indicating the number of times, in addition to the original dispense, (aka refills or repeats) that the patient can receive the prescribed medication. Usage Notes: This integer does not include the original order dispense. This means that if an order indicates dispense 30 tablets plus \"3 repeats\", then the order can be dispensed a total of 4 times and the patient can receive a total of 120 tablets.  A prescriber may explicitly say that zero refills are permitted after the initial dispense."]
    pub r#number_of_repeats_allowed: Option<super::super::types::UnsignedInt>,
    #[doc = "The amount that is to be dispensed for one fill."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the period time over which the supplied product is expected to be used, or the length of time the dispense is expected to last."]
    pub r#expected_supply_duration: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates the intended performing Organization that will dispense the medication as specified by the prescriber."]
    pub r#dispenser: Option<Box<super::super::types::Reference>>,
    #[doc = "Provides additional information to the dispenser, for example, counselling to be provided to the patient."]
    pub r#dispenser_instruction: Vec<super::super::types::Annotation>,
    #[doc = "Provides information about the type of adherence packaging to be supplied for the medication dispense."]
    pub r#dose_administration_aid: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationRequestDispenseRequest {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#initial_fill: Default::default(),
            r#dispense_interval: Default::default(),
            r#validity_period: Default::default(),
            r#number_of_repeats_allowed: Default::default(),
            r#quantity: Default::default(),
            r#expected_supply_duration: Default::default(),
            r#dispenser: Default::default(),
            r#dispenser_instruction: Default::default(),
            r#dose_administration_aid: Default::default(),
        }
    }
}
#[doc = "Indicates whether or not substitution can or should be part of the dispense. In some cases, substitution must happen, in other cases substitution must not happen. This block explains the prescriber's intent. If nothing is specified substitution may be done."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequestSubstitution {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "True if the prescriber allows a different drug to be dispensed from what was prescribed."]
    pub r#allowed: MedicationRequestSubstitutionAllowed,
    #[doc = "Indicates the reason for the substitution, or why substitution must or must not be performed."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationRequestSubstitution {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#allowed: Default::default(),
            r#reason: Default::default(),
        }
    }
}
#[doc = "An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called \"MedicationRequest\" rather than \"MedicationPrescription\" or \"MedicationOrder\" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequest {
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
    #[doc = "Identifiers associated with this medication request that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A plan or request that is fulfilled in whole or in part by this medication request."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "Reference to an order/prescription that is being replaced by this MedicationRequest."]
    pub r#prior_prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "A shared identifier common to multiple independent Request instances that were activated/authorized more or less simultaneously by a single author.  The presence of the same identifier on each request ties those requests together and may have business ramifications in terms of reporting of results, billing, etc.  E.g. a requisition number shared by a set of lab tests ordered together, or a prescription number shared by all meds ordered at one time."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "A code specifying the current state of the order.  Generally, this will be active or completed state."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the MedicationRequest."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date (and perhaps time) when the status was changed."]
    pub r#status_changed: Option<super::super::types::DateTime>,
    #[doc = "Whether the request is a proposal, plan, or an original order."]
    pub r#intent: super::super::types::Code,
    #[doc = "An arbitrary categorization or grouping of the medication request.  It could be used for indicating where meds are intended to be administered, eg. in an inpatient setting or in a patient's home, or a legal category of the medication."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates how quickly the Medication Request should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "If true, indicates that the provider is asking for the patient to either stop taking or to not start taking the specified medication. For example, the patient is taking an existing medication and the provider is changing their medication. They want to create two seperate requests: one to stop using the current medication and another to start the new medication."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "Identifies the medication being requested. This is a link to a resource that represents the medication which may be the details of the medication or simply an attribute carrying a code that identifies the medication from a known list of medications."]
    pub r#medication: Box<super::super::types::CodeableReference>,
    #[doc = "The individual or group for whom the medication has been requested."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The person or organization who provided the information about this request, if the source is someone other than the requestor.  This is often used when the MedicationRequest is reported by another person."]
    pub r#information_source: Vec<super::super::types::Reference>,
    #[doc = "The Encounter during which this \\[x\\] was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Information to support fulfilling (i.e. dispensing or administering) of the medication, for example, patient height and weight, a MedicationStatement for the patient)."]
    pub r#supporting_information: Vec<super::super::types::Reference>,
    #[doc = "The date (and perhaps time) when the prescription was initially written or authored on."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The individual, organization, or device that initiated the request and has responsibility for its activation."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates if this record was captured as a secondary 'reported' record rather than as an original primary source-of-truth record.  It may also indicate the source of the report."]
    pub r#reported: Option<super::super::types::Boolean>,
    #[doc = "Indicates the type of performer of the administration of the medication."]
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specified desired performer of the medication treatment (e.g. the performer of the medication administration).  For devices, this is the device that is intended to perform the administration of the medication.  An IV Pump would be an example of a device that is performing the administration.  Both the IV Pump and the practitioner that set the rate or bolus on the pump can be listed as performers."]
    pub r#performer: Vec<super::super::types::Reference>,
    #[doc = "The intended type of device that is to be used for the administration of the medication (for example, PCA Pump)."]
    pub r#device: Vec<super::super::types::CodeableReference>,
    #[doc = "The person who entered the order on behalf of another individual for example in the case of a verbal or a telephone order."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason or the indication for ordering or not ordering the medication."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "The description of the overall pattern of the administration of the medication to the patient."]
    pub r#course_of_therapy_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be required for delivering the requested service."]
    pub r#insurance: Vec<super::super::types::Reference>,
    #[doc = "Extra information about the prescription that could not be conveyed by the other attributes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The full representation of the dose of the medication included in all dosage instructions.  To be used when multiple dosage instructions are included to represent complex dosing such as increasing or tapering doses."]
    pub r#rendered_dosage_instruction: Option<super::super::types::Markdown>,
    #[doc = "The period over which the medication is to be taken.  Where there are multiple dosageInstruction lines (for example, tapering doses), this is the earliest date and the latest end date of the dosageInstructions."]
    pub r#effective_dose_period: Option<Box<super::super::types::Period>>,
    #[doc = "Specific instructions for how the medication is to be used by the patient."]
    pub r#dosage_instruction: Vec<super::super::types::Dosage>,
    #[doc = "Indicates the specific details for the dispense or medication supply part of a medication request (also known as a Medication Prescription or Medication Order).  Note that this information is not always sent with the order.  There may be in some settings (e.g. hospitals) institutional or system support for completing the dispense details in the pharmacy department."]
    pub r#dispense_request: Option<MedicationRequestDispenseRequest>,
    #[doc = "Indicates whether or not substitution can or should be part of the dispense. In some cases, substitution must happen, in other cases substitution must not happen. This block explains the prescriber's intent. If nothing is specified substitution may be done."]
    pub r#substitution: Option<MedicationRequestSubstitution>,
    #[doc = "Links to Provenance records for past versions of this resource or fulfilling request or event resources that identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the resource."]
    pub r#event_history: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationRequest {
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
            r#based_on: Default::default(),
            r#prior_prescription: Default::default(),
            r#group_identifier: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#status_changed: Default::default(),
            r#intent: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Default::default(),
            r#priority: Default::default(),
            r#do_not_perform: Default::default(),
            r#medication: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#information_source: Default::default(),
            r#encounter: Default::default(),
            r#supporting_information: Default::default(),
            r#authored_on: Default::default(),
            r#requester: Default::default(),
            r#reported: Default::default(),
            r#performer_type: Default::default(),
            r#performer: Default::default(),
            r#device: Default::default(),
            r#recorder: Default::default(),
            r#reason: Default::default(),
            r#course_of_therapy_type: Default::default(),
            r#insurance: Default::default(),
            r#note: Default::default(),
            r#rendered_dosage_instruction: Default::default(),
            r#effective_dose_period: Default::default(),
            r#dosage_instruction: Default::default(),
            r#dispense_request: Default::default(),
            r#substitution: Default::default(),
            r#event_history: Default::default(),
        }
    }
}
