// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum QuestionnaireVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "A value that the referenced question is tested using the specified operator in order for the item to be enabled.  If there are multiple answers, a match on any of the answers suffices.  If different behavior is desired (all must match, at least 2 must match, etc.), consider using the enableWhenExpression extension."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum QuestionnaireItemEnableWhenAnswer {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "A potential answer that's allowed as the answer to this question."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum QuestionnaireItemAnswerOptionValue {
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The actual value to for an initial answer."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum QuestionnaireItemInitialValue {
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
#[doc = "A constraint indicating that this item should only be enabled (displayed/allow answers to be captured) when the specified condition is true."]
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItemEnableWhen {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The linkId for the question whose answer (or lack of answer) governs whether this item is enabled."]
    pub r#question: super::super::types::String,
    #[doc = "Specifies the criteria by which the question is enabled."]
    pub r#operator: super::super::types::Code,
    #[doc = "A value that the referenced question is tested using the specified operator in order for the item to be enabled.  If there are multiple answers, a match on any of the answers suffices.  If different behavior is desired (all must match, at least 2 must match, etc.), consider using the enableWhenExpression extension."]
    pub r#answer: QuestionnaireItemEnableWhenAnswer,
}
#[allow(clippy::derivable_impls)]
impl Default for QuestionnaireItemEnableWhen {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#question: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#operator: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#answer: Default::default(),
        }
    }
}
#[doc = "One of the permitted answers for the question."]
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItemAnswerOption {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A potential answer that's allowed as the answer to this question."]
    pub r#value: QuestionnaireItemAnswerOptionValue,
    #[doc = "Indicates whether the answer value is selected when the list of possible answers is initially shown."]
    pub r#initial_selected: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for QuestionnaireItemAnswerOption {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
            r#initial_selected: Default::default(),
        }
    }
}
#[doc = "One or more values that should be pre-populated in the answer when initially rendering the questionnaire for user input."]
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItemInitial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The actual value to for an initial answer."]
    pub r#value: QuestionnaireItemInitialValue,
}
#[allow(clippy::derivable_impls)]
impl Default for QuestionnaireItemInitial {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "A particular question, question grouping or display text that is part of the questionnaire."]
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier that is unique within the Questionnaire allowing linkage to the equivalent item in a QuestionnaireResponse resource."]
    pub r#link_id: super::super::types::String,
    #[doc = "This element is a URI that refers to an [ElementDefinition](https://hl7.org/FHIR/elementdefinition.html)) or to an [ObservationDefinition](https://hl7.org/FHIR/observationdefinition.html)) that provides information about this item, including information that might otherwise be included in the instance of the Questionnaire resource. A detailed description of the construction of the URI is shown in [Comments](questionnaire.html#definition), below."]
    pub r#definition: Option<super::super::types::Uri>,
    #[doc = "A terminology code that corresponds to this group or question (e.g. a code from LOINC, which defines many questions and answers)."]
    pub r#code: Vec<super::super::types::Coding>,
    #[doc = "A short label for a particular group, question or set of display text within the questionnaire used for reference by the individual completing the questionnaire."]
    pub r#prefix: Option<super::super::types::String>,
    #[doc = "The name of a section, the text of a question or text content for a display item."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The type of questionnaire item this is - whether text for display, a grouping of other items or a particular type of data to be captured (string, integer, Coding, etc.)."]
    pub r#type: super::super::types::Code,
    #[doc = "A constraint indicating that this item should only be enabled (displayed/allow answers to be captured) when the specified condition is true."]
    pub r#enable_when: Vec<QuestionnaireItemEnableWhen>,
    #[doc = "Controls how multiple enableWhen values are interpreted -  whether all or any must be true."]
    pub r#enable_behavior: Option<super::super::types::Code>,
    #[doc = "Indicates if and how items that are disabled (because enableWhen evaluates to 'false') should be displayed."]
    pub r#disabled_display: Option<super::super::types::Code>,
    #[doc = "An indication, if true, that the item must be present in a \"completed\" QuestionnaireResponse.  If false, the item may be skipped when answering the questionnaire."]
    pub r#required: Option<super::super::types::Boolean>,
    #[doc = "An indication, if true, that a QuestionnaireResponse for this item may include multiple answers associated with a single instance of this item (for question-type items) or multiple repetitions of the item (for group-type items)."]
    pub r#repeats: Option<super::super::types::Boolean>,
    #[doc = "An indication, when true, that the value cannot be changed by a human respondent to the Questionnaire."]
    pub r#read_only: Option<super::super::types::Boolean>,
    #[doc = "The maximum number of characters that are permitted in the answer to be considered a \"valid\" QuestionnaireResponse."]
    pub r#max_length: Option<super::super::types::Integer>,
    #[doc = "For items that have a defined set of allowed answers (via answerOption or answerValueSet), indicates whether values *other* than those specified can be selected."]
    pub r#answer_constraint: Option<super::super::types::Code>,
    #[doc = "A reference to a value set containing a list of values representing permitted answers for a question."]
    pub r#answer_value_set: Option<super::super::types::Canonical>,
    #[doc = "One of the permitted answers for the question."]
    pub r#answer_option: Vec<QuestionnaireItemAnswerOption>,
    #[doc = "One or more values that should be pre-populated in the answer when initially rendering the questionnaire for user input."]
    pub r#initial: Vec<QuestionnaireItemInitial>,
    #[doc = "Text, questions and other groups to be nested beneath a question or group."]
    pub r#item: Vec<QuestionnaireItem>,
}
#[allow(clippy::derivable_impls)]
impl Default for QuestionnaireItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#definition: Default::default(),
            r#code: Default::default(),
            r#prefix: Default::default(),
            r#text: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#enable_when: Default::default(),
            r#enable_behavior: Default::default(),
            r#disabled_display: Default::default(),
            r#required: Default::default(),
            r#repeats: Default::default(),
            r#read_only: Default::default(),
            r#max_length: Default::default(),
            r#answer_constraint: Default::default(),
            r#answer_value_set: Default::default(),
            r#answer_option: Default::default(),
            r#initial: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coherent, consistent data collection.\n\nTo support structured, hierarchical registration of data gathered using digital forms and other questionnaires.  Questionnaires provide greater control over presentation and allow capture of data in a domain-independent way (i.e. capturing information that would otherwise require multiple distinct types of resources)."]
