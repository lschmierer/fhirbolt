// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMonograph {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMonitoringProgram {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#name: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgePackaging {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMedicineClassification {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#period: Option<Box<super::super::types::Duration>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySchedule {
    pub r#id: Option<std::string::String>,
    pub r#schedule: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySubstitution {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#allowed: super::super::types::Boolean,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatory {
    pub r#id: Option<std::string::String>,
    pub r#max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
    pub r#schedule: Vec<MedicationKnowledgeRegulatorySchedule>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#regulatory_authority: Box<super::super::types::Reference>,
    pub r#substitution: Vec<MedicationKnowledgeRegulatorySubstitution>,
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeIngredient {
    pub r#id: Option<std::string::String>,
    pub r#is_active: Option<super::super::types::Boolean>,
    pub r#item: MedicationKnowledgeIngredientItem,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#strength: Option<Box<super::super::types::Ratio>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeCost {
    pub r#cost: Box<super::super::types::Money>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#source: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    pub r#id: Option<std::string::String>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesDosage {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    pub r#value: Vec<super::super::types::String>,
    pub r#characteristic:
        MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelines {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#dosage: Vec<MedicationKnowledgeAdministrationGuidelinesDosage>,
    pub r#id: Option<std::string::String>,
    pub r#patient_characteristics:
        Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>,
    pub r#indication: Option<MedicationKnowledgeAdministrationGuidelinesIndication>,
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeDrugCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeDrugCharacteristic {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<MedicationKnowledgeDrugCharacteristicValue>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeKinetics {
    pub r#lethal_dose_50: Vec<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#area_under_curve: Vec<Box<super::super::types::Quantity>>,
    pub r#half_life_period: Option<Box<super::super::types::Duration>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledge {
    pub r#language: Option<super::super::types::Code>,
    pub r#monograph: Vec<MedicationKnowledgeMonograph>,
    pub r#synonym: Vec<super::super::types::String>,
    pub r#dose_form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#packaging: Option<MedicationKnowledgePackaging>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#regulatory: Vec<MedicationKnowledgeRegulatory>,
    pub r#product_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    pub r#ingredient: Vec<MedicationKnowledgeIngredient>,
    pub r#cost: Vec<MedicationKnowledgeCost>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#contraindication: Vec<Box<super::super::types::Reference>>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,
    pub r#administration_guidelines: Vec<MedicationKnowledgeAdministrationGuidelines>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#associated_medication: Vec<Box<super::super::types::Reference>>,
    pub r#drug_characteristic: Vec<MedicationKnowledgeDrugCharacteristic>,
    pub r#preparation_instruction: Option<super::super::types::Markdown>,
    pub r#status: Option<super::super::types::Code>,
    pub r#intended_route: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#kinetics: Vec<MedicationKnowledgeKinetics>,
}
