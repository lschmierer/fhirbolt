// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
#[doc = "The item or items in this listing."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryReportInventoryListingItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The inventory category or classification of the items being reported. This is meant not for defining the product, but for inventory categories e.g. 'pending recount' or 'damaged'."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quantity of the item or items being reported."]
    pub r#quantity: Box<super::super::types::Quantity>,
    #[doc = "The code or reference to the item type."]
    pub r#item: Box<super::super::types::CodeableReference>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryReportInventoryListingItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Default::default(),
            r#quantity: Box::new(super::super::types::Quantity {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#item: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "An inventory listing section (grouped by any of the attributes)."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryReportInventoryListing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Location of the inventory items."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The status of the items."]
    pub r#item_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date and time when the items were counted."]
    pub r#counting_date_time: Option<super::super::types::DateTime>,
    #[doc = "The item or items in this listing."]
    pub r#item: Vec<InventoryReportInventoryListingItem>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryReportInventoryListing {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#location: Default::default(),
            r#item_status: Default::default(),
            r#counting_date_time: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "A report of inventory or stock items."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryReport {
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
    #[doc = "Business identifier for the InventoryReport."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of the inventory check or notification - whether this is draft (e.g. the report is still pending some updates) or active."]
    pub r#status: super::super::types::Code,
    #[doc = "Whether the report is about the current inventory count (snapshot) or a differential change in inventory (change)."]
    pub r#count_type: super::super::types::Code,
    #[doc = "What type of operation is being performed - addition or subtraction."]
    pub r#operation_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The reason for this count - regular count, ad-hoc count, new arrivals, etc."]
    pub r#operation_type_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the report has been submitted."]
    pub r#reported_date_time: super::super::types::DateTime,
    #[doc = "Who submits the report."]
    pub r#reporter: Option<Box<super::super::types::Reference>>,
    #[doc = "The period the report refers to."]
    pub r#reporting_period: Option<Box<super::super::types::Period>>,
    #[doc = "An inventory listing section (grouped by any of the attributes)."]
    pub r#inventory_listing: Vec<InventoryReportInventoryListing>,
    #[doc = "A note associated with the InventoryReport."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryReport {
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
            r#count_type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#operation_type: Default::default(),
            r#operation_type_reason: Default::default(),
            r#reported_date_time: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#reporter: Default::default(),
            r#reporting_period: Default::default(),
            r#inventory_listing: Default::default(),
            r#note: Default::default(),
        }
    }
}
