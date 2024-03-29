// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Condition for which the medicinal use applies."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicinalProductSpecialDesignationIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Coding words or phrases of the name."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductNameNamePart {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A fragment of a product name."]
    pub r#part: super::super::types::String,
    #[doc = "Idenifying type for this part of the name (e.g. strength part)."]
    pub r#type: Box<super::super::types::Coding>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductNameNamePart {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#part: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Country where the name applies."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductNameCountryLanguage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Country code for where this name applies."]
    pub r#country: Box<super::super::types::CodeableConcept>,
    #[doc = "Jurisdiction code for where this name applies."]
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Language code for this name."]
    pub r#language: Box<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductNameCountryLanguage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#country: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#jurisdiction: Default::default(),
            r#language: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "The product's name, including full name and possibly coded parts."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The full product name."]
    pub r#product_name: super::super::types::String,
    #[doc = "Coding words or phrases of the name."]
    pub r#name_part: Vec<MedicinalProductNameNamePart>,
    #[doc = "Country where the name applies."]
    pub r#country_language: Vec<MedicinalProductNameCountryLanguage>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductName {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#product_name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#name_part: Default::default(),
            r#country_language: Default::default(),
        }
    }
}
#[doc = "An operation applied to the product, for manufacturing or adminsitrative purpose."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductManufacturingBusinessOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of manufacturing operation."]
    pub r#operation_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Regulatory authorization reference number."]
    pub r#authorisation_reference_number: Option<Box<super::super::types::Identifier>>,
    #[doc = "Regulatory authorization date."]
    pub r#effective_date: Option<super::super::types::DateTime>,
    #[doc = "To indicate if this proces is commercially confidential."]
    pub r#confidentiality_indicator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The manufacturer or establishment associated with the process."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
    #[doc = "A regulator which oversees the operation."]
    pub r#regulator: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductManufacturingBusinessOperation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#operation_type: Default::default(),
            r#authorisation_reference_number: Default::default(),
            r#effective_date: Default::default(),
            r#confidentiality_indicator: Default::default(),
            r#manufacturer: Default::default(),
            r#regulator: Default::default(),
        }
    }
}
#[doc = "Indicates if the medicinal product has an orphan designation for the treatment of a rare disease."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductSpecialDesignation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier for the designation, or procedure number."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The type of special designation, e.g. orphan drug, minor use."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The intended use of the product, e.g. prevention, treatment."]
    pub r#intended_use: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Condition for which the medicinal use applies."]
    pub r#indication: Option<MedicinalProductSpecialDesignationIndication>,
    #[doc = "For example granted, pending, expired or withdrawn."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date when the designation was granted."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Animal species for which this applies."]
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductSpecialDesignation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#type: Default::default(),
            r#intended_use: Default::default(),
            r#indication: Default::default(),
            r#status: Default::default(),
            r#date: Default::default(),
            r#species: Default::default(),
        }
    }
}
#[doc = "Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use)."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProduct {
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
    #[doc = "Business identifier for this product. Could be an MPID."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Regulatory type, e.g. Investigational or Authorized."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this medicine applies to human or veterinary uses."]
    pub r#domain: Option<Box<super::super::types::Coding>>,
    #[doc = "The dose form for a single part product, or combined form of a multiple part product."]
    pub r#combined_pharmaceutical_dose_form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The legal status of supply of the medicinal product as classified by the regulator."]
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the Medicinal Product is subject to additional monitoring for regulatory reasons."]
    pub r#additional_monitoring_indicator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the Medicinal Product is subject to special measures for regulatory reasons."]
    pub r#special_measures: Vec<super::super::types::String>,
    #[doc = "If authorised for use in children."]
    pub r#paediatric_use_indicator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Allows the product to be classified by various systems."]
    pub r#product_classification: Vec<super::super::types::CodeableConcept>,
    #[doc = "Marketing status of the medicinal product, in contrast to marketing authorizaton."]
    pub r#marketing_status: Vec<super::super::types::MarketingStatus>,
    #[doc = "Pharmaceutical aspects of product."]
    pub r#pharmaceutical_product: Vec<super::super::types::Reference>,
    #[doc = "Package representation for the product."]
    pub r#packaged_medicinal_product: Vec<super::super::types::Reference>,
    #[doc = "Supporting documentation, typically for regulatory submission."]
    pub r#attached_document: Vec<super::super::types::Reference>,
    #[doc = "A master file for to the medicinal product (e.g. Pharmacovigilance System Master File)."]
    pub r#master_file: Vec<super::super::types::Reference>,
    #[doc = "A product specific contact, person (in a role), or an organization."]
    pub r#contact: Vec<super::super::types::Reference>,
    #[doc = "Clinical trials or studies that this product is involved in."]
    pub r#clinical_trial: Vec<super::super::types::Reference>,
    #[doc = "The product's name, including full name and possibly coded parts."]
    pub r#name: Vec<MedicinalProductName>,
    #[doc = "Reference to another product, e.g. for linking authorised to investigational product."]
    pub r#cross_reference: Vec<super::super::types::Identifier>,
    #[doc = "An operation applied to the product, for manufacturing or adminsitrative purpose."]
    pub r#manufacturing_business_operation: Vec<MedicinalProductManufacturingBusinessOperation>,
    #[doc = "Indicates if the medicinal product has an orphan designation for the treatment of a rare disease."]
    pub r#special_designation: Vec<MedicinalProductSpecialDesignation>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProduct {
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
            r#domain: Default::default(),
            r#combined_pharmaceutical_dose_form: Default::default(),
            r#legal_status_of_supply: Default::default(),
            r#additional_monitoring_indicator: Default::default(),
            r#special_measures: Default::default(),
            r#paediatric_use_indicator: Default::default(),
            r#product_classification: Default::default(),
            r#marketing_status: Default::default(),
            r#pharmaceutical_product: Default::default(),
            r#packaged_medicinal_product: Default::default(),
            r#attached_document: Default::default(),
            r#master_file: Default::default(),
            r#contact: Default::default(),
            r#clinical_trial: Default::default(),
            r#name: Default::default(),
            r#cross_reference: Default::default(),
            r#manufacturing_business_operation: Default::default(),
            r#special_designation: Default::default(),
        }
    }
}
