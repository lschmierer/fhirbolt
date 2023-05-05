// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "The value of the attribute."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum InventoryItemCharacteristicValue {
    String(Box<super::super::types::String>),
    Integer(Box<super::super::types::Integer>),
    Decimal(Box<super::super::types::Decimal>),
    Boolean(Box<super::super::types::Boolean>),
    Url(Box<super::super::types::Url>),
    DateTime(Box<super::super::types::DateTime>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    Annotation(Box<super::super::types::Annotation>),
    Address(Box<super::super::types::Address>),
    Duration(Box<super::super::types::Duration>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "The item name(s) - the brand name, or common name, functional name, generic name."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of name e.g. 'brand-name', 'functional-name', 'common-name'."]
    pub r#name_type: Box<super::super::types::Coding>,
    #[doc = "The language that the item name is expressed in."]
    pub r#language: super::super::types::Code,
    #[doc = "The name or designation that the item is given."]
    pub r#name: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryItemName {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name_type: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#language: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Organization(s) responsible for the product."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemResponsibleOrganization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The role of the organization e.g. manufacturer, distributor, etc."]
    pub r#role: Box<super::super::types::CodeableConcept>,
    #[doc = "An organization that has an association with the item, e.g. manufacturer, distributor, responsible, etc."]
    pub r#organization: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryItemResponsibleOrganization {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#role: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#organization: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "The descriptive characteristics of the inventory item."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemDescription {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The language for the item description, when an item must be described in different languages and those languages may be authoritative and not translations of a 'main' language."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "Textual description of the item."]
    pub r#description: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryItemDescription {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#language: Default::default(),
            r#description: Default::default(),
        }
    }
}
#[doc = "Association with other items or products."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemAssociation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "This attribute defined the type of association when establishing associations or relations between items, e.g. 'packaged within' or 'used with' or 'to be mixed with."]
    pub r#association_type: Box<super::super::types::CodeableConcept>,
    #[doc = "The related item or product."]
    pub r#related_item: Box<super::super::types::Reference>,
    #[doc = "The quantity of the related product in this product - Numerator is the quantity of the related product. Denominator is the quantity of the present product. For example a value of 20 means that this product contains 20 units of the related product; a value of 1:20 means the inverse - that the contained product contains 20 units of the present product."]
    pub r#quantity: Box<super::super::types::Ratio>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryItemAssociation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#association_type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#related_item: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#quantity: Box::new(super::super::types::Ratio {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "The descriptive or identifying characteristics of the item."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of characteristic that is being defined."]
    pub r#characteristic_type: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the attribute."]
    pub r#value: InventoryItemCharacteristicValue,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryItemCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#characteristic_type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
        }
    }
}
#[doc = "Instances or occurrences of the product."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The identifier for the physical instance, typically a serial number."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The lot or batch number of the item."]
    pub r#lot_number: Option<super::super::types::String>,
    #[doc = "The expiry date or date and time for the product."]
    pub r#expiry: Option<super::super::types::DateTime>,
    #[doc = "The subject that the item is associated with."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The location that the item is associated with."]
    pub r#location: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryItemInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#lot_number: Default::default(),
            r#expiry: Default::default(),
            r#subject: Default::default(),
            r#location: Default::default(),
        }
    }
}
#[doc = "functional description of an inventory item used in inventory and supply-related workflows."]
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItem {
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
    #[doc = "Business identifier for the inventory item."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Status of the item entry."]
    pub r#status: super::super::types::Code,
    #[doc = "Category or class of the item."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Code designating the specific type of item."]
    pub r#code: Vec<super::super::types::CodeableConcept>,
    #[doc = "The item name(s) - the brand name, or common name, functional name, generic name."]
    pub r#name: Vec<InventoryItemName>,
    #[doc = "Organization(s) responsible for the product."]
    pub r#responsible_organization: Vec<InventoryItemResponsibleOrganization>,
    #[doc = "The descriptive characteristics of the inventory item."]
    pub r#description: Option<InventoryItemDescription>,
    #[doc = "The usage status e.g. recalled, in use, discarded... This can be used to indicate that the items have been taken out of inventory, or are in use, etc."]
    pub r#inventory_status: Vec<super::super::types::CodeableConcept>,
    #[doc = "The base unit of measure - the unit in which the product is used or counted."]
    pub r#base_unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Net content or amount present in the item."]
    pub r#net_content: Option<Box<super::super::types::Quantity>>,
    #[doc = "Association with other items or products."]
    pub r#association: Vec<InventoryItemAssociation>,
    #[doc = "The descriptive or identifying characteristics of the item."]
    pub r#characteristic: Vec<InventoryItemCharacteristic>,
    #[doc = "Instances or occurrences of the product."]
    pub r#instance: Option<InventoryItemInstance>,
    #[doc = "Link to a product resource used in clinical workflows."]
    pub r#product_reference: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for InventoryItem {
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
            r#category: Default::default(),
            r#code: Default::default(),
            r#name: Default::default(),
            r#responsible_organization: Default::default(),
            r#description: Default::default(),
            r#inventory_status: Default::default(),
            r#base_unit: Default::default(),
            r#net_content: Default::default(),
            r#association: Default::default(),
            r#characteristic: Default::default(),
            r#instance: Default::default(),
            r#product_reference: Default::default(),
        }
    }
}
