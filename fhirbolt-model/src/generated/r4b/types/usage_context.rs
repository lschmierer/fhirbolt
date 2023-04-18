// Generated on 2023-04-18 by fhirbolt-codegen v0.2.0
#[doc = "A value that defines the context specified in this context of use. The interpretation of the value is defined by the code."]
#[derive(Debug, Clone, PartialEq)]
pub enum UsageContextValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for UsageContextValue {
    fn default() -> UsageContextValue {
        UsageContextValue::Invalid
    }
}
#[doc = "Base StructureDefinition for UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).\n\nConsumers of the resource must be able to determine the intended applicability for the resource. Ideally, this information would be used programmatically to determine when and how it should be incorporated or exposed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UsageContext {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code that identifies the type of context being specified by this usage context."]
    pub r#code: Box<super::super::types::Coding>,
    #[doc = "A value that defines the context specified in this context of use. The interpretation of the value is defined by the code."]
    pub r#value: UsageContextValue,
}
