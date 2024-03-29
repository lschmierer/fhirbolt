// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Time of product collection."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductCollectionCollected {
    DateTime(super::super::types::DateTime),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Time of processing."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductProcessingTime {
    DateTime(super::super::types::DateTime),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Time of manipulation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductManipulationTime {
    DateTime(super::super::types::DateTime),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "How this product was collected."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductCollection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
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
#[doc = "Any processing of the product during collection that does not change the fundamental nature of the product. For example adding anti-coagulants during the collection of Peripheral Blood Stem Cells."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductProcessing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Description of of processing."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Procesing code."]
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Substance added during processing."]
    pub r#additive: Option<Box<super::super::types::Reference>>,
    #[doc = "Time of processing."]
    pub r#time: Option<BiologicallyDerivedProductProcessingTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProductProcessing {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#procedure: Default::default(),
            r#additive: Default::default(),
            r#time: Default::default(),
        }
    }
}
#[doc = "Any manipulation of product post-collection that is intended to alter the product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral Blood Stem Cells to make it more suitable for infusion."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductManipulation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Description of manipulation."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Time of manipulation."]
    pub r#time: Option<BiologicallyDerivedProductManipulationTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProductManipulation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#time: Default::default(),
        }
    }
}
#[doc = "Product storage."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductStorage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Description of storage."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Storage temperature."]
    pub r#temperature: Option<super::super::types::Decimal>,
    #[doc = "Temperature scale used."]
    pub r#scale: Option<super::super::types::Code>,
    #[doc = "Storage timeperiod."]
    pub r#duration: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProductStorage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#temperature: Default::default(),
            r#scale: Default::default(),
            r#duration: Default::default(),
        }
    }
}
#[doc = "A material substance originating from a biological entity intended to be transplanted or infused\ninto another (possibly the same) biological entity."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProduct {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
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
    #[doc = "This records identifiers associated with this biologically derived product instance that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate (e.g. in CDA documents, or in written / printed documentation)."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Broad category of this product."]
    pub r#product_category: Option<super::super::types::Code>,
    #[doc = "A code that identifies the kind of this biologically derived product (SNOMED Ctcode)."]
    pub r#product_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the product is currently available."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Procedure request to obtain this biologically derived product."]
    pub r#request: Vec<super::super::types::Reference>,
    #[doc = "Number of discrete units within this product."]
    pub r#quantity: Option<super::super::types::Integer>,
    #[doc = "Parent product (if any)."]
    pub r#parent: Vec<super::super::types::Reference>,
    #[doc = "How this product was collected."]
    pub r#collection: Option<BiologicallyDerivedProductCollection>,
    #[doc = "Any processing of the product during collection that does not change the fundamental nature of the product. For example adding anti-coagulants during the collection of Peripheral Blood Stem Cells."]
    pub r#processing: Vec<BiologicallyDerivedProductProcessing>,
    #[doc = "Any manipulation of product post-collection that is intended to alter the product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral Blood Stem Cells to make it more suitable for infusion."]
    pub r#manipulation: Option<BiologicallyDerivedProductManipulation>,
    #[doc = "Product storage."]
    pub r#storage: Vec<BiologicallyDerivedProductStorage>,
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
            r#identifier: Default::default(),
            r#product_category: Default::default(),
            r#product_code: Default::default(),
            r#status: Default::default(),
            r#request: Default::default(),
            r#quantity: Default::default(),
            r#parent: Default::default(),
            r#collection: Default::default(),
            r#processing: Default::default(),
            r#manipulation: Default::default(),
            r#storage: Default::default(),
        }
    }
}
