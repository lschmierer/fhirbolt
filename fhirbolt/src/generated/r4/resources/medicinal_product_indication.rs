// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductIndicationOtherTherapyMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicinalProductIndicationOtherTherapy {
    pub r#therapy_relationship_type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#medication: MedicinalProductIndicationOtherTherapyMedication,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductIndication {
    pub r#other_therapy: Vec<MedicinalProductIndicationOtherTherapy>,
    pub r#intended_effect: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#population: Vec<Box<super::super::types::Population>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#comorbidity: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#disease_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#undesirable_effect: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
