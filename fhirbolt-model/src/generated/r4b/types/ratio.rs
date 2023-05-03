// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "Base StructureDefinition for Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.\n\nNeed to able to capture ratios for some measurements (titers) and some rates (costs)."]
#[derive(Debug, Clone, PartialEq)]
pub struct Ratio {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The value of the numerator."]
    pub r#numerator: Option<Box<super::super::types::Quantity>>,
    #[doc = "The value of the denominator."]
    pub r#denominator: Option<Box<super::super::types::Quantity>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Ratio {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#numerator: Default::default(),
            r#denominator: Default::default(),
        }
    }
}
