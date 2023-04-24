// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "MonetaryComponent Type: Availability data for an {item}."]
#[derive(Debug, Clone, PartialEq)]
pub struct MonetaryComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "base | surcharge | deduction | discount | tax | informational."]
    pub r#type: super::super::types::Code,
    #[doc = "Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Factor used for calculating this component."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "Explicit value amount to be used."]
    pub r#amount: Option<Box<super::super::types::Money>>,
}
impl Default for MonetaryComponent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#type: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#code: Default::default(),
            r#factor: Default::default(),
            r#amount: Default::default(),
        }
    }
}
