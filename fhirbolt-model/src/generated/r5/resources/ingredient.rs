// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. Unit of presentation refers to the quantity that the item occurs in e.g. a strength per tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally normalized as a unitary unit, which would be 'per mg')."]
#[derive(Debug, Clone, PartialEq)]
pub enum IngredientSubstanceStrengthPresentation {
    Ratio(Box<super::super::types::Ratio>),
    RatioRange(Box<super::super::types::RatioRange>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for IngredientSubstanceStrengthPresentation {
    fn default() -> IngredientSubstanceStrengthPresentation {
        IngredientSubstanceStrengthPresentation::Invalid
    }
}
#[doc = "The strength per unitary volume (or mass)."]
#[derive(Debug, Clone, PartialEq)]
pub enum IngredientSubstanceStrengthConcentration {
    Ratio(Box<super::super::types::Ratio>),
    RatioRange(Box<super::super::types::RatioRange>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for IngredientSubstanceStrengthConcentration {
    fn default() -> IngredientSubstanceStrengthConcentration {
        IngredientSubstanceStrengthConcentration::Invalid
    }
}
#[doc = "Strength expressed in terms of a reference substance."]
#[derive(Debug, Clone, PartialEq)]
pub enum IngredientSubstanceStrengthReferenceStrengthStrength {
    Ratio(Box<super::super::types::Ratio>),
    RatioRange(Box<super::super::types::RatioRange>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for IngredientSubstanceStrengthReferenceStrengthStrength {
    fn default() -> IngredientSubstanceStrengthReferenceStrengthStrength {
        IngredientSubstanceStrengthReferenceStrengthStrength::Invalid
    }
}
#[doc = "The organization(s) that manufacture this ingredient. Can be used to indicate:         1) Organizations we are aware of that manufacture this ingredient         2) Specific Manufacturer(s) currently being used         3) Set of organisations allowed to manufacture this ingredient for this product         Users must be clear on the application of context relevant to their use case."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IngredientManufacturer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The way in which this manufacturer is associated with the ingredient. For example whether it is a possible one (others allowed), or an exclusive authorized one for this ingredient. Note that this is not the manufacturing process role."]
    pub r#role: Option<super::super::types::Code>,
    #[doc = "An organization that manufactures this ingredient."]
    pub r#manufacturer: Box<super::super::types::Reference>,
}
#[doc = "Strength expressed in terms of a reference substance. For when the ingredient strength is additionally expressed as equivalent to the strength of some other closely related substance (e.g. salt vs. base). Reference strength represents the strength (quantitative composition) of the active moiety of the active substance. There are situations when the active substance and active moiety are different, therefore both a strength and a reference strength are needed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IngredientSubstanceStrengthReferenceStrength {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Relevant reference substance."]
    pub r#substance: Box<super::super::types::CodeableReference>,
    #[doc = "Strength expressed in terms of a reference substance."]
    pub r#strength: IngredientSubstanceStrengthReferenceStrengthStrength,
    #[doc = "For when strength is measured at a particular point or distance."]
    pub r#measurement_point: Option<super::super::types::String>,
    #[doc = "The country or countries for which the strength range applies."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
}
#[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. The allowed repetitions do not represent different strengths, but are different representations - mathematically equivalent - of a single strength."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IngredientSubstanceStrength {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. Unit of presentation refers to the quantity that the item occurs in e.g. a strength per tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally normalized as a unitary unit, which would be 'per mg')."]
    pub r#presentation: Option<IngredientSubstanceStrengthPresentation>,
    #[doc = "A textual represention of either the whole of the presentation strength or a part of it - with the rest being in Strength.presentation as a ratio."]
    pub r#text_presentation: Option<super::super::types::String>,
    #[doc = "The strength per unitary volume (or mass)."]
    pub r#concentration: Option<IngredientSubstanceStrengthConcentration>,
    #[doc = "A textual represention of either the whole of the concentration strength or a part of it - with the rest being in Strength.concentration as a ratio."]
    pub r#text_concentration: Option<super::super::types::String>,
    #[doc = "A code that indicates if the strength is, for example, based on the ingredient substance as stated or on the substance base (when the ingredient is a salt)."]
    pub r#basis: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "For when strength is measured at a particular point or distance. There are products where strength is measured at a particular point. For example, the strength of the ingredient in some inhalers is measured at a particular position relative to the point of aerosolization."]
    pub r#measurement_point: Option<super::super::types::String>,
    #[doc = "The country or countries for which the strength range applies."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Strength expressed in terms of a reference substance. For when the ingredient strength is additionally expressed as equivalent to the strength of some other closely related substance (e.g. salt vs. base). Reference strength represents the strength (quantitative composition) of the active moiety of the active substance. There are situations when the active substance and active moiety are different, therefore both a strength and a reference strength are needed."]
    pub r#reference_strength: Vec<IngredientSubstanceStrengthReferenceStrength>,
}
#[doc = "The substance that comprises this ingredient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IngredientSubstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code or full resource that represents the ingredient's substance."]
    pub r#code: Box<super::super::types::CodeableReference>,
    #[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. The allowed repetitions do not represent different strengths, but are different representations - mathematically equivalent - of a single strength."]
    pub r#strength: Vec<IngredientSubstanceStrength>,
}
#[doc = "An ingredient of a manufactured item or pharmaceutical product."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ingredient {
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
    #[doc = "The identifier(s) of this Ingredient that are assigned by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of this ingredient. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The product which this ingredient is a constituent part of."]
    pub r#for: Vec<Box<super::super::types::Reference>>,
    #[doc = "A classification of the ingredient identifying its purpose within the product, e.g. active, inactive."]
    pub r#role: Box<super::super::types::CodeableConcept>,
    #[doc = "A classification of the ingredient identifying its precise purpose(s) in the drug product. This extends the Ingredient.role to add more detail. Example: antioxidant, alkalizing agent."]
    pub r#function: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A classification of the ingredient according to where in the physical item it tends to be used, such the outer shell of a tablet, inner body or ink."]
    pub r#group: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If the ingredient is a known or suspected allergen. Note that this is a property of the substance, so if a reference to a SubstanceDefinition is used to decribe that (rather than just a code), the allergen information should go there, not here."]
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
    #[doc = "A place for providing any notes that are relevant to the component, e.g. removed during process, adjusted for loss on drying."]
    pub r#comment: Option<super::super::types::Markdown>,
    #[doc = "The organization(s) that manufacture this ingredient. Can be used to indicate:         1) Organizations we are aware of that manufacture this ingredient         2) Specific Manufacturer(s) currently being used         3) Set of organisations allowed to manufacture this ingredient for this product         Users must be clear on the application of context relevant to their use case."]
    pub r#manufacturer: Vec<IngredientManufacturer>,
    #[doc = "The substance that comprises this ingredient."]
    pub r#substance: IngredientSubstance,
}
