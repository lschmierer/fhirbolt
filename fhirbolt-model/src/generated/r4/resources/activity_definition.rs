// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "A code or group definition that describes the intended subject of the activity being defined."]
#[derive(Debug, Clone, PartialEq)]
pub enum ActivityDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ActivityDefinitionSubject {
    fn default() -> ActivityDefinitionSubject {
        ActivityDefinitionSubject::Invalid
    }
}
#[doc = "The period, timing or frequency upon which the described activity is to occur."]
#[derive(Debug, Clone, PartialEq)]
pub enum ActivityDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    Duration(Box<super::super::types::Duration>),
    Invalid,
}
impl Default for ActivityDefinitionTiming {
    fn default() -> ActivityDefinitionTiming {
        ActivityDefinitionTiming::Invalid
    }
}
#[doc = "Identifies the food, drug or other product being consumed or supplied in the activity."]
#[derive(Debug, Clone, PartialEq)]
pub enum ActivityDefinitionProduct {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for ActivityDefinitionProduct {
    fn default() -> ActivityDefinitionProduct {
        ActivityDefinitionProduct::Invalid
    }
}
#[doc = "Indicates who should participate in performing the action described."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ActivityDefinitionParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of participant in the action."]
    pub r#type: super::super::types::Code,
    #[doc = "The role the participant should play in performing the described action."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Dynamic values that will be evaluated to produce values for elements of the resulting resource. For example, if the dosage of a medication must be computed based on the patient's weight, a dynamic value would be used to specify an expression that calculated the weight, and the path on the request resource that would contain the result."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ActivityDefinitionDynamicValue {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The path to the element to be customized. This is the path on the resource that will hold the result of the calculation defined by the expression. The specified path SHALL be a FHIRPath resolveable on the specified target type of the ActivityDefinition, and SHALL consist only of identifiers, constant indexers, and a restricted subset of functions. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details)."]
    pub r#path: super::super::types::String,
    #[doc = "An expression specifying the value of the customized element."]
    pub r#expression: Box<super::super::types::Expression>,
}
#[doc = "This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ActivityDefinition {
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
    #[doc = "An absolute URI that is used to identify this activity definition when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this activity definition is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the activity definition is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this activity definition when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the activity definition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the activity definition author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active assets."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the activity definition. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the activity definition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the activity definition giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "The status of this activity definition. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this activity definition is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "A code or group definition that describes the intended subject of the activity being defined."]
    pub r#subject: Option<ActivityDefinitionSubject>,
    #[doc = "The date  (and optionally time) when the activity definition was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the activity definition changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the activity definition."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the activity definition from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate activity definition instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the activity definition is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this activity definition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A detailed description of how the activity definition is used from a clinical perspective."]
    pub r#usage: Option<super::super::types::String>,
    #[doc = "A copyright statement relating to the activity definition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the activity definition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the activity definition content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the activity. Topics provide a high-level categorization of the activity that can be useful for filtering and searching."]
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Related artifacts such as additional documentation, justification, or bibliographic references."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "A reference to a Library resource containing any formal logic used by the activity definition."]
    pub r#library: Vec<super::super::types::Canonical>,
    #[doc = "A description of the kind of resource the activity definition is representing. For example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest. Typically, but not always, this is a Request resource."]
    pub r#kind: Option<super::super::types::Code>,
    #[doc = "A profile to which the target of the activity definition is expected to conform."]
    pub r#profile: Option<super::super::types::Canonical>,
    #[doc = "Detailed description of the type of activity; e.g. What lab test, what procedure, what kind of encounter."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the level of authority/intentionality associated with the activity and where the request should fit into the workflow chain."]
    pub r#intent: Option<super::super::types::Code>,
    #[doc = "Indicates how quickly the activity  should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "Set this to true if the definition is to indicate that a particular activity should NOT be performed. If true, this element should be interpreted to reinforce a negative coding. For example NPO as a code with a doNotPerform of true would still indicate to NOT perform the action."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "The period, timing or frequency upon which the described activity is to occur."]
    pub r#timing: Option<ActivityDefinitionTiming>,
    #[doc = "Identifies the facility where the activity will occur; e.g. home, hospital, specific clinic, etc."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates who should participate in performing the action described."]
    pub r#participant: Vec<ActivityDefinitionParticipant>,
    #[doc = "Identifies the food, drug or other product being consumed or supplied in the activity."]
    pub r#product: Option<ActivityDefinitionProduct>,
    #[doc = "Identifies the quantity expected to be consumed at once (per dose, per meal, etc.)."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Provides detailed dosage instructions in the same way that they are described for MedicationRequest resources."]
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
    #[doc = "Indicates the sites on the subject's body where the procedure should be performed (I.e. the target sites)."]
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Defines specimen requirements for the action to be performed, such as required specimens for a lab test."]
    pub r#specimen_requirement: Vec<Box<super::super::types::Reference>>,
    #[doc = "Defines observation requirements for the action to be performed, such as body weight or surface area."]
    pub r#observation_requirement: Vec<Box<super::super::types::Reference>>,
    #[doc = "Defines the observations that are expected to be produced by the action."]
    pub r#observation_result_requirement: Vec<Box<super::super::types::Reference>>,
    #[doc = "A reference to a StructureMap resource that defines a transform that can be executed to produce the intent resource using the ActivityDefinition instance as the input."]
    pub r#transform: Option<super::super::types::Canonical>,
    #[doc = "Dynamic values that will be evaluated to produce values for elements of the resulting resource. For example, if the dosage of a medication must be computed based on the patient's weight, a dynamic value would be used to specify an expression that calculated the weight, and the path on the request resource that would contain the result."]
    pub r#dynamic_value: Vec<ActivityDefinitionDynamicValue>,
}
