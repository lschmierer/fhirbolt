// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
#[doc = "The time or time-period the observed value is asserted as being true. For biological subjects - e.g. human patients - this is usually called the \"physiologically relevant time\". This is usually either the time of the procedure or of specimen collection, but very often the source of the date/time is not known, only the date/time itself."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ObservationEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Instant(Box<super::super::types::Instant>),
    #[default]
    Invalid,
}
#[doc = "The information determined as a result of making the observation, if the information has a simple value."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ObservationValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "The information determined as a result of making the observation, if the information has a simple value."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ObservationComponentValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Guidance on how to interpret the value by comparison to a normal or recommended range.  Multiple reference ranges are interpreted as an \"OR\".   In other words, to represent two distinct target populations, two `referenceRange` elements would be used."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationReferenceRange {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The value of the low bound of the reference range.  The low bound of the reference range endpoint is inclusive of the value (e.g.  reference range is >=5 - <=9). If the low bound is omitted,  it is assumed to be meaningless (e.g. reference range is <=2.3)."]
    pub r#low: Option<Box<super::super::types::Quantity>>,
    #[doc = "The value of the high bound of the reference range.  The high bound of the reference range endpoint is inclusive of the value (e.g.  reference range is >=5 - <=9). If the high bound is omitted,  it is assumed to be meaningless (e.g. reference range is >= 2.3)."]
    pub r#high: Option<Box<super::super::types::Quantity>>,
    #[doc = "Codes to indicate the what part of the targeted reference population it applies to. For example, the normal or therapeutic range."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Codes to indicate the target population this reference range applies to.  For example, a reference range may be based on the normal population or a particular sex or race.  Multiple `appliesTo`  are interpreted as an \"AND\" of the target populations.  For example, to represent a target population of African American females, both a code of female and a code for African American would be used."]
    pub r#applies_to: Vec<super::super::types::CodeableConcept>,
    #[doc = "The age at which this reference range is applicable. This is a neonatal age (e.g. number of weeks at term) if the meaning says so."]
    pub r#age: Option<Box<super::super::types::Range>>,
    #[doc = "Text based reference range in an observation which may be used when a quantitative range is not appropriate for an observation.  An example would be a reference value of \"Negative\" or a list or table of \"normals\"."]
    pub r#text: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ObservationReferenceRange {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#low: Default::default(),
            r#high: Default::default(),
            r#type: Default::default(),
            r#applies_to: Default::default(),
            r#age: Default::default(),
            r#text: Default::default(),
        }
    }
}
#[doc = "Some observations have multiple component observations.  These component observations are expressed as separate code value pairs that share the same attributes.  Examples include systolic and diastolic component observations for blood pressure measurement and multiple component observations for genetics observations."]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes what was observed. Sometimes this is called the observation \"code\"."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The information determined as a result of making the observation, if the information has a simple value."]
    pub r#value: Option<ObservationComponentValue>,
    #[doc = "Provides a reason why the expected value in the element Observation.component.value\\[x\\] is missing."]
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A categorical assessment of an observation value.  For example, high, low, normal."]
    pub r#interpretation: Vec<super::super::types::CodeableConcept>,
    #[doc = "Guidance on how to interpret the value by comparison to a normal or recommended range."]
    pub r#reference_range: Vec<ObservationReferenceRange>,
}
#[allow(clippy::derivable_impls)]
impl Default for ObservationComponent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
            r#data_absent_reason: Default::default(),
            r#interpretation: Default::default(),
            r#reference_range: Default::default(),
        }
    }
}
#[doc = "Measurements and simple assertions made about a patient, device or other subject.\n\nObservations are a key aspect of healthcare.  This resource is used to capture those that do not require more sophisticated mechanisms."]
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique identifier assigned to this observation."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this event.  For example, a MedicationRequest may require a patient to have laboratory test performed before  it is dispensed."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event of which this particular Observation is a component or step.  For example,  an observation as part of a procedure."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "The status of the result value."]
    pub r#status: super::super::types::Code,
    #[doc = "A code that classifies the general type of observation being made."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Describes what was observed. Sometimes this is called the observation \"name\"."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The patient, or group of patients, location, or device this observation is about and into whose record the observation is placed. If the actual focus of the observation is different from the subject (or a sample of, part, or region of the subject), the `focus` element or the `code` itself specifies the actual focus of the observation."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The actual focus of an observation when it is not the patient of record representing something or someone associated with the patient such as a spouse, parent, fetus, or donor. For example, fetus observations in a mother's record.  The focus of an observation could also be an existing condition,  an intervention, the subject's diet,  another observation of the subject,  or a body structure such as tumor or implanted device.   An example use case would be using the Observation resource to capture whether the mother is trained to change her child's tracheostomy tube. In this example, the child is the patient of record and the mother is the focus."]
    pub r#focus: Vec<super::super::types::Reference>,
    #[doc = "The healthcare event  (e.g. a patient and healthcare provider interaction) during which this observation is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The time or time-period the observed value is asserted as being true. For biological subjects - e.g. human patients - this is usually called the \"physiologically relevant time\". This is usually either the time of the procedure or of specimen collection, but very often the source of the date/time is not known, only the date/time itself."]
    pub r#effective: Option<ObservationEffective>,
    #[doc = "The date and time this version of the observation was made available to providers, typically after the results have been reviewed and verified."]
    pub r#issued: Option<super::super::types::Instant>,
    #[doc = "Who was responsible for asserting the observed value as \"true\"."]
    pub r#performer: Vec<super::super::types::Reference>,
    #[doc = "The information determined as a result of making the observation, if the information has a simple value."]
    pub r#value: Option<ObservationValue>,
    #[doc = "Provides a reason why the expected value in the element Observation.value\\[x\\] is missing."]
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A categorical assessment of an observation value.  For example, high, low, normal."]
    pub r#interpretation: Vec<super::super::types::CodeableConcept>,
    #[doc = "Comments about the observation or the results."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Indicates the site on the subject's body where the observation was made (i.e. the target site)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the mechanism used to perform the observation."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specimen that was used when this observation was made."]
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    #[doc = "The device used to generate the observation data."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Guidance on how to interpret the value by comparison to a normal or recommended range.  Multiple reference ranges are interpreted as an \"OR\".   In other words, to represent two distinct target populations, two `referenceRange` elements would be used."]
    pub r#reference_range: Vec<ObservationReferenceRange>,
    #[doc = "This observation is a group observation (e.g. a battery, a panel of tests, a set of vital sign measurements) that includes the target as a member of the group."]
    pub r#has_member: Vec<super::super::types::Reference>,
    #[doc = "The target resource that represents a measurement from which this observation value is derived. For example, a calculated anion gap or a fetal measurement based on an ultrasound image."]
    pub r#derived_from: Vec<super::super::types::Reference>,
    #[doc = "Some observations have multiple component observations.  These component observations are expressed as separate code value pairs that share the same attributes.  Examples include systolic and diastolic component observations for blood pressure measurement and multiple component observations for genetics observations."]
    pub r#component: Vec<ObservationComponent>,
}
#[allow(clippy::derivable_impls)]
impl Default for Observation {
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
            r#category: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#subject: Default::default(),
            r#focus: Default::default(),
            r#encounter: Default::default(),
            r#effective: Default::default(),
            r#issued: Default::default(),
            r#performer: Default::default(),
            r#value: Default::default(),
            r#data_absent_reason: Default::default(),
            r#interpretation: Default::default(),
            r#note: Default::default(),
            r#body_site: Default::default(),
            r#method: Default::default(),
            r#specimen: Default::default(),
            r#device: Default::default(),
            r#reference_range: Default::default(),
            r#has_member: Default::default(),
            r#derived_from: Default::default(),
            r#component: Default::default(),
        }
    }
}
