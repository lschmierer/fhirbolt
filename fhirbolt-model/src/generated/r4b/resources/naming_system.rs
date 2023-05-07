// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "Indicates how the system may be identified when referenced in electronic exchange."]
#[derive(Debug, Clone, PartialEq)]
pub struct NamingSystemUniqueId {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies the unique identifier scheme used for this particular identifier."]
    pub r#type: super::super::types::Code,
    #[doc = "The string that should be sent over the wire to identify the code system or identifier system."]
    pub r#value: super::super::types::String,
    #[doc = "Indicates whether this identifier is the \"preferred\" identifier of this type."]
    pub r#preferred: Option<super::super::types::Boolean>,
    #[doc = "Notes about the past or intended usage of this identifier."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Identifies the period of time over which this identifier is considered appropriate to refer to the naming system.  Outside of this window, the identifier might be non-deterministic."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for NamingSystemUniqueId {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#preferred: Default::default(),
            r#comment: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a \"System\" used within the Identifier and Coding data types."]
#[derive(Debug, Clone, PartialEq)]
pub struct NamingSystem {
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
    #[doc = "A natural language name identifying the naming system. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "The status of this naming system. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the purpose for the naming system - what kinds of things does it make unique?"]
    pub r#kind: super::super::types::Code,
    #[doc = "The date  (and optionally time) when the naming system was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the naming system changes."]
    pub r#date: super::super::types::DateTime,
    #[doc = "The name of the organization or individual that published the naming system."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "The name of the organization that is responsible for issuing identifiers or codes for this namespace and ensuring their non-collision."]
    pub r#responsible: Option<super::super::types::String>,
    #[doc = "Categorizes a naming system for easier search by grouping related naming systems."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A free text natural language description of the naming system from a consumer's perspective. Details about what the namespace identifies including scope, granularity, version labeling, etc."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate naming system instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the naming system is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Provides guidance on the use of the namespace, including the handling of formatting characters, use of upper vs. lower case, etc."]
    pub r#usage: Option<super::super::types::String>,
    #[doc = "Indicates how the system may be identified when referenced in electronic exchange."]
    pub r#unique_id: Vec<NamingSystemUniqueId>,
}
#[allow(clippy::derivable_impls)]
impl Default for NamingSystem {
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
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#kind: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#date: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#responsible: Default::default(),
            r#type: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#usage: Default::default(),
            r#unique_id: Default::default(),
        }
    }
}
