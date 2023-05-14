// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "Batch numbering."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPackagedBatchIdentifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A number appearing on the outer packaging of a specific batch."]
    pub r#outer_packaging: Box<super::super::types::Identifier>,
    #[doc = "A number appearing on the immediate packaging (and not the outer packaging)."]
    pub r#immediate_packaging: Option<Box<super::super::types::Identifier>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPackagedBatchIdentifier {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#outer_packaging: Box::new(super::super::types::Identifier {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#immediate_packaging: Default::default(),
        }
    }
}
#[doc = "A packaging item, as a contained for medicine, possibly with other packaging items within."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPackagedPackageItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Including possibly Data Carrier Identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The physical type of the container of the medicine."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The quantity of this package in the medicinal product, at the current level of packaging. The outermost is always 1."]
    pub r#quantity: Box<super::super::types::Quantity>,
    #[doc = "Material type of the package item."]
    pub r#material: Vec<super::super::types::CodeableConcept>,
    #[doc = "A possible alternate material for the packaging."]
    pub r#alternate_material: Vec<super::super::types::CodeableConcept>,
    #[doc = "A device accompanying a medicinal product."]
    pub r#device: Vec<super::super::types::Reference>,
    #[doc = "The manufactured item as contained in the packaged medicinal product."]
    pub r#manufactured_item: Vec<super::super::types::Reference>,
    #[doc = "Allows containers within containers."]
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
    #[doc = "Dimensions, color etc."]
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    #[doc = "Other codeable characteristics."]
    pub r#other_characteristics: Vec<super::super::types::CodeableConcept>,
    #[doc = "Shelf Life and storage information."]
    pub r#shelf_life_storage: Vec<super::super::types::ProductShelfLife>,
    #[doc = "Manufacturer of this Package Item."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPackagedPackageItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#quantity: Box::new(super::super::types::Quantity {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#material: Default::default(),
            r#alternate_material: Default::default(),
            r#device: Default::default(),
            r#manufactured_item: Default::default(),
            r#package_item: Default::default(),
            r#physical_characteristics: Default::default(),
            r#other_characteristics: Default::default(),
            r#shelf_life_storage: Default::default(),
            r#manufacturer: Default::default(),
        }
    }
}
#[doc = "A medicinal product in a container or package."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPackaged {
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
    #[doc = "Unique identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The product with this is a pack for."]
    pub r#subject: Vec<super::super::types::Reference>,
    #[doc = "Textual description."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The legal status of supply of the medicinal product as classified by the regulator."]
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Marketing information."]
    pub r#marketing_status: Vec<super::super::types::MarketingStatus>,
    #[doc = "Manufacturer of this Package Item."]
    pub r#marketing_authorization: Option<Box<super::super::types::Reference>>,
    #[doc = "Manufacturer of this Package Item."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
    #[doc = "Batch numbering."]
    pub r#batch_identifier: Vec<MedicinalProductPackagedBatchIdentifier>,
    #[doc = "A packaging item, as a contained for medicine, possibly with other packaging items within."]
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPackaged {
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
            r#subject: Default::default(),
            r#description: Default::default(),
            r#legal_status_of_supply: Default::default(),
            r#marketing_status: Default::default(),
            r#marketing_authorization: Default::default(),
            r#manufacturer: Default::default(),
            r#batch_identifier: Default::default(),
            r#package_item: Default::default(),
        }
    }
}
