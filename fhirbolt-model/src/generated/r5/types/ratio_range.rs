// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.\n\nNeed to be able to specify ranges of ratios."]
#[derive(Debug, Clone, PartialEq)]
pub struct RatioRange {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The value of the low limit numerator."]
    pub r#low_numerator: Option<Box<super::super::types::Quantity>>,
    #[doc = "The value of the high limit numerator."]
    pub r#high_numerator: Option<Box<super::super::types::Quantity>>,
    #[doc = "The value of the denominator."]
    pub r#denominator: Option<Box<super::super::types::Quantity>>,
}
impl Default for RatioRange {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#low_numerator: Default::default(),
            r#high_numerator: Default::default(),
            r#denominator: Default::default(),
        }
    }
}