#[derive(Debug, Clone, PartialEq)]
pub struct Questionnaire {
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
    #[doc = "An absolute URI that is used to identify this questionnaire when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this questionnaire is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the questionnaire is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this questionnaire when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the questionnaire when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the questionnaire author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<QuestionnaireVersionAlgorithm>,
    #[doc = "A natural language name identifying the questionnaire. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the questionnaire."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The URL of a Questionnaire that this Questionnaire is based on."]
    pub r#derived_from: Vec<super::super::types::Canonical>,
    #[doc = "The current state of this questionnaire."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this questionnaire is authored for testing purposes (or education/evaluation/marketing) and is not intended for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The types of subjects that can be the subject of responses created for the questionnaire."]
    pub r#subject_type: Vec<super::super::types::Code>,
    #[doc = "The date  (and optionally time) when the questionnaire was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the questionnaire changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the questionnaire."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the questionnaire from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate questionnaires."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the questionnaire is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this questionnaire is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the questionnaire and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the questionnaire."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the questionnaire content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "An identifier for this collection of questions in a particular terminology such as LOINC."]
    pub r#code: Vec<super::super::types::Coding>,
    #[doc = "A particular question, question grouping or display text that is part of the questionnaire."]
    pub r#item: Vec<QuestionnaireItem>,
}
#[allow(clippy::derivable_impls)]
impl Default for Questionnaire {
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
            r#url: Default::default(),
            r#identifier: Default::default(),
            r#version: Default::default(),
            r#version_algorithm: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#derived_from: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#subject_type: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#copyright_label: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#code: Default::default(),
            r#item: Default::default(),
        }
    }
}
