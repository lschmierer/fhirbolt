// Generated on 2023-04-14 by fhirbolt-codegen v0.1.0
#[doc = "The rate of administration of formula via a feeding pump, e.g. 60 mL per hour, according to the specified schedule."]
#[derive(Debug, Clone, PartialEq)]
pub enum NutritionOrderEnteralFormulaAdministrationRate {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Invalid,
}
impl Default for NutritionOrderEnteralFormulaAdministrationRate {
    fn default() -> NutritionOrderEnteralFormulaAdministrationRate {
        NutritionOrderEnteralFormulaAdministrationRate::Invalid
    }
}
#[doc = "Class that defines the quantity and type of nutrient modifications (for example carbohydrate, fiber or sodium) required for the oral diet."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDietNutrient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The nutrient that is being modified such as carbohydrate or sodium."]
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quantity of the specified nutrient to include in diet."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
#[doc = "Class that describes any texture modifications required for the patient to safely consume various types of solid foods."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDietTexture {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Any texture modifications (for solid foods) that should be made, e.g. easy to chew, chopped, ground, and pureed."]
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The food type(s) (e.g. meats, all foods)  that the texture modification applies to.  This could be all foods types."]
    pub r#food_type: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Diet given orally in contrast to enteral (tube) feeding."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NutritionOrderOralDiet {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of diet or dietary restriction such as fiber restricted diet or diabetic diet."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The time period and frequency at which the diet should be given.  The diet should be given for the combination of all schedules if more than one schedule is present."]
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    #[doc = "Class that defines the quantity and type of nutrient modifications (for example carbohydrate, fiber or sodium) required for the oral diet."]
    pub r#nutrient: Vec<NutritionOrderOralDietNutrient>,
    #[doc = "Class that describes any texture modifications required for the patient to safely consume various types of solid foods."]
    pub r#texture: Vec<NutritionOrderOralDietTexture>,
    #[doc = "The required consistency (e.g. honey-thick, nectar-thick, thin, thickened.) of liquids or fluids served to the patient."]
    pub r#fluid_consistency_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Free text or additional instructions or information pertaining to the oral diet."]
    pub r#instruction: Option<super::super::types::String>,
}
#[doc = "Oral nutritional products given in order to add further nutritional value to the patient's diet."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NutritionOrderSupplement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of nutritional supplement product required such as a high protein or pediatric clear liquid supplement."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The product or brand name of the nutritional supplement such as \"Acme Protein Shake\"."]
    pub r#product_name: Option<super::super::types::String>,
    #[doc = "The time period and frequency at which the supplement(s) should be given.  The supplement should be given for the combination of all schedules if more than one schedule is present."]
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    #[doc = "The amount of the nutritional supplement to be given."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Free text or additional instructions or information pertaining to the oral supplement."]
    pub r#instruction: Option<super::super::types::String>,
}
#[doc = "Formula administration instructions as structured data.  This repeating structure allows for changing the administration rate or volume over time for both bolus and continuous feeding.  An example of this would be an instruction to increase the rate of continuous feeding every 2 hours."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NutritionOrderEnteralFormulaAdministration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The time period and frequency at which the enteral formula should be delivered to the patient."]
    pub r#schedule: Option<Box<super::super::types::Timing>>,
    #[doc = "The volume of formula to provide to the patient per the specified administration schedule."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The rate of administration of formula via a feeding pump, e.g. 60 mL per hour, according to the specified schedule."]
    pub r#rate: Option<NutritionOrderEnteralFormulaAdministrationRate>,
}
#[doc = "Feeding provided through the gastrointestinal tract via a tube, catheter, or stoma that delivers nutrition distal to the oral cavity."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NutritionOrderEnteralFormula {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of enteral or infant formula such as an adult standard formula with fiber or a soy-based infant formula."]
    pub r#base_formula_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The product or brand name of the enteral or infant formula product such as \"ACME Adult Standard Formula\"."]
    pub r#base_formula_product_name: Option<super::super::types::String>,
    #[doc = "Indicates the type of modular component such as protein, carbohydrate, fat or fiber to be provided in addition to or mixed with the base formula."]
    pub r#additive_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The product or brand name of the type of modular component to be added to the formula."]
    pub r#additive_product_name: Option<super::super::types::String>,
    #[doc = "The amount of energy (calories) that the formula should provide per specified volume, typically per mL or fluid oz.  For example, an infant may require a formula that provides 24 calories per fluid ounce or an adult may require an enteral formula that provides 1.5 calorie/mL."]
    pub r#caloric_density: Option<Box<super::super::types::Quantity>>,
    #[doc = "The route or physiological path of administration into the patient's gastrointestinal  tract for purposes of providing the formula feeding, e.g. nasogastric tube."]
    pub r#routeof_administration: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Formula administration instructions as structured data.  This repeating structure allows for changing the administration rate or volume over time for both bolus and continuous feeding.  An example of this would be an instruction to increase the rate of continuous feeding every 2 hours."]
    pub r#administration: Vec<NutritionOrderEnteralFormulaAdministration>,
    #[doc = "The maximum total quantity of formula that may be administered to a subject over the period of time, e.g. 1440 mL over 24 hours."]
    pub r#max_volume_to_deliver: Option<Box<super::super::types::Quantity>>,
    #[doc = "Free text formula administration, feeding instructions or additional instructions or information."]
    pub r#administration_instruction: Option<super::super::types::String>,
}
#[doc = "A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers assigned to this order by the order sender or by the order receiver."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "The URL pointing to a protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates: Vec<super::super::types::Uri>,
    #[doc = "The workflow status of the nutrition order/request."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the NutrionOrder and where the request fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "The person (patient) who needs the nutrition order for an oral diet, nutritional supplement and/or enteral or formula feeding."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "An encounter that provides additional information about the healthcare context in which this request is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date and time that this nutrition order was requested."]
    pub r#date_time: super::super::types::DateTime,
    #[doc = "The practitioner that holds legal responsibility for ordering the diet, nutritional supplement, or formula feedings."]
    pub r#orderer: Option<Box<super::super::types::Reference>>,
    #[doc = "A link to a record of allergies or intolerances  which should be included in the nutrition order."]
    pub r#allergy_intolerance: Vec<Box<super::super::types::Reference>>,
    #[doc = "This modifier is used to convey order-specific modifiers about the type of food that should be given. These can be derived from patient allergies, intolerances, or preferences such as Halal, Vegan or Kosher. This modifier applies to the entire nutrition order inclusive of the oral diet, nutritional supplements and enteral formula feedings."]
    pub r#food_preference_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "This modifier is used to convey Order-specific modifier about the type of oral food or oral fluids that should not be given. These can be derived from patient allergies, intolerances, or preferences such as No Red Meat, No Soy or No Wheat or  Gluten-Free.  While it should not be necessary to repeat allergy or intolerance information captured in the referenced AllergyIntolerance resource in the excludeFoodModifier, this element may be used to convey additional specificity related to foods that should be eliminated from the patient’s diet for any reason.  This modifier applies to the entire nutrition order inclusive of the oral diet, nutritional supplements and enteral formula feedings."]
    pub r#exclude_food_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Diet given orally in contrast to enteral (tube) feeding."]
    pub r#oral_diet: Option<NutritionOrderOralDiet>,
    #[doc = "Oral nutritional products given in order to add further nutritional value to the patient's diet."]
    pub r#supplement: Vec<NutritionOrderSupplement>,
    #[doc = "Feeding provided through the gastrointestinal tract via a tube, catheter, or stoma that delivers nutrition distal to the oral cavity."]
    pub r#enteral_formula: Option<NutritionOrderEnteralFormula>,
    #[doc = "Comments made about the {{title}} by the requester, performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
