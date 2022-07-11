// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SubstanceAmountAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct SubstanceAmountReferenceRange {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#low_limit: Option<Box<super::super::types::Quantity>>,
    pub r#high_limit: Option<Box<super::super::types::Quantity>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceAmount {
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount_text: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#reference_range: Option<Box<super::super::types::Element>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<SubstanceAmountAmount>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
