// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
#[doc = "The specific medication, food or laboratory test that interacts."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicinalProductInteractionInteractantItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "The specific medication, food or laboratory test that interacts."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductInteractionInteractant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The specific medication, food or laboratory test that interacts."]
    pub r#item: MedicinalProductInteractionInteractantItem,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductInteractionInteractant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "The interactions of the medicinal product with other medicinal products, or other forms of interactions."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductInteraction {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The medication for which this is a described interaction."]
    pub r#subject: Vec<super::super::types::Reference>,
    #[doc = "The interaction described."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The specific medication, food or laboratory test that interacts."]
    pub r#interactant: Vec<MedicinalProductInteractionInteractant>,
    #[doc = "The type of the interaction e.g. drug-drug interaction, drug-food interaction, drug-lab test interaction."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The effect of the interaction, for example \"reduced gastric absorption of primary medication\"."]
    pub r#effect: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The incidence of the interaction, e.g. theoretical, observed."]
    pub r#incidence: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Actions for managing the interaction."]
    pub r#management: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductInteraction {
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
            r#subject: Default::default(),
            r#description: Default::default(),
            r#interactant: Default::default(),
            r#type: Default::default(),
            r#effect: Default::default(),
            r#incidence: Default::default(),
            r#management: Default::default(),
        }
    }
}
