// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "The age of the specific population."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum PopulationAge {
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Base StructureDefinition for Population Type: A populatioof people with some set of grouping criteria."]
#[derive(Debug, Clone, PartialEq)]
pub struct Population {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The age of the specific population."]
    pub r#age: Option<PopulationAge>,
    #[doc = "The gender of the specific population."]
    pub r#gender: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Race of the specific population."]
    pub r#race: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The existing physiological conditions of the specific population to which this applies."]
    pub r#physiological_condition: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Population {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#age: Default::default(),
            r#gender: Default::default(),
            r#race: Default::default(),
            r#physiological_condition: Default::default(),
        }
    }
}
