// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The period, timing or frequency upon which the described activity is to occur."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CarePlanActivityDetailScheduled {
    Timing(Box<super::super::types::Timing>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Identifies the food, drug or other product to be consumed or supplied in the activity."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CarePlanActivityDetailProduct {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "A simple summary of a planned activity suitable for a general care plan system (e.g. form driven) that doesn't know about specific resources such as procedure etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct CarePlanActivityDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A description of the kind of resource the in-line definition of a care plan activity is representing.  The CarePlan.activity.detail is an in-line definition when a resource is not referenced using CarePlan.activity.reference.  For example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest."]
    pub r#kind: Option<super::super::types::Code>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan activity."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan activity."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Detailed description of the type of planned activity; e.g. what lab test, what procedure, what kind of encounter."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Provides the rationale that drove the inclusion of this particular activity as part of the plan or the reason why the activity was prohibited."]
    pub r#reason_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates another resource, such as the health condition(s), whose existence justifies this request and drove the inclusion of this particular activity as part of the plan."]
    pub r#reason_reference: Vec<super::super::types::Reference>,
    #[doc = "Internal reference that identifies the goals that this activity is intended to contribute towards meeting."]
    pub r#goal: Vec<super::super::types::Reference>,
    #[doc = "Identifies what progress is being made for the specific activity."]
    pub r#status: super::super::types::Code,
    #[doc = "Provides reason why the activity isn't yet started, is on hold, was cancelled, etc."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If true, indicates that the described activity is one that must NOT be engaged in when following the plan.  If false, or missing, indicates that the described activity is one that should be engaged in when following the plan."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "The period, timing or frequency upon which the described activity is to occur."]
    pub r#scheduled: Option<CarePlanActivityDetailScheduled>,
    #[doc = "Identifies the facility where the activity will occur; e.g. home, hospital, specific clinic, etc."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies who's expected to be involved in the activity."]
    pub r#performer: Vec<super::super::types::Reference>,
    #[doc = "Identifies the food, drug or other product to be consumed or supplied in the activity."]
    pub r#product: Option<CarePlanActivityDetailProduct>,
    #[doc = "Identifies the quantity expected to be consumed in a given day."]
    pub r#daily_amount: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the quantity expected to be supplied, administered or consumed by the subject."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "This provides a textual description of constraints on the intended activity occurrence, including relation to other activities.  It may also include objectives, pre-conditions and end-conditions.  Finally, it may convey specifics about the activity such as body site, method, route, etc."]
    pub r#description: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for CarePlanActivityDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#kind: Default::default(),
            r#instantiates_canonical: Default::default(),
            r#instantiates_uri: Default::default(),
            r#code: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#goal: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status_reason: Default::default(),
            r#do_not_perform: Default::default(),
            r#scheduled: Default::default(),
            r#location: Default::default(),
            r#performer: Default::default(),
            r#product: Default::default(),
            r#daily_amount: Default::default(),
            r#quantity: Default::default(),
            r#description: Default::default(),
        }
    }
}
#[doc = "Identifies a planned action to occur as part of the plan.  For example, a medication to be used, lab tests to perform, self-monitoring, education, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct CarePlanActivity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies the outcome at the point when the status of the activity is assessed.  For example, the outcome of an education activity could be patient understands (or not)."]
    pub r#outcome_codeable_concept: Vec<super::super::types::CodeableConcept>,
    #[doc = "Details of the outcome or action resulting from the activity.  The reference to an \"event\" resource, such as Procedure or Encounter or Observation, is the result/outcome of the activity itself.  The activity can be conveyed using CarePlan.activity.detail OR using the CarePlan.activity.reference (a reference to a “request” resource)."]
    pub r#outcome_reference: Vec<super::super::types::Reference>,
    #[doc = "Notes about the adherence/status/progress of the activity."]
    pub r#progress: Vec<super::super::types::Annotation>,
    #[doc = "The details of the proposed activity represented in a specific resource."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
    #[doc = "A simple summary of a planned activity suitable for a general care plan system (e.g. form driven) that doesn't know about specific resources such as procedure etc."]
    pub r#detail: Option<CarePlanActivityDetail>,
}
#[allow(clippy::derivable_impls)]
impl Default for CarePlanActivity {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#outcome_codeable_concept: Default::default(),
            r#outcome_reference: Default::default(),
            r#progress: Default::default(),
            r#reference: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions."]
#[derive(Debug, Clone, PartialEq)]
pub struct CarePlan {
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
    #[doc = "Business identifiers assigned to this care plan by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A care plan that is fulfilled in whole or in part by this care plan."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "Completed or terminated care plan whose function is taken by this new care plan."]
    pub r#replaces: Vec<super::super::types::Reference>,
    #[doc = "A larger care plan of which this particular care plan is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "Indicates whether the plan is currently being acted upon, represents future intentions or is now a historical record."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the care plan and where the care plan fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "Identifies what \"kind\" of plan this is to support differentiation between multiple co-existing plans; e.g. \"Home health\", \"psychiatric\", \"asthma\", \"disease management\", \"wellness plan\", etc."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Human-friendly name for the care plan."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A description of the scope and nature of the plan."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Identifies the patient or group whose intended care is described by the plan."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this CarePlan was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the plan did (or is intended to) come into effect and end."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Represents when this particular CarePlan record was created in the system, which is often a system-generated date."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "When populated, the author is responsible for the care plan.  The care plan is attributed to the author."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the individual(s) or organization who provided the contents of the care plan."]
    pub r#contributor: Vec<super::super::types::Reference>,
    #[doc = "Identifies all people and organizations who are expected to be involved in the care envisioned by this plan."]
    pub r#care_team: Vec<super::super::types::Reference>,
    #[doc = "Identifies the conditions/problems/concerns/diagnoses/etc. whose management and/or mitigation are handled by this plan."]
    pub r#addresses: Vec<super::super::types::Reference>,
    #[doc = "Identifies portions of the patient's record that specifically influenced the formation of the plan.  These might include comorbidities, recent procedures, limitations, recent assessments, etc."]
    pub r#supporting_info: Vec<super::super::types::Reference>,
    #[doc = "Describes the intended objective(s) of carrying out the care plan."]
    pub r#goal: Vec<super::super::types::Reference>,
    #[doc = "Identifies a planned action to occur as part of the plan.  For example, a medication to be used, lab tests to perform, self-monitoring, education, etc."]
    pub r#activity: Vec<CarePlanActivity>,
    #[doc = "General notes about the care plan not covered elsewhere."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for CarePlan {
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
            r#replaces: Default::default(),
            r#part_of: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#intent: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Default::default(),
            r#title: Default::default(),
            r#description: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#period: Default::default(),
            r#created: Default::default(),
            r#author: Default::default(),
            r#contributor: Default::default(),
            r#care_team: Default::default(),
            r#addresses: Default::default(),
            r#supporting_info: Default::default(),
            r#goal: Default::default(),
            r#activity: Default::default(),
            r#note: Default::default(),
        }
    }
}
