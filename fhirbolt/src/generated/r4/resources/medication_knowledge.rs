// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeDrugCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeDrugCharacteristic {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<MedicationKnowledgeDrugCharacteristicValue>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySubstitution {
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#allowed: super::super::types::Boolean,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySchedule {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#period: Option<Box<super::super::types::Duration>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatory {
    pub r#substitution: Vec<MedicationKnowledgeRegulatorySubstitution>,
    pub r#schedule: Vec<MedicationKnowledgeRegulatorySchedule>,
    pub r#max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#regulatory_authority: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesDosage {
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    pub r#value: Vec<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#characteristic:
        MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelines {
    pub r#indication: Option<MedicationKnowledgeAdministrationGuidelinesIndication>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#dosage: Vec<MedicationKnowledgeAdministrationGuidelinesDosage>,
    pub r#patient_characteristics:
        Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeKinetics {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#area_under_curve: Vec<Box<super::super::types::Quantity>>,
    pub r#half_life_period: Option<Box<super::super::types::Duration>>,
    pub r#lethal_dose_50: Vec<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMonograph {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeCost {
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#source: Option<super::super::types::String>,
    pub r#cost: Box<super::super::types::Money>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgePackaging {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMonitoringProgram {
    pub r#name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeIngredient {
    pub r#id: Option<std::string::String>,
    pub r#is_active: Option<super::super::types::Boolean>,
    pub r#item: MedicationKnowledgeIngredientItem,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#strength: Option<Box<super::super::types::Ratio>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMedicineClassification {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledge {
    pub r#drug_characteristic: Vec<MedicationKnowledgeDrugCharacteristic>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#associated_medication: Vec<Box<super::super::types::Reference>>,
    pub r#regulatory: Vec<MedicationKnowledgeRegulatory>,
    pub r#synonym: Vec<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#administration_guidelines: Vec<MedicationKnowledgeAdministrationGuidelines>,
    pub r#kinetics: Vec<MedicationKnowledgeKinetics>,
    pub r#monograph: Vec<MedicationKnowledgeMonograph>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#contraindication: Vec<Box<super::super::types::Reference>>,
    pub r#cost: Vec<MedicationKnowledgeCost>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#intended_route: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#packaging: Option<MedicationKnowledgePackaging>,
    pub r#id: Option<std::string::String>,
    pub r#dose_form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#ingredient: Vec<MedicationKnowledgeIngredient>,
    pub r#status: Option<super::super::types::Code>,
    pub r#preparation_instruction: Option<super::super::types::Markdown>,
    pub r#medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,
}
