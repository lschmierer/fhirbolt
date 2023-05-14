// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "Entries in this list."]
#[derive(Debug, Clone, PartialEq)]
pub struct ListEntry {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The flag allows the system constructing the list to indicate the role and significance of the item in the list."]
    pub r#flag: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "True if this item is marked as deleted in the list."]
    pub r#deleted: Option<super::super::types::Boolean>,
    #[doc = "When this item was added to the list."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "A reference to the actual resource from which data was derived."]
    pub r#item: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ListEntry {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#flag: Default::default(),
            r#deleted: Default::default(),
            r#date: Default::default(),
            r#item: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "A list is a curated collection of resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct List {
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
    #[doc = "Identifier for the List assigned for business purposes outside the context of FHIR."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Indicates the current state of this list."]
    pub r#status: super::super::types::Code,
    #[doc = "How this list was prepared - whether it is a working list that is suitable for being maintained on an ongoing basis, or if it represents a snapshot of a list of items from another source, or whether it is a prepared list where items may be marked as added, modified or deleted."]
    pub r#mode: super::super::types::Code,
    #[doc = "A label for the list assigned by the author."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "This code defines the purpose of the list - why it was created."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The common subject (or patient) of the resources that are in the list if there is one."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter that is the context in which this list was created."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date that the list was prepared."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The entity responsible for deciding what the contents of the list were. Where the list was created by a human, this is the same as the author of the list."]
    pub r#source: Option<Box<super::super::types::Reference>>,
    #[doc = "What order applies to the items in the list."]
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Comments that apply to the overall list."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Entries in this list."]
    pub r#entry: Vec<ListEntry>,
    #[doc = "If the list is empty, why the list is empty."]
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for List {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#mode: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#title: Default::default(),
            r#code: Default::default(),
            r#subject: Default::default(),
            r#encounter: Default::default(),
            r#date: Default::default(),
            r#source: Default::default(),
            r#ordered_by: Default::default(),
            r#note: Default::default(),
            r#entry: Default::default(),
            r#empty_reason: Default::default(),
        }
    }
}
