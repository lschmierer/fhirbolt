// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "A value for the characteristic.text."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicinalProductDefinitionCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Markdown(Box<super::super::types::Markdown>),
    Quantity(Box<super::super::types::Quantity>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    Boolean(Box<super::super::types::Boolean>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "A product specific contact, person (in a role), or an organization."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionContact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Allows the contact to be classified, for example QPPV, Pharmacovigilance Enquiry Information."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A product specific contact, person (in a role), or an organization."]
    pub r#contact: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinitionContact {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#contact: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Coding words or phrases of the name."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionNamePart {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A fragment of a product name."]
    pub r#part: super::super::types::String,
    #[doc = "Identifying type for this part of the name (e.g. strength part)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinitionNamePart {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#part: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Country and jurisdiction where the name applies, and associated language."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionNameUsage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Country code for where this name applies."]
    pub r#country: Box<super::super::types::CodeableConcept>,
    #[doc = "Jurisdiction code for where this name applies. A jurisdiction may be a sub- or supra-national entity (e.g. a state or a geographic region)."]
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Language code for this name."]
    pub r#language: Box<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinitionNameUsage {
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
pub struct MedicinalProductDefinitionName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The full product name."]
    pub r#product_name: super::super::types::String,
    #[doc = "Type of product name, such as rINN, BAN, Proprietary, Non-Proprietary."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Coding words or phrases of the name."]
    pub r#part: Vec<MedicinalProductDefinitionNamePart>,
    #[doc = "Country and jurisdiction where the name applies, and associated language."]
    pub r#usage: Vec<MedicinalProductDefinitionNameUsage>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinitionName {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#product_name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Default::default(),
            r#part: Default::default(),
            r#usage: Default::default(),
        }
    }
}
#[doc = "Reference to another product, e.g. for linking authorised to investigational product, or a virtual product."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionCrossReference {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Reference to another product, e.g. for linking authorised to investigational product."]
    pub r#product: Box<super::super::types::CodeableReference>,
    #[doc = "The type of relationship, for instance branded to generic, virtual to actual product, product to development product (investigational), parallel import version."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinitionCrossReference {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#product: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#type: Default::default(),
        }
    }
}
#[doc = "A manufacturing or administrative process or step associated with (or performed on) the medicinal product."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of manufacturing operation e.g. manufacturing itself, re-packaging. For the authorization of this, a RegulatedAuthorization would point to the same plan or activity referenced here."]
    pub r#type: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Date range of applicability."]
    pub r#effective_date: Option<Box<super::super::types::Period>>,
    #[doc = "The organization or establishment responsible for (or associated with) the particular process or step, examples include the manufacturer, importer, agent."]
    pub r#organization: Vec<super::super::types::Reference>,
    #[doc = "Specifies whether this particular business or manufacturing process is considered proprietary or confidential."]
    pub r#confidentiality_indicator: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinitionOperation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#effective_date: Default::default(),
            r#organization: Default::default(),
            r#confidentiality_indicator: Default::default(),
        }
    }
}
#[doc = "Allows the key product features to be recorded, such as \"sugar free\", \"modified release\", \"parallel import\"."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinitionCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code expressing the type of characteristic."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the characteristic.text."]
    pub r#value: Option<MedicinalProductDefinitionCharacteristicValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinitionCharacteristic {
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
#[doc = "Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.)."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductDefinition {
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
    #[doc = "Business identifier for this product. Could be an MPID. When in development or being regulated, products are typically referenced by official identifiers, assigned by a manufacturer or regulator, and unique to a product (which, when compared to a product instance being prescribed, is actually a product type). See also MedicinalProductDefinition.code."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Regulatory type, e.g. Investigational or Authorized."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this medicine applies to human or veterinary uses."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A business identifier relating to a specific version of the product, this is commonly used to support revisions to an existing product."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "The status within the lifecycle of this product record. A high-level status, this is not intended to duplicate details carried elsewhere such as legal status, or authorization status."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the given status became applicable."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "General description of this product."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The dose form for a single part product, or combined form of a multiple part product. This is one concept that describes all the components. It does not represent the form with components physically mixed, if that might be necessary, for which see (AdministrableProductDefinition.administrableDoseForm)."]
    pub r#combined_pharmaceutical_dose_form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The path by which the product is taken into or makes contact with the body. In some regions this is referred to as the licenced or approved route. See also AdministrableProductDefinition resource. MedicinalProductDefinition.route is the same concept as AdministrableProductDefinition.routeOfAdministration.code, and they cannot be used together."]
    pub r#route: Vec<super::super::types::CodeableConcept>,
    #[doc = "Description of indication(s) for this product, used when structured indications are not required. In cases where structured indications are required, they are captured using the ClinicalUseDefinition resource. An indication is a medical situation for which using the product is appropriate."]
    pub r#indication: Option<super::super::types::Markdown>,
    #[doc = "The legal status of supply of the medicinal product as classified by the regulator."]
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the Medicinal Product is subject to additional monitoring for regulatory reasons, such as heightened reporting requirements."]
    pub r#additional_monitoring_indicator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the Medicinal Product is subject to special measures for regulatory reasons, such as a requirement to conduct post-authorization studies."]
    pub r#special_measures: Vec<super::super::types::CodeableConcept>,
    #[doc = "If authorised for use in children, or infants, neonates etc."]
    pub r#pediatric_use_indicator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Allows the product to be classified by various systems, commonly WHO ATC."]
    pub r#classification: Vec<super::super::types::CodeableConcept>,
    #[doc = "Marketing status of the medicinal product, in contrast to marketing authorization. This refers to the product being actually 'on the market' as opposed to being allowed to be on the market (which is an authorization)."]
    pub r#marketing_status: Vec<super::super::types::MarketingStatus>,
    #[doc = "Package type for the product. See also the PackagedProductDefinition resource."]
    pub r#packaged_medicinal_product: Vec<super::super::types::CodeableConcept>,
    #[doc = "Types of medicinal manufactured items and/or devices that this product consists of, such as tablets, capsule, or syringes. Used as a direct link when the item's packaging is not being recorded (see also PackagedProductDefinition.package.containedItem.item)."]
    pub r#comprised_of: Vec<super::super::types::Reference>,
    #[doc = "The ingredients of this medicinal product - when not detailed in other resources. This is only needed if the ingredients are not specified by incoming references from the Ingredient resource, or indirectly via incoming AdministrableProductDefinition, PackagedProductDefinition or ManufacturedItemDefinition references. In cases where those levels of detail are not used, the ingredients may be specified directly here as codes."]
    pub r#ingredient: Vec<super::super::types::CodeableConcept>,
    #[doc = "Any component of the drug product which is not the chemical entity defined as the drug substance, or an excipient in the drug product. This includes process-related impurities and contaminants, product-related impurities including degradation products."]
    pub r#impurity: Vec<super::super::types::CodeableReference>,
    #[doc = "Additional information or supporting documentation about the medicinal product."]
    pub r#attached_document: Vec<super::super::types::Reference>,
    #[doc = "A master file for the medicinal product (e.g. Pharmacovigilance System Master File). Drug master files (DMFs) are documents submitted to regulatory agencies to provide confidential detailed information about facilities, processes or articles used in the manufacturing, processing, packaging and storing of drug products."]
    pub r#master_file: Vec<super::super::types::Reference>,
    #[doc = "A product specific contact, person (in a role), or an organization."]
    pub r#contact: Vec<MedicinalProductDefinitionContact>,
    #[doc = "Clinical trials or studies that this product is involved in."]
    pub r#clinical_trial: Vec<super::super::types::Reference>,
    #[doc = "A code that this product is known by, usually within some formal terminology, perhaps assigned by a third party (i.e. not the manufacturer or regulator). Products (types of medications) tend to be known by identifiers during development and within regulatory process. However when they are prescribed they tend to be identified by codes. The same product may be have multiple codes, applied to it by multiple organizations."]
    pub r#code: Vec<super::super::types::Coding>,
    #[doc = "The product's name, including full name and possibly coded parts."]
    pub r#name: Vec<MedicinalProductDefinitionName>,
    #[doc = "Reference to another product, e.g. for linking authorised to investigational product, or a virtual product."]
    pub r#cross_reference: Vec<MedicinalProductDefinitionCrossReference>,
    #[doc = "A manufacturing or administrative process or step associated with (or performed on) the medicinal product."]
    pub r#operation: Vec<MedicinalProductDefinitionOperation>,
    #[doc = "Allows the key product features to be recorded, such as \"sugar free\", \"modified release\", \"parallel import\"."]
    pub r#characteristic: Vec<MedicinalProductDefinitionCharacteristic>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductDefinition {
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
            r#version: Default::default(),
            r#status: Default::default(),
            r#status_date: Default::default(),
            r#description: Default::default(),
            r#combined_pharmaceutical_dose_form: Default::default(),
            r#route: Default::default(),
            r#indication: Default::default(),
            r#legal_status_of_supply: Default::default(),
            r#additional_monitoring_indicator: Default::default(),
            r#special_measures: Default::default(),
            r#pediatric_use_indicator: Default::default(),
            r#classification: Default::default(),
            r#marketing_status: Default::default(),
            r#packaged_medicinal_product: Default::default(),
            r#comprised_of: Default::default(),
            r#ingredient: Default::default(),
            r#impurity: Default::default(),
            r#attached_document: Default::default(),
            r#master_file: Default::default(),
            r#contact: Default::default(),
            r#clinical_trial: Default::default(),
            r#code: Default::default(),
            r#name: Default::default(),
            r#cross_reference: Default::default(),
            r#operation: Default::default(),
            r#characteristic: Default::default(),
        }
    }
}
