// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A specific date/time or interval of time during which the administration took place (or did not take place). For many administrations, such as swallowing a tablet the use of dateTime is more appropriate."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationAdministrationOccurence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "Identifies the speed with which the medication was or will be introduced into the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or 100 ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2 hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationAdministrationDosageRate {
    Ratio(Box<super::super::types::Ratio>),
    Quantity(Box<super::super::types::Quantity>),
    #[default]
    Invalid,
}
#[doc = "The performer of the medication treatment.  For devices this is the device that performed the administration of the medication.  An IV Pump would be an example of a device that is performing the administration. Both the IV Pump and the practitioner that set the rate or bolus on the pump can be listed as performers."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationAdministrationPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Distinguishes the type of involvement of the performer in the medication administration."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what performed the medication administration."]
    pub r#actor: Box<super::super::types::CodeableReference>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationAdministrationPerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Describes the medication dosage information details e.g. dose, rate, site, route, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationAdministrationDosage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Free text dosage can be used for cases where the dosage administered is too complex to code. When coded dosage is present, the free text dosage may still be present for display to humans.\r\rThe dosage instructions should reflect the dosage of the medication that was administered."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "A coded specification of the anatomic site where the medication first entered the body.  For example, \"left arm\"."]
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code specifying the route or physiological path of administration of a therapeutic agent into or onto the patient.  For example, topical, intravenous, etc."]
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A coded value indicating the method by which the medication is intended to be or was introduced into or on the body.  This attribute will most often NOT be populated.  It is most commonly used for injections.  For example, Slow Push, Deep IV."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The amount of the medication given at one administration event.   Use this value when the administration is essentially an instantaneous event such as a swallowing a tablet or giving an injection."]
    pub r#dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the speed with which the medication was or will be introduced into the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or 100 ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2 hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours."]
    pub r#rate: Option<MedicationAdministrationDosageRate>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationAdministrationDosage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#text: Default::default(),
            r#site: Default::default(),
            r#route: Default::default(),
            r#method: Default::default(),
            r#dose: Default::default(),
            r#rate: Default::default(),
        }
    }
}
#[doc = "Describes the event of a patient consuming or otherwise being administered a medication.  This may be as simple as swallowing a tablet or it may be a long running infusion. Related resources tie this event to the authorizing prescription, and the specific encounter between patient and health care practitioner. This event can also be used to record waste using a status of not-done and the appropriate statusReason."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationAdministration {
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
    #[doc = "Identifiers associated with this Medication Administration that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A plan that is fulfilled in whole or in part by this MedicationAdministration."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "Will generally be set to show that the administration has been completed.  For some long running administrations such as infusions, it is possible for an administration to be started but not completed or it may be paused while some other process is under way."]
    pub r#status: super::super::types::Code,
    #[doc = "A code indicating why the administration was not performed."]
    pub r#status_reason: Vec<super::super::types::CodeableConcept>,
    #[doc = "The type of medication administration (for example, drug classification like ATC, where meds would be administered, legal category of the medication)."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Identifies the medication that was administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
    pub r#medication: Box<super::super::types::CodeableReference>,
    #[doc = "The person or animal or group receiving the medication."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The visit, admission, or other contact between patient and health care provider during which the medication administration was performed."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional information (for example, patient height and weight) that supports the administration of the medication.  This attribute can be used to provide documentation of specific characteristics of the patient present at the time of administration.  For example, if the dose says \"give \"x\" if the heartrate exceeds \"y\"\", then the heart rate can be included using this attribute."]
    pub r#supporting_information: Vec<super::super::types::Reference>,
    #[doc = "A specific date/time or interval of time during which the administration took place (or did not take place). For many administrations, such as swallowing a tablet the use of dateTime is more appropriate."]
    pub r#occurence: MedicationAdministrationOccurence,
    #[doc = "The date the occurrence of the  MedicationAdministration was first captured in the record - potentially significantly after the occurrence of the event."]
    pub r#recorded: Option<super::super::types::DateTime>,
    #[doc = "An indication that the full dose was not administered."]
    pub r#is_sub_potent: Option<super::super::types::Boolean>,
    #[doc = "The reason or reasons why the full dose was not administered."]
    pub r#sub_potent_reason: Vec<super::super::types::CodeableConcept>,
    #[doc = "The performer of the medication treatment.  For devices this is the device that performed the administration of the medication.  An IV Pump would be an example of a device that is performing the administration. Both the IV Pump and the practitioner that set the rate or bolus on the pump can be listed as performers."]
    pub r#performer: Vec<MedicationAdministrationPerformer>,
    #[doc = "A code, Condition or observation that supports why the medication was administered."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "The original request, instruction or authority to perform the administration."]
    pub r#request: Option<Box<super::super::types::Reference>>,
    #[doc = "The device that is to be used for the administration of the medication (for example, PCA Pump)."]
    pub r#device: Vec<super::super::types::CodeableReference>,
    #[doc = "Extra information about the medication administration that is not conveyed by the other attributes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Describes the medication dosage information details e.g. dose, rate, site, route, etc."]
    pub r#dosage: Option<MedicationAdministrationDosage>,
    #[doc = "A summary of the events of interest that have occurred, such as when the administration was verified."]
    pub r#event_history: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationAdministration {
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
            r#part_of: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#category: Default::default(),
            r#medication: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#supporting_information: Default::default(),
            r#occurence: Default::default(),
            r#recorded: Default::default(),
            r#is_sub_potent: Default::default(),
            r#sub_potent_reason: Default::default(),
            r#performer: Default::default(),
            r#reason: Default::default(),
            r#request: Default::default(),
            r#device: Default::default(),
            r#note: Default::default(),
            r#dosage: Default::default(),
            r#event_history: Default::default(),
        }
    }
}
