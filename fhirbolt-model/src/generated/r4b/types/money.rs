// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Base StructureDefinition for Money Type: An amount of economic utility in some recognized currency."]
#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Numerical value (with implicit precision)."]
    pub r#value: Option<super::super::types::Decimal>,
    #[doc = "ISO 4217 Currency Code."]
    pub r#currency: Option<super::super::types::Code>,
}
#[allow(clippy::derivable_impls)]
impl Default for Money {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#value: Default::default(),
            r#currency: Default::default(),
        }
    }
}
