// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "Another substance that is a component of this substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceIngredientSubstance {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "A substance can be composed of other substances."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceIngredient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The amount of the ingredient in the substance - a concentration ratio."]
    pub r#quantity: Option<Box<super::super::types::Ratio>>,
    #[doc = "Another substance that is a component of this substance."]
    pub r#substance: SubstanceIngredientSubstance,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceIngredient {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#quantity: Default::default(),
            r#substance: Default::default(),
        }
    }
}
#[doc = "A homogeneous material with a definite composition."]
#[derive(Debug, Clone, PartialEq)]
pub struct Substance {
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
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique identifier for the substance. For an instance, an identifier associated with the package/container (usually a label affixed directly)."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A boolean to indicate if this an instance of a substance or a kind of one (a definition)."]
    pub r#instance: super::super::types::Boolean,
    #[doc = "A code to indicate if the substance is actively used."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "A code that classifies the general type of substance.  This is used  for searching, sorting and display purposes."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "A code (or set of codes) that identify this substance."]
    pub r#code: Box<super::super::types::CodeableReference>,
    #[doc = "A description of the substance - its appearance, handling requirements, and other usage notes."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "When the substance is no longer valid to use. For some substances, a single arbitrary date is used for expiry."]
    pub r#expiry: Option<super::super::types::DateTime>,
    #[doc = "The amount of the substance."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A substance can be composed of other substances."]
    pub r#ingredient: Vec<SubstanceIngredient>,
}
#[allow(clippy::derivable_impls)]
impl Default for Substance {
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
            r#instance: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#status: Default::default(),
            r#category: Default::default(),
            r#code: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#description: Default::default(),
            r#expiry: Default::default(),
            r#quantity: Default::default(),
            r#ingredient: Default::default(),
        }
    }
}
