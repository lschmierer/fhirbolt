// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct SubstanceAmountReferenceRange {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#high_limit: Option<Box<super::super::types::Quantity>>,
    pub r#low_limit: Option<Box<super::super::types::Quantity>>,
}
#[derive(Debug, Clone)]
pub enum SubstanceAmountAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct SubstanceAmount {
    pub r#id: Option<std::string::String>,
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount_text: Option<super::super::types::String>,
    pub r#reference_range: Option<Box<super::super::types::Element>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<SubstanceAmountAmount>,
}
