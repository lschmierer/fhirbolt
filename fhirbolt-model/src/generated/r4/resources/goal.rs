// Generated on 2023-04-18 by fhirbolt-codegen v0.2.0
#[doc = "The date or event after which the goal should begin being pursued."]
#[derive(Debug, Clone, PartialEq)]
pub enum GoalStart {
    Date(Box<super::super::types::Date>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for GoalStart {
    fn default() -> GoalStart {
        GoalStart::Invalid
    }
}
#[doc = "The target value of the focus to be achieved to signify the fulfillment of the goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any focus value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any focus value at or above the low value."]
#[derive(Debug, Clone, PartialEq)]
pub enum GoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Ratio(Box<super::super::types::Ratio>),
    Invalid,
}
impl Default for GoalTargetDetail {
    fn default() -> GoalTargetDetail {
        GoalTargetDetail::Invalid
    }
}
#[doc = "Indicates either the date or the duration after start by which the goal should be met."]
#[derive(Debug, Clone, PartialEq)]
pub enum GoalTargetDue {
    Date(Box<super::super::types::Date>),
    Duration(Box<super::super::types::Duration>),
    Invalid,
}
impl Default for GoalTargetDue {
    fn default() -> GoalTargetDue {
        GoalTargetDue::Invalid
    }
}
#[doc = "Indicates what should be done by when."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GoalTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The parameter whose value is being tracked, e.g. body weight, blood pressure, or hemoglobin A1c level."]
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target value of the focus to be achieved to signify the fulfillment of the goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any focus value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any focus value at or above the low value."]
    pub r#detail: Option<GoalTargetDetail>,
    #[doc = "Indicates either the date or the duration after start by which the goal should be met."]
    pub r#due: Option<GoalTargetDue>,
}
#[doc = "Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Goal {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business identifiers assigned to this goal by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The state of the goal throughout its lifecycle."]
    pub r#lifecycle_status: super::super::types::Code,
    #[doc = "Describes the progression, or lack thereof, towards the goal against the target."]
    pub r#achievement_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates a category the goal falls within."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the mutually agreed level of importance associated with reaching/sustaining the goal."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Human-readable and/or coded description of a specific desired objective of care, such as \"control blood pressure\" or \"negotiate an obstacle course\" or \"dance with child at wedding\"."]
    pub r#description: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the patient, group or organization for whom the goal is being established."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The date or event after which the goal should begin being pursued."]
    pub r#start: Option<GoalStart>,
    #[doc = "Indicates what should be done by when."]
    pub r#target: Vec<GoalTarget>,
    #[doc = "Identifies when the current status.  I.e. When initially created, when achieved, when cancelled, etc."]
    pub r#status_date: Option<super::super::types::Date>,
    #[doc = "Captures the reason for the current status."]
    pub r#status_reason: Option<super::super::types::String>,
    #[doc = "Indicates whose goal this is - patient goal, practitioner goal, etc."]
    pub r#expressed_by: Option<Box<super::super::types::Reference>>,
    #[doc = "The identified conditions and other health record elements that are intended to be addressed by the goal."]
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    #[doc = "Any comments related to the goal."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Identifies the change (or lack of change) at the point when the status of the goal is assessed."]
    pub r#outcome_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Details of what's changed (or not changed)."]
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
}
