// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationStatementMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The interval of time during which it is being asserted that the patient is/was/will be taking the medication (or was not taking, when the MedicationStatement.taken element is No)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationStatementEffective {
    DateTime(super::super::types::DateTime),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "A record of a medication that is being consumed by a patient.   A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be taking the medication in the future.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay.   The medication information may come from sources such as the patient's memory, from a prescription bottle,  or from a list of medications the patient, clinician or other party maintains. \n\nThe primary difference between a medication statement and a medication administration is that the medication administration has complete administration information and is based on actual administration information from the person who administered the medication.  A medication statement is often, if not always, less specific.  There is no required date/time when the medication was administered, in fact we only know that a source has reported the patient is taking this medication, where details such as time, quantity, or rate or even medication product may be incomplete or missing or less precise.  As stated earlier, the medication statement information may come from the patient's memory, from a prescription bottle or from a list of medications the patient, clinician or other party maintains.  Medication administration is more formal and is not missing detailed information."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationStatement {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
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
    #[doc = "Identifiers associated with this Medication Statement that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this event."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "A code representing the patient or other source's judgment about the state of the medication used that this statement is about.  Generally, this will be active or completed."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the MedicationStatement."]
    pub r#status_reason: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates where the medication is expected to be consumed or administered."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
    pub r#medication: MedicationStatementMedication,
    #[doc = "The person, animal or group who is/was taking the medication."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The encounter or episode of care that establishes the context for this MedicationStatement."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "The interval of time during which it is being asserted that the patient is/was/will be taking the medication (or was not taking, when the MedicationStatement.taken element is No)."]
    pub r#effective: Option<MedicationStatementEffective>,
    #[doc = "The date when the medication statement was asserted by the information source."]
    pub r#date_asserted: Option<super::super::types::DateTime>,
    #[doc = "The person or organization that provided the information about the taking of this medication. Note: Use derivedFrom when a MedicationStatement is derived from other resources, e.g. Claim or MedicationRequest."]
    pub r#information_source: Option<Box<super::super::types::Reference>>,
    #[doc = "Allows linking the MedicationStatement to the underlying MedicationRequest, or to other information that supports or is used to derive the MedicationStatement."]
    pub r#derived_from: Vec<super::super::types::Reference>,
    #[doc = "A reason for why the medication is being/was taken."]
    pub r#reason_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Condition or observation that supports why the medication is being/was taken."]
    pub r#reason_reference: Vec<super::super::types::Reference>,
    #[doc = "Provides extra information about the medication statement that is not conveyed by the other attributes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Indicates how the medication is/was or should be taken by the patient."]
    pub r#dosage: Vec<super::super::types::Dosage>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationStatement {
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
            r#medication: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#context: Default::default(),
            r#effective: Default::default(),
            r#date_asserted: Default::default(),
            r#information_source: Default::default(),
            r#derived_from: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#note: Default::default(),
            r#dosage: Default::default(),
        }
    }
}
