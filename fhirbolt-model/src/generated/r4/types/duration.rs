// Generated on 2023-04-14 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Duration Type: A length of time."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Duration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The value of the measured amount. The value includes an implicit precision in the presentation of the value."]
    pub r#value: Option<super::super::types::Decimal>,
    #[doc = "How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is \"<\" , then the real value is < stated value."]
    pub r#comparator: Option<super::super::types::Code>,
    #[doc = "A human-readable form of the unit."]
    pub r#unit: Option<super::super::types::String>,
    #[doc = "The identification of the system that provides the coded form of the unit."]
    pub r#system: Option<super::super::types::Uri>,
    #[doc = "A computer processable form of the unit in some unit representation system."]
    pub r#code: Option<super::super::types::Code>,
}
