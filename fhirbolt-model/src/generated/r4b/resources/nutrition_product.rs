// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The actual characteristic value corresponding to the type."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum NutritionProductProductCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(super::super::types::String),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(super::super::types::Base64Binary),
    Attachment(Box<super::super::types::Attachment>),
    Boolean(super::super::types::Boolean),
    #[default]
    Invalid,
}
#[doc = "The product's nutritional information expressed by the nutrients."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductNutrient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The (relevant) nutrients in the product."]
    pub r#item: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The amount of nutrient expressed in one or more units: X per pack / per serving / per dose."]
    pub r#amount: Vec<super::super::types::Ratio>,
}
#[allow(clippy::derivable_impls)]
impl Default for NutritionProductNutrient {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "Ingredients contained in this product."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductIngredient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The ingredient contained in the product."]
    pub r#item: Box<super::super::types::CodeableReference>,
    #[doc = "The amount of ingredient that is in the product."]
    pub r#amount: Vec<super::super::types::Ratio>,
}
#[allow(clippy::derivable_impls)]
impl Default for NutritionProductIngredient {
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
#[doc = "Specifies descriptive properties of the nutrition product."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductProductCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code specifying which characteristic of the product is being described (for example, colour, shape)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The actual characteristic value corresponding to the type."]
    pub r#value: NutritionProductProductCharacteristicValue,
}
#[allow(clippy::derivable_impls)]
impl Default for NutritionProductProductCharacteristic {
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
#[doc = "Conveys instance-level information about this product item. One or several physical, countable instances or occurrences of the product."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProductInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The amount of items or instances that the resource considers, for instance when referring to 2 identical units together."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The identifier for the physical instance, typically a serial number."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identification of the batch or lot of the product."]
    pub r#lot_number: Option<super::super::types::String>,
    #[doc = "The time after which the product is no longer expected to be in proper condition, or its use is not advised or not allowed."]
    pub r#expiry: Option<super::super::types::DateTime>,
    #[doc = "The time after which the product is no longer expected to be in proper condition, or its use is not advised or not allowed."]
    pub r#use_by: Option<super::super::types::DateTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for NutritionProductInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#quantity: Default::default(),
            r#identifier: Default::default(),
            r#lot_number: Default::default(),
            r#expiry: Default::default(),
            r#use_by: Default::default(),
        }
    }
}
#[doc = "A food or fluid product that is consumed by patients."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionProduct {
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
    #[doc = "The current state of the product."]
    pub r#status: super::super::types::Code,
    #[doc = "Nutrition products can have different classifications - according to its nutritional properties, preparation methods, etc."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The code assigned to the product, for example a manufacturer number or other terminology."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The organisation (manufacturer, representative or legal authorisation holder) that is responsible for the device."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
    #[doc = "The product's nutritional information expressed by the nutrients."]
    pub r#nutrient: Vec<NutritionProductNutrient>,
    #[doc = "Ingredients contained in this product."]
    pub r#ingredient: Vec<NutritionProductIngredient>,
    #[doc = "Allergens that are known or suspected to be a part of this nutrition product."]
    pub r#known_allergen: Vec<super::super::types::CodeableReference>,
    #[doc = "Specifies descriptive properties of the nutrition product."]
    pub r#product_characteristic: Vec<NutritionProductProductCharacteristic>,
    #[doc = "Conveys instance-level information about this product item. One or several physical, countable instances or occurrences of the product."]
    pub r#instance: Option<NutritionProductInstance>,
    #[doc = "Comments made about the product."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for NutritionProduct {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Default::default(),
            r#code: Default::default(),
            r#manufacturer: Default::default(),
            r#nutrient: Default::default(),
            r#ingredient: Default::default(),
            r#known_allergen: Default::default(),
            r#product_characteristic: Default::default(),
            r#instance: Default::default(),
            r#note: Default::default(),
        }
    }
}
