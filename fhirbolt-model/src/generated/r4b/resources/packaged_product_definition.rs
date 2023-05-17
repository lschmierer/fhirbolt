// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum PackagedProductDefinitionPackageShelfLifeStoragePeriod {
    Duration(Box<super::super::types::Duration>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "A value for the characteristic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum PackagedProductDefinitionPackagePropertyValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Date(Box<super::super::types::Date>),
    Boolean(Box<super::super::types::Boolean>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "The legal status of supply of the packaged item as classified by the regulator."]
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionLegalStatusOfSupply {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The actual status of supply. Conveys in what situation this package type may be supplied for use."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The place where the legal status of supply applies. When not specified, this indicates it is unknown in this context."]
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for PackagedProductDefinitionLegalStatusOfSupply {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#jurisdiction: Default::default(),
        }
    }
}
#[doc = "Shelf Life and storage information."]
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionPackageShelfLifeStorage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life after the first opening of a bottle, etc. The shelf life type shall be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#period: Option<PackagedProductDefinitionPackageShelfLifeStoragePeriod>,
    #[doc = "Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary. The controlled term and the controlled term identifier shall be specified."]
    pub r#special_precautions_for_storage: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for PackagedProductDefinitionPackageShelfLifeStorage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#period: Default::default(),
            r#special_precautions_for_storage: Default::default(),
        }
    }
}
#[doc = "General characteristics of this item."]
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionPackageProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code expressing the type of characteristic."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the characteristic."]
    pub r#value: Option<PackagedProductDefinitionPackagePropertyValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for PackagedProductDefinitionPackageProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
        }
    }
}
#[doc = "The item(s) within the packaging."]
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionPackageContainedItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The actual item(s) of medication, as manufactured, or a device (typically, but not necessarily, a co-packaged one), or other medically related item (such as food, biologicals, raw materials, medical fluids, gases etc.), as contained in the package. This also allows another whole packaged product to be included, which is solely for the case where a package of other entire packages is wanted - such as a wholesale or distribution pack (for layers within one package, use PackagedProductDefinition.package.package)."]
    pub r#item: Box<super::super::types::CodeableReference>,
    #[doc = "The number of this type of item within this packaging."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
#[allow(clippy::derivable_impls)]
impl Default for PackagedProductDefinitionPackageContainedItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#amount: Default::default(),
        }
    }
}
#[doc = "A packaging item, as a container for medically related items, possibly with other packaging items within, or a packaging component, such as bottle cap (which is not a device or a medication manufactured item)."]
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinitionPackage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier that is specific to this particular part of the packaging. Including possibly Data Carrier Identifier (a GS1 barcode)."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The physical type of the container of the items."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quantity of this level of packaging in the package that contains it. If specified, the outermost level is always 1."]
    pub r#quantity: Option<super::super::types::Integer>,
    #[doc = "Material type of the package item."]
    pub r#material: Vec<super::super::types::CodeableConcept>,
    #[doc = "A possible alternate material for this part of the packaging, that is allowed to be used instead of the usual material (e.g. different types of plastic for a blister sleeve)."]
    pub r#alternate_material: Vec<super::super::types::CodeableConcept>,
    #[doc = "Shelf Life and storage information."]
    pub r#shelf_life_storage: Vec<PackagedProductDefinitionPackageShelfLifeStorage>,
    #[doc = "Manufacturer of this package Item. When there are multiple it means these are all possible manufacturers."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
    #[doc = "General characteristics of this item."]
    pub r#property: Vec<PackagedProductDefinitionPackageProperty>,
    #[doc = "The item(s) within the packaging."]
    pub r#contained_item: Vec<PackagedProductDefinitionPackageContainedItem>,
    #[doc = "Allows containers (and parts of containers) parwithin containers, still a single packaged product.  See also PackagedProductDefinition.package.containedItem.item(PackagedProductDefinition)."]
    pub r#package: Vec<PackagedProductDefinitionPackage>,
}
#[allow(clippy::derivable_impls)]
impl Default for PackagedProductDefinitionPackage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#type: Default::default(),
            r#quantity: Default::default(),
            r#material: Default::default(),
            r#alternate_material: Default::default(),
            r#shelf_life_storage: Default::default(),
            r#manufacturer: Default::default(),
            r#property: Default::default(),
            r#contained_item: Default::default(),
            r#package: Default::default(),
        }
    }
}
#[doc = "A medically related item or items, in a container or package."]
#[derive(Debug, Clone, PartialEq)]
pub struct PackagedProductDefinition {
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
    #[doc = "A unique identifier for this package as whole. Unique instance identifiers assigned to a package by manufacturers, regulators, drug catalogue custodians or other organizations."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A name for this package. Typically what it would be listed as in a drug formulary or catalogue, inventory etc."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A high level category e.g. medicinal product, raw material, shipping/transport container, etc."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The product that this is a pack for."]
    pub r#package_for: Vec<super::super::types::Reference>,
    #[doc = "The status within the lifecycle of this item. A high level status, this is not intended to duplicate details carried elsewhere such as legal status, or authorization or marketing status."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the given status became applicable."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "A total of the complete count of contained items of a particular type/form, independent of sub-packaging or organization. This can be considered as the pack size. This attribute differs from containedItem.amount in that it can give a single aggregated count of all tablet types in a pack, even when these are different manufactured items. For example a pill pack of 21 tablets plus 7 sugar tablets, can be denoted here as '28 tablets'. This attribute is repeatable so that the different item types in one pack type can be counted (e.g. a count of vials and count of syringes). Each repeat must have different units, so that it is clear what the different sets of counted items are, and it is not intended to allow different counts of similar items (e.g. not '2 tubes and 3 tubes'). Repeats are not to be used to represent different pack sizes (e.g. 20 pack vs. 50 pack) - which would be different instances of this resource."]
    pub r#contained_item_quantity: Vec<super::super::types::Quantity>,
    #[doc = "Textual description. Note that this is not the name of the package or product."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The legal status of supply of the packaged item as classified by the regulator."]
    pub r#legal_status_of_supply: Vec<PackagedProductDefinitionLegalStatusOfSupply>,
    #[doc = "Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated."]
    pub r#marketing_status: Vec<super::super::types::MarketingStatus>,
    #[doc = "Allows the key features to be recorded, such as \"hospital pack\", \"nurse prescribable\", \"calendar pack\"."]
    pub r#characteristic: Vec<super::super::types::CodeableConcept>,
    #[doc = "States whether a drug product is supplied with another item such as a diluent or adjuvant."]
    pub r#copackaged_indicator: Option<super::super::types::Boolean>,
    #[doc = "Manufacturer of this package type. When there are multiple it means these are all possible manufacturers."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
    #[doc = "A packaging item, as a container for medically related items, possibly with other packaging items within, or a packaging component, such as bottle cap (which is not a device or a medication manufactured item)."]
    pub r#package: Option<PackagedProductDefinitionPackage>,
}
#[allow(clippy::derivable_impls)]
impl Default for PackagedProductDefinition {
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
            r#name: Default::default(),
            r#type: Default::default(),
            r#package_for: Default::default(),
            r#status: Default::default(),
            r#status_date: Default::default(),
            r#contained_item_quantity: Default::default(),
            r#description: Default::default(),
            r#legal_status_of_supply: Default::default(),
            r#marketing_status: Default::default(),
            r#characteristic: Default::default(),
            r#copackaged_indicator: Default::default(),
            r#manufacturer: Default::default(),
            r#package: Default::default(),
        }
    }
}
