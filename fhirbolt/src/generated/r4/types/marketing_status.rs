// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MarketingStatus {
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#restore_date: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#date_range: Box<super::super::types::Period>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#country: Box<super::super::types::CodeableConcept>,
    pub r#status: Box<super::super::types::CodeableConcept>,
}
