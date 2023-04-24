// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The interval of time during which it is being asserted that the patient is/was consuming the food or fluid."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum NutritionIntakeOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "The person or organization that provided the information about the consumption of this food or fluid. Note: Use derivedFrom when a NutritionIntake is derived from other resources."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum NutritionIntakeReported {
    Boolean(Box<super::super::types::Boolean>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "What food or fluid product or item was consumed."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntakeConsumedItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates what a category of item that was consumed: e.g., food, fluid, enteral, etc."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the food or fluid product that was consumed. This is potentially a link to a resource representing the details of the food product (TBD) or a simple attribute carrying a code that identifies the food from a known list of foods."]
    pub r#nutrition_product: Box<super::super::types::CodeableReference>,
    #[doc = "Scheduled frequency of consumption."]
    pub r#schedule: Option<Box<super::super::types::Timing>>,
    #[doc = "Quantity of the specified food."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    #[doc = "Rate at which enteral feeding was administered."]
    pub r#rate: Option<Box<super::super::types::Quantity>>,
    #[doc = "Indicator when a patient is in a setting where it is helpful to know if food was not consumed, such as it was refused, held (as in tube feedings), or otherwise not provided. If a consumption is being recorded from an app, such as MyFitnessPal, this indicator will likely not be used."]
    pub r#not_consumed: Option<super::super::types::Boolean>,
    #[doc = "Document the reason the food or fluid was not consumed, such as refused, held, etc."]
    pub r#not_consumed_reason: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for NutritionIntakeConsumedItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#nutrition_product: {
                let mut default: Box<super::super::types::CodeableReference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#schedule: Default::default(),
            r#amount: Default::default(),
            r#rate: Default::default(),
            r#not_consumed: Default::default(),
            r#not_consumed_reason: Default::default(),
        }
    }
}
#[doc = "Total nutrient amounts for the whole meal, product, serving, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntakeIngredientLabel {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Total nutrient consumed. This could be a macronutrient (protein, fat, carbohydrate), or a vitamin and mineral."]
    pub r#nutrient: Box<super::super::types::CodeableReference>,
    #[doc = "Total amount of nutrient consumed."]
    pub r#amount: Box<super::super::types::Quantity>,
}
impl Default for NutritionIntakeIngredientLabel {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#nutrient: {
                let mut default: Box<super::super::types::CodeableReference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#amount: {
                let mut default: Box<super::super::types::Quantity> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "Who performed the intake and how they were involved."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntakePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of performer."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Who performed the intake."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl Default for NutritionIntakePerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "A record of food or fluid that is being consumed by a patient.  A NutritionIntake may indicate that the patient may be consuming the food or fluid now or has consumed the food or fluid in the past.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay or through an app that tracks food or fluids consumed.   The consumption information may come from sources such as the patient's memory, from a nutrition label,  or from a clinician documenting observed intake."]
#[derive(Debug, Clone, PartialEq)]
pub struct NutritionIntake {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers associated with this Nutrition Intake that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Instantiates FHIR protocol or definition."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "Instantiates external protocol or definition."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this event."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code representing the patient or other source's judgment about the state of the intake that this assertion is about.  Generally, this will be active or completed."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the NutritionIntake."]
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Overall type of nutrition intake."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person, animal or group who is/was consuming the food or fluid."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The encounter that establishes the context for this NutritionIntake."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The interval of time during which it is being asserted that the patient is/was consuming the food or fluid."]
    pub r#occurrence: Option<NutritionIntakeOccurrence>,
    #[doc = "The date when the Nutrition Intake was asserted by the information source."]
    pub r#recorded: Option<super::super::types::DateTime>,
    #[doc = "The person or organization that provided the information about the consumption of this food or fluid. Note: Use derivedFrom when a NutritionIntake is derived from other resources."]
    pub r#reported: Option<NutritionIntakeReported>,
    #[doc = "What food or fluid product or item was consumed."]
    pub r#consumed_item: Vec<NutritionIntakeConsumedItem>,
    #[doc = "Total nutrient amounts for the whole meal, product, serving, etc."]
    pub r#ingredient_label: Vec<NutritionIntakeIngredientLabel>,
    #[doc = "Who performed the intake and how they were involved."]
    pub r#performer: Vec<NutritionIntakePerformer>,
    #[doc = "Where the intake occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Allows linking the NutritionIntake to the underlying NutritionOrder, or to other information, such as AllergyIntolerance, that supports or is used to derive the NutritionIntake."]
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    #[doc = "A reason, Condition or observation for why the food or fluid is /was consumed."]
    pub r#reason: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Provides extra information about the Nutrition Intake that is not conveyed by the other attributes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl Default for NutritionIntake {
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
            r#based_on: Default::default(),
            r#part_of: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status_reason: Default::default(),
            r#code: Default::default(),
            r#subject: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#recorded: Default::default(),
            r#reported: Default::default(),
            r#consumed_item: Default::default(),
            r#ingredient_label: Default::default(),
            r#performer: Default::default(),
            r#location: Default::default(),
            r#derived_from: Default::default(),
            r#reason: Default::default(),
            r#note: Default::default(),
        }
    }
}
