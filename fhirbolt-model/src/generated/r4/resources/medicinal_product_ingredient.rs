// Generated on 2023-04-14 by fhirbolt-codegen v0.1.0
#[doc = "Strength expressed in terms of a reference substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Relevant reference substance."]
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Strength expressed in terms of a reference substance."]
    pub r#strength: Box<super::super::types::Ratio>,
    #[doc = "Strength expressed in terms of a reference substance."]
    pub r#strength_low_limit: Option<Box<super::super::types::Ratio>>,
    #[doc = "For when strength is measured at a particular point or distance."]
    pub r#measurement_point: Option<super::super::types::String>,
    #[doc = "The country or countries for which the strength range applies."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrength {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item."]
    pub r#presentation: Box<super::super::types::Ratio>,
    #[doc = "A lower limit for the quantity of substance in the unit of presentation. For use when there is a range of strengths, this is the lower limit, with the presentation attribute becoming the upper limit."]
    pub r#presentation_low_limit: Option<Box<super::super::types::Ratio>>,
    #[doc = "The strength per unitary volume (or mass)."]
    pub r#concentration: Option<Box<super::super::types::Ratio>>,
    #[doc = "A lower limit for the strength per unitary volume (or mass), for when there is a range. The concentration attribute then becomes the upper limit."]
    pub r#concentration_low_limit: Option<Box<super::super::types::Ratio>>,
    #[doc = "For when strength is measured at a particular point or distance."]
    pub r#measurement_point: Option<super::super::types::String>,
    #[doc = "The country or countries for which the strength range applies."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Strength expressed in terms of a reference substance."]
    pub r#reference_strength:
        Vec<MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength>,
}
#[doc = "A specified substance that comprises this ingredient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductIngredientSpecifiedSubstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specified substance."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The group of specified substance, e.g. group 1 to 4."]
    pub r#group: Box<super::super::types::CodeableConcept>,
    #[doc = "Confidentiality level of the specified substance as the ingredient."]
    pub r#confidentiality: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product."]
    pub r#strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
}
#[doc = "The ingredient substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductIngredientSubstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The ingredient substance."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product."]
    pub r#strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
}
#[doc = "An ingredient of a manufactured item or pharmaceutical product."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductIngredient {
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
    #[doc = "The identifier(s) of this Ingredient that are assigned by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Ingredient role e.g. Active ingredient, excipient."]
    pub r#role: Box<super::super::types::CodeableConcept>,
    #[doc = "If the ingredient is a known or suspected allergen."]
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
    #[doc = "Manufacturer of this Ingredient."]
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    #[doc = "A specified substance that comprises this ingredient."]
    pub r#specified_substance: Vec<MedicinalProductIngredientSpecifiedSubstance>,
    #[doc = "The ingredient substance."]
    pub r#substance: Option<MedicinalProductIngredientSubstance>,
}
