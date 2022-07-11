// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductContraindicationOtherTherapyMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicinalProductContraindicationOtherTherapy {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#medication: MedicinalProductContraindicationOtherTherapyMedication,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#therapy_relationship_type: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductContraindication {
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#comorbidity: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#disease_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#other_therapy: Vec<MedicinalProductContraindicationOtherTherapy>,
    pub r#population: Vec<Box<super::super::types::Population>>,
    pub r#therapeutic_indication: Vec<Box<super::super::types::Reference>>,
    pub r#disease: Option<Box<super::super::types::CodeableConcept>>,
}
