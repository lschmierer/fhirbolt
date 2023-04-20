// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "The rate of administration of formula via a feeding pump, e.g. 60 mL per hour, according to the specified schedule."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum NutritionOrderEnteralFormulaAdministrationRate {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    #[default]
    Invalid,
}
#[doc = "Schedule information for an oral diet."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDietSchedule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The time period and frequency at which the diet should be given.  The diet should be given for the combination of all schedules if more than one schedule is present."]
    pub r#timing: Vec<Box<super::super::types::Timing>>,
    #[doc = "Indicates whether the product is only taken when needed within a specific dosing schedule."]
    pub r#as_needed: Option<super::super::types::Boolean>,
    #[doc = "Indicates whether the product is only taken based on a precondition for taking the product."]
    pub r#as_needed_for: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for NutritionOrderOralDietSchedule {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#timing: Default::default(),
            r#as_needed: Default::default(),
            r#as_needed_for: Default::default(),
        }
    }
}
#[doc = "Class that defines the quantity and type of nutrient modifications (for example carbohydrate, fiber or sodium) required for the oral diet."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDietNutrient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The nutrient that is being modified such as carbohydrate or sodium."]
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quantity of the specified nutrient to include in diet."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
impl Default for NutritionOrderOralDietNutrient {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#modifier: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "Class that describes any texture modifications required for the patient to safely consume various types of solid foods."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDietTexture {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Any texture modifications (for solid foods) that should be made, e.g. easy to chew, chopped, ground, and pureed."]
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The food type(s) (e.g. meats, all foods)  that the texture modification applies to.  This could be all foods types."]
    pub r#food_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for NutritionOrderOralDietTexture {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#modifier: Default::default(),
            r#food_type: Default::default(),
        }
    }
}
#[doc = "Diet given orally in contrast to enteral (tube) feeding."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDiet {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of diet or dietary restriction such as fiber restricted diet or diabetic diet."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Schedule information for an oral diet."]
    pub r#schedule: Option<NutritionOrderOralDietSchedule>,
    #[doc = "Class that defines the quantity and type of nutrient modifications (for example carbohydrate, fiber or sodium) required for the oral diet."]
    pub r#nutrient: Vec<NutritionOrderOralDietNutrient>,
    #[doc = "Class that describes any texture modifications required for the patient to safely consume various types of solid foods."]
    pub r#texture: Vec<NutritionOrderOralDietTexture>,
    #[doc = "The required consistency (e.g. honey-thick, nectar-thick, thin, thickened.) of liquids or fluids served to the patient."]
    pub r#fluid_consistency_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Free text or additional instructions or information pertaining to the oral diet."]
    pub r#instruction: Option<super::super::types::String>,
}
impl Default for NutritionOrderOralDiet {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#schedule: Default::default(),
            r#nutrient: Default::default(),
            r#texture: Default::default(),
            r#fluid_consistency_type: Default::default(),
            r#instruction: Default::default(),
        }
    }
}
#[doc = "Schedule information for a supplement."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderSupplementSchedule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The time period and frequency at which the supplement should be given.  The supplement should be given for the combination of all schedules if more than one schedule is present."]
    pub r#timing: Vec<Box<super::super::types::Timing>>,
    #[doc = "Indicates whether the supplement is only taken when needed within a specific dosing schedule."]
    pub r#as_needed: Option<super::super::types::Boolean>,
    #[doc = "Indicates whether the supplement is only taken based on a precondition for taking the supplement."]
    pub r#as_needed_for: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for NutritionOrderSupplementSchedule {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#timing: Default::default(),
            r#as_needed: Default::default(),
            r#as_needed_for: Default::default(),
        }
    }
}
#[doc = "Oral nutritional products given in order to add further nutritional value to the patient's diet."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderSupplement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of nutritional supplement product required such as a high protein or pediatric clear liquid supplement."]
    pub r#type: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The product or brand name of the nutritional supplement such as \"Acme Protein Shake\"."]
    pub r#product_name: Option<super::super::types::String>,
    #[doc = "Schedule information for a supplement."]
    pub r#schedule: Option<NutritionOrderSupplementSchedule>,
    #[doc = "The amount of the nutritional supplement to be given."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Free text or additional instructions or information pertaining to the oral supplement."]
    pub r#instruction: Option<super::super::types::String>,
}
impl Default for NutritionOrderSupplement {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#product_name: Default::default(),
            r#schedule: Default::default(),
            r#quantity: Default::default(),
            r#instruction: Default::default(),
        }
    }
}
#[doc = "Indicates modular components to be provided in addition or mixed with the base formula."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderEnteralFormulaAdditive {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates the type of modular component such as protein, carbohydrate, fat or fiber to be provided in addition to or mixed with the base formula."]
    pub r#type: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The product or brand name of the type of modular component to be added to the formula."]
    pub r#product_name: Option<super::super::types::String>,
    #[doc = "The amount of additive to be given in addition or to be mixed in with the base formula."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
}
impl Default for NutritionOrderEnteralFormulaAdditive {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#product_name: Default::default(),
            r#quantity: Default::default(),
        }
    }
}
#[doc = "Schedule information for an enteral formula."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderEnteralFormulaAdministrationSchedule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The time period and frequency at which the enteral formula should be given.  The enteral formula should be given for the combination of all schedules if more than one schedule is present."]
    pub r#timing: Vec<Box<super::super::types::Timing>>,
    #[doc = "Indicates whether the enteral formula is only taken when needed within a specific dosing schedule."]
    pub r#as_needed: Option<super::super::types::Boolean>,
    #[doc = "Indicates whether the enteral formula is only taken based on a precondition for taking the enteral formula."]
    pub r#as_needed_for: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for NutritionOrderEnteralFormulaAdministrationSchedule {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#timing: Default::default(),
            r#as_needed: Default::default(),
            r#as_needed_for: Default::default(),
        }
    }
}
#[doc = "Formula administration instructions as structured data.  This repeating structure allows for changing the administration rate or volume over time for both bolus and continuous feeding.  An example of this would be an instruction to increase the rate of continuous feeding every 2 hours."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderEnteralFormulaAdministration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Schedule information for an enteral formula."]
    pub r#schedule: Option<NutritionOrderEnteralFormulaAdministrationSchedule>,
    #[doc = "The volume of formula to provide to the patient per the specified administration schedule."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The rate of administration of formula via a feeding pump, e.g. 60 mL per hour, according to the specified schedule."]
    pub r#rate: Option<NutritionOrderEnteralFormulaAdministrationRate>,
}
impl Default for NutritionOrderEnteralFormulaAdministration {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#schedule: Default::default(),
            r#quantity: Default::default(),
            r#rate: Default::default(),
        }
    }
}
#[doc = "Feeding provided through the gastrointestinal tract via a tube, catheter, or stoma that delivers nutrition distal to the oral cavity."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrderEnteralFormula {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of enteral or infant formula such as an adult standard formula with fiber or a soy-based infant formula."]
    pub r#base_formula_type: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The product or brand name of the enteral or infant formula product such as \"ACME Adult Standard Formula\"."]
    pub r#base_formula_product_name: Option<super::super::types::String>,
    #[doc = "The intended type of device that is to be used for the administration of the enteral formula."]
    pub r#delivery_device: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Indicates modular components to be provided in addition or mixed with the base formula."]
    pub r#additive: Vec<NutritionOrderEnteralFormulaAdditive>,
    #[doc = "The amount of energy (calories) that the formula should provide per specified volume, typically per mL or fluid oz.  For example, an infant may require a formula that provides 24 calories per fluid ounce or an adult may require an enteral formula that provides 1.5 calorie/mL."]
    pub r#caloric_density: Option<Box<super::super::types::Quantity>>,
    #[doc = "The route or physiological path of administration into the patient's gastrointestinal  tract for purposes of providing the formula feeding, e.g. nasogastric tube."]
    pub r#route_of_administration: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Formula administration instructions as structured data.  This repeating structure allows for changing the administration rate or volume over time for both bolus and continuous feeding.  An example of this would be an instruction to increase the rate of continuous feeding every 2 hours."]
    pub r#administration: Vec<NutritionOrderEnteralFormulaAdministration>,
    #[doc = "The maximum total quantity of formula that may be administered to a subject over the period of time, e.g. 1440 mL over 24 hours."]
    pub r#max_volume_to_deliver: Option<Box<super::super::types::Quantity>>,
    #[doc = "Free text formula administration, feeding instructions or additional instructions or information."]
    pub r#administration_instruction: Option<super::super::types::Markdown>,
}
impl Default for NutritionOrderEnteralFormula {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#base_formula_type: Default::default(),
            r#base_formula_product_name: Default::default(),
            r#delivery_device: Default::default(),
            r#additive: Default::default(),
            r#caloric_density: Default::default(),
            r#route_of_administration: Default::default(),
            r#administration: Default::default(),
            r#max_volume_to_deliver: Default::default(),
            r#administration_instruction: Default::default(),
        }
    }
}
#[doc = "A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionOrder {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers assigned to this order by the order sender or by the order receiver."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "The URL pointing to a protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates: Vec<super::super::types::Uri>,
    #[doc = "A plan or request that is fulfilled in whole or in part by this nutrition order."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A shared identifier common to all nutrition orders that were authorized more or less simultaneously by a single author, representing the composite or group identifier."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The workflow status of the nutrition order/request."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the NutrionOrder and where the request fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates how quickly the Nutrition Order should be addressed with respect to other        requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "The person or set of individuals who needs the nutrition order for an oral diet, nutritional supplement and/or enteral or formula feeding."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "An encounter that provides additional information about the healthcare context in which this request is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Information to support fulfilling (i.e. dispensing or administering) of the nutrition,        for example, patient height and weight)."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date and time that this nutrition order was requested."]
    pub r#date_time: super::super::types::DateTime,
    #[doc = "The practitioner that holds legal responsibility for ordering the diet, nutritional supplement, or formula feedings."]
    pub r#orderer: Option<Box<super::super::types::Reference>>,
    #[doc = "The specified desired performer of the nutrition order."]
    pub r#performer: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "A link to a record of allergies or intolerances  which should be included in the nutrition order."]
    pub r#allergy_intolerance: Vec<Box<super::super::types::Reference>>,
    #[doc = "This modifier is used to convey order-specific modifiers about the type of food that should be given. These can be derived from patient allergies, intolerances, or preferences such as Halal, Vegan or Kosher. This modifier applies to the entire nutrition order inclusive of the oral diet, nutritional supplements and enteral formula feedings."]
    pub r#food_preference_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "This modifier is used to convey Order-specific modifier about the type of oral food or oral fluids that should not be given. These can be derived from patient allergies, intolerances, or preferences such as No Red Meat, No Soy or No Wheat or  Gluten-Free.  While it should not be necessary to repeat allergy or intolerance information captured in the referenced AllergyIntolerance resource in the excludeFoodModifier, this element may be used to convey additional specificity related to foods that should be eliminated from the patient’s diet for any reason.  This modifier applies to the entire nutrition order inclusive of the oral diet, nutritional supplements and enteral formula feedings."]
    pub r#exclude_food_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "This modifier is used to convey whether a food item is allowed to be brought in by the patient and/or family.  If set to true, indicates that the receiving system does not need to supply the food item."]
    pub r#outside_food_allowed: Option<super::super::types::Boolean>,
    #[doc = "Diet given orally in contrast to enteral (tube) feeding."]
    pub r#oral_diet: Option<NutritionOrderOralDiet>,
    #[doc = "Oral nutritional products given in order to add further nutritional value to the patient's diet."]
    pub r#supplement: Vec<NutritionOrderSupplement>,
    #[doc = "Feeding provided through the gastrointestinal tract via a tube, catheter, or stoma that delivers nutrition distal to the oral cavity."]
    pub r#enteral_formula: Option<NutritionOrderEnteralFormula>,
    #[doc = "Comments made about the {{title}} by the requester, performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl Default for NutritionOrder {
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
            r#instantiates_canonical: Default::default(),
            r#instantiates_uri: Default::default(),
            r#instantiates: Default::default(),
            r#based_on: Default::default(),
            r#group_identifier: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#intent: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#priority: Default::default(),
            r#subject: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#encounter: Default::default(),
            r#supporting_information: Default::default(),
            r#date_time: {
                let mut default: super::super::types::DateTime = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#orderer: Default::default(),
            r#performer: Default::default(),
            r#allergy_intolerance: Default::default(),
            r#food_preference_modifier: Default::default(),
            r#exclude_food_modifier: Default::default(),
            r#outside_food_allowed: Default::default(),
            r#oral_diet: Default::default(),
            r#supplement: Default::default(),
            r#enteral_formula: Default::default(),
            r#note: Default::default(),
        }
    }
}
