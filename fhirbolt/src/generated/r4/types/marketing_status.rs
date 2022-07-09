// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MarketingStatus {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: Box<super::super::types::CodeableConcept>,
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#country: Box<super::super::types::CodeableConcept>,
    pub r#restore_date: Option<super::super::types::DateTime>,
    pub r#date_range: Box<super::super::types::Period>,
    pub r#id: Option<std::string::String>,
}
