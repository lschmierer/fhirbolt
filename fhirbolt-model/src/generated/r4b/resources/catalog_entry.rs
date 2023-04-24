// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Used for example, to point to a substance, or to a device used to administer a medication."]
#[derive(Debug, Clone, PartialEq)]
pub struct CatalogEntryRelatedEntry {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relation to the related item: child, parent, packageContent, containerPackage, usedIn, uses, requires, etc."]
    pub r#relationtype: super::super::types::Code,
    #[doc = "The reference to the related item."]
    pub r#item: Box<super::super::types::Reference>,
}
impl Default for CatalogEntryRelatedEntry {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#relationtype: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#item: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "Catalog entries are wrappers that contextualize items included in a catalog."]
#[derive(Debug, Clone, PartialEq)]
pub struct CatalogEntry {
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
    #[doc = "Used in supporting different identifiers for the same product, e.g. manufacturer code and retailer code."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The type of item - medication, device, service, protocol or other."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the entry represents an orderable item."]
    pub r#orderable: super::super::types::Boolean,
    #[doc = "The item in a catalog or definition."]
    pub r#referenced_item: Box<super::super::types::Reference>,
    #[doc = "Used in supporting related concepts, e.g. NDC to RxNorm."]
    pub r#additional_identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Classes of devices, or ATC for medication."]
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to support catalog exchange even for unsupported products, e.g. getting list of medications even if not prescribable."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The time period in which this catalog entry is expected to be active."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date until which this catalog entry is expected to be active."]
    pub r#valid_to: Option<super::super::types::DateTime>,
    #[doc = "Typically date of issue is different from the beginning of the validity. This can be used to see when an item was last updated."]
    pub r#last_updated: Option<super::super::types::DateTime>,
    #[doc = "Used for examplefor Out of Formulary, or any specifics."]
    pub r#additional_characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "User for example for ATC classification, or."]
    pub r#additional_classification: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used for example, to point to a substance, or to a device used to administer a medication."]
    pub r#related_entry: Vec<CatalogEntryRelatedEntry>,
}
impl Default for CatalogEntry {
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
            r#type: Default::default(),
            r#orderable: {
                let mut default: super::super::types::Boolean = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#referenced_item: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#additional_identifier: Default::default(),
            r#classification: Default::default(),
            r#status: Default::default(),
            r#validity_period: Default::default(),
            r#valid_to: Default::default(),
            r#last_updated: Default::default(),
            r#additional_characteristic: Default::default(),
            r#additional_classification: Default::default(),
            r#related_entry: Default::default(),
        }
    }
}
