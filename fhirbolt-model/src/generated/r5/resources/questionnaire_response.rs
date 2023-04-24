// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The answer (or one of the answers) provided by the respondent to the question."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum QuestionnaireResponseItemAnswerValue {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Attachment(Box<super::super::types::Attachment>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The respondent's answer(s) to the question."]
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireResponseItemAnswer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The answer (or one of the answers) provided by the respondent to the question."]
    pub r#value: QuestionnaireResponseItemAnswerValue,
    #[doc = "Nested groups and/or questions found within this particular answer."]
    pub r#item: Vec<QuestionnaireResponseItem>,
}
impl Default for QuestionnaireResponseItemAnswer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "A group or question item from the original questionnaire for which answers are provided."]
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireResponseItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The item from the Questionnaire that corresponds to this item in the QuestionnaireResponse resource."]
    pub r#link_id: super::super::types::String,
    #[doc = "A reference to an [ElementDefinition](https://hl7.org/FHIR/elementdefinition.html)) that provides the details for the item."]
    pub r#definition: Option<super::super::types::Uri>,
    #[doc = "Text that is displayed above the contents of the group or as the text of the question being answered."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The respondent's answer(s) to the question."]
    pub r#answer: Vec<QuestionnaireResponseItemAnswer>,
    #[doc = "Sub-questions, sub-groups or display items nested beneath a group."]
    pub r#item: Vec<QuestionnaireResponseItem>,
}
impl Default for QuestionnaireResponseItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#definition: Default::default(),
            r#text: Default::default(),
            r#answer: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "A structured set of questions and their answers. The questions are ordered and grouped into coherent subsets, corresponding to the structure of the grouping of the questionnaire being responded to.\n\nTo support structured, hierarchical reporting of data gathered using digital forms and other questionnaires."]
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireResponse {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Business identifiers assigned to this questionnaire response by the performer and/or other systems.  These identifiers remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this questionnaire response.  For example, a ServiceRequest seeking an intake assessment or a decision support recommendation to assess for post-partum depression."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A procedure or observation that this questionnaire was performed as part of the execution of.  For example, the surgery a checklist was executed as part of."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "The Questionnaire that defines and organizes the questions for which answers are being provided."]
    pub r#questionnaire: super::super::types::Canonical,
    #[doc = "The current state of the questionnaire response."]
    pub r#status: super::super::types::Code,
    #[doc = "The subject of the questionnaire response.  This could be a patient, organization, practitioner, device, etc.  This is who/what the answers apply to, but is not necessarily the source of information."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The Encounter during which this questionnaire response was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date and/or time that this questionnaire response was last modified by the user - e.g. changing answers or revising status."]
    pub r#authored: Option<super::super::types::DateTime>,
    #[doc = "The individual or device that received the answers to the questions in the QuestionnaireResponse and recorded them in the system."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "The individual or device that answered the questions about the subject."]
    pub r#source: Option<Box<super::super::types::Reference>>,
    #[doc = "A group or question item from the original questionnaire for which answers are provided."]
    pub r#item: Vec<QuestionnaireResponseItem>,
}
impl Default for QuestionnaireResponse {
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
            r#questionnaire: {
                let mut default: super::super::types::Canonical = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#subject: Default::default(),
            r#encounter: Default::default(),
            r#authored: Default::default(),
            r#author: Default::default(),
            r#source: Default::default(),
            r#item: Default::default(),
        }
    }
}
