// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A value for the characteristic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ManufacturedItemDefinitionPropertyValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Date(super::super::types::Date),
    Boolean(super::super::types::Boolean),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "General characteristics of this item."]
#[derive(Debug, Clone, PartialEq)]
pub struct ManufacturedItemDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code expressing the type of characteristic."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the characteristic."]
    pub r#value: Option<ManufacturedItemDefinitionPropertyValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for ManufacturedItemDefinitionProperty {
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
#[doc = "The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product."]
#[derive(Debug, Clone, PartialEq)]
pub struct ManufacturedItemDefinition {
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
    #[doc = "Unique identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of this item. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "Dose form as manufactured and before any transformation into the pharmaceutical product."]
    pub r#manufactured_dose_form: Box<super::super::types::CodeableConcept>,
    #[doc = "The “real world” units in which the quantity of the manufactured item is described."]
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Manufacturer of the item (Note that this should be named \"manufacturer\" but it currently causes technical issues)."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
    #[doc = "The ingredients of this manufactured item. This is only needed if the ingredients are not specified by incoming references from the Ingredient resource."]
    pub r#ingredient: Vec<super::super::types::CodeableConcept>,
    #[doc = "General characteristics of this item."]
    pub r#property: Vec<ManufacturedItemDefinitionProperty>,
}
#[allow(clippy::derivable_impls)]
impl Default for ManufacturedItemDefinition {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#manufactured_dose_form: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#unit_of_presentation: Default::default(),
            r#manufacturer: Default::default(),
            r#ingredient: Default::default(),
            r#property: Default::default(),
        }
    }
}
