// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Indicates the reason why a dispense was not performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationDispenseStatusReason {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationDispenseMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Indicates who or what performed the event."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationDispensePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Distinguishes the type of performer in the dispense.  For example, date enterer, packager, final checker."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who performed the action.  It should be assumed that the actor is the dispenser of the medication."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl Default for MedicationDispensePerformer {
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
#[doc = "Indicates whether or not substitution was made as part of the dispense.  In some cases, substitution will be expected but does not happen, in other cases substitution is not expected but does happen.  This block explains what substitution did or did not happen and why.  If nothing is specified, substitution was not done."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationDispenseSubstitution {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "True if the dispenser dispensed a different drug or product from what was prescribed."]
    pub r#was_substituted: super::super::types::Boolean,
    #[doc = "A code signifying whether a different drug was dispensed from what was prescribed."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the reason for the substitution (or lack of substitution) from what was prescribed."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person or organization that has primary responsibility for the substitution."]
    pub r#responsible_party: Vec<Box<super::super::types::Reference>>,
}
impl Default for MedicationDispenseSubstitution {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#was_substituted: {
                let mut default: super::super::types::Boolean = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#type: Default::default(),
            r#reason: Default::default(),
            r#responsible_party: Default::default(),
        }
    }
}
#[doc = "Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationDispense {
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
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers associated with this Medication Dispense that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The procedure that trigger the dispense."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code specifying the state of the set of dispense events."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the reason why a dispense was not performed."]
    pub r#status_reason: Option<MedicationDispenseStatusReason>,
    #[doc = "Indicates the type of medication dispense (for example, where the medication is expected to be consumed or administered (i.e. inpatient or outpatient))."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
    pub r#medication: MedicationDispenseMedication,
    #[doc = "A link to a resource representing the person or the group to whom the medication will be given."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter or episode of care that establishes the context for this event."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional information that supports the medication being dispensed."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates who or what performed the event."]
    pub r#performer: Vec<MedicationDispensePerformer>,
    #[doc = "The principal physical location where the dispense was performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the medication order that is being dispensed against."]
    pub r#authorizing_prescription: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates the type of dispensing event that is performed. For example, Trial Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The amount of medication that has been dispensed. Includes unit of measure."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The amount of medication expressed as a timing amount."]
    pub r#days_supply: Option<Box<super::super::types::Quantity>>,
    #[doc = "The time when the dispensed product was packaged and reviewed."]
    pub r#when_prepared: Option<super::super::types::DateTime>,
    #[doc = "The time the dispensed product was provided to the patient or their representative."]
    pub r#when_handed_over: Option<super::super::types::DateTime>,
    #[doc = "Identification of the facility/location where the medication was shipped to, as part of the dispense event."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the person who picked up the medication.  This will usually be a patient or their caregiver, but some cases exist where it can be a healthcare professional."]
    pub r#receiver: Vec<Box<super::super::types::Reference>>,
    #[doc = "Extra information about the dispense that could not be conveyed in the other attributes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Indicates how the medication is to be used by the patient."]
    pub r#dosage_instruction: Vec<Box<super::super::types::Dosage>>,
    #[doc = "Indicates whether or not substitution was made as part of the dispense.  In some cases, substitution will be expected but does not happen, in other cases substitution is not expected but does happen.  This block explains what substitution did or did not happen and why.  If nothing is specified, substitution was not done."]
    pub r#substitution: Option<MedicationDispenseSubstitution>,
    #[doc = "Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. drug-drug interaction, duplicate therapy, dosage alert etc."]
    pub r#detected_issue: Vec<Box<super::super::types::Reference>>,
    #[doc = "A summary of the events of interest that have occurred, such as when the dispense was verified."]
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
}
impl Default for MedicationDispense {
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
            r#part_of: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status_reason: Default::default(),
            r#category: Default::default(),
            r#medication: Default::default(),
            r#subject: Default::default(),
            r#context: Default::default(),
            r#supporting_information: Default::default(),
            r#performer: Default::default(),
            r#location: Default::default(),
            r#authorizing_prescription: Default::default(),
            r#type: Default::default(),
            r#quantity: Default::default(),
            r#days_supply: Default::default(),
            r#when_prepared: Default::default(),
            r#when_handed_over: Default::default(),
            r#destination: Default::default(),
            r#receiver: Default::default(),
            r#note: Default::default(),
            r#dosage_instruction: Default::default(),
            r#substitution: Default::default(),
            r#detected_issue: Default::default(),
            r#event_history: Default::default(),
        }
    }
}
