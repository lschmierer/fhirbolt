// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Time of product collection."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Property values."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductPropertyValue {
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Period(Box<super::super::types::Period>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "How this product was collected."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductCollection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Healthcare professional who is performing the collection."]
    pub r#collector: Option<Box<super::super::types::Reference>>,
    #[doc = "The patient or entity, such as a hospital or vendor in the case of a processed/manipulated/manufactured product, providing the product."]
    pub r#source: Option<Box<super::super::types::Reference>>,
    #[doc = "Time of product collection."]
    pub r#collected: Option<BiologicallyDerivedProductCollectionCollected>,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProductCollection {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#collector: Default::default(),
            r#source: Default::default(),
            r#collected: Default::default(),
        }
    }
}
#[doc = "A property that is specific to this BiologicallyDerviedProduct instance."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code that specifies the property. It should reference an established coding system."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Property values."]
    pub r#value: BiologicallyDerivedProductPropertyValue,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProductProperty {
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
#[doc = "A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProduct {
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
    #[doc = "Broad category of this product."]
    pub r#product_category: Option<Box<super::super::types::Coding>>,
    #[doc = "A codified value that systematically supports characterization and classification of medical products of human origin inclusive of processing conditions such as additives, volumes and handling conditions."]
    pub r#product_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Parent product (if any) for this biologically-derived product."]
    pub r#parent: Vec<super::super::types::Reference>,
    #[doc = "Request to obtain and/or infuse this biologically derived product."]
    pub r#request: Vec<super::super::types::Reference>,
    #[doc = "Unique instance identifiers assigned to a biologically derived product. Note: This is a business identifier, not a resource identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled."]
    pub r#biological_source_event: Option<Box<super::super::types::Identifier>>,
    #[doc = "Processing facilities responsible for the labeling and distribution of this biologically derived product."]
    pub r#processing_facility: Vec<super::super::types::Reference>,
    #[doc = "A unique identifier for an aliquot of a product.  Used to distinguish individual aliquots of a product carrying the same biologicalSource and productCode identifiers."]
    pub r#division: Option<super::super::types::String>,
    #[doc = "Whether the product is currently available."]
    pub r#product_status: Option<Box<super::super::types::Coding>>,
    #[doc = "Date, and where relevant time, of expiration."]
    pub r#expiration_date: Option<super::super::types::DateTime>,
    #[doc = "How this product was collected."]
    pub r#collection: Option<BiologicallyDerivedProductCollection>,
    #[doc = "The temperature requirements for storage of the biologically-derived product."]
    pub r#storage_temp_requirements: Option<Box<super::super::types::Range>>,
    #[doc = "A property that is specific to this BiologicallyDerviedProduct instance."]
    pub r#property: Vec<BiologicallyDerivedProductProperty>,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProduct {
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
            r#product_category: Default::default(),
            r#product_code: Default::default(),
            r#parent: Default::default(),
            r#request: Default::default(),
            r#identifier: Default::default(),
            r#biological_source_event: Default::default(),
            r#processing_facility: Default::default(),
            r#division: Default::default(),
            r#product_status: Default::default(),
            r#expiration_date: Default::default(),
            r#collection: Default::default(),
            r#storage_temp_requirements: Default::default(),
            r#property: Default::default(),
        }
    }
}
