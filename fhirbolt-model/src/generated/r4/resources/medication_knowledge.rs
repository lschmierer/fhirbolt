// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The actual ingredient - either a substance (simple ingredient) or another medication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Indication for use that apply to the specific administration guidelines."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeAdministrationGuidelinesIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Specific characteristic that is relevant to the administration guideline (e.g. height, weight, gender)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    #[default]
    Invalid,
}
#[doc = "Description of the characteristic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeDrugCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    #[default]
    Invalid,
}
#[doc = "Associated or related knowledge about a medication."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The category of the associated medication knowledge reference."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Associated documentation about the associated medication knowledge."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
}
impl Default for MedicationKnowledgeRelatedMedicationKnowledge {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#reference: Default::default(),
        }
    }
}
#[doc = "Associated documentation about the medication."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMonograph {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The category of documentation about the medication. (e.g. professional monograph, patient education monograph)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Associated documentation about the medication."]
    pub r#source: Option<Box<super::super::types::Reference>>,
}
impl Default for MedicationKnowledgeMonograph {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "Identifies a particular constituent of interest in the product."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeIngredient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual ingredient - either a substance (simple ingredient) or another medication."]
    pub r#item: MedicationKnowledgeIngredientItem,
    #[doc = "Indication of whether this ingredient affects the therapeutic action of the drug."]
    pub r#is_active: Option<super::super::types::Boolean>,
    #[doc = "Specifies how many (or how much) of the items there are in this Medication.  For example, 250 mg per tablet.  This is expressed as a ratio where the numerator is 250mg and the denominator is 1 tablet."]
    pub r#strength: Option<Box<super::super::types::Ratio>>,
}
impl Default for MedicationKnowledgeIngredient {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
            r#is_active: Default::default(),
            r#strength: Default::default(),
        }
    }
}
#[doc = "The price of the medication."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeCost {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The category of the cost information.  For example, manufacturers' cost, patient cost, claim reimbursement cost, actual acquisition cost."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The source or owner that assigns the price to the medication."]
    pub r#source: Option<super::super::types::String>,
    #[doc = "The price of the medication."]
    pub r#cost: Box<super::super::types::Money>,
}
impl Default for MedicationKnowledgeCost {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#source: Default::default(),
            r#cost: {
                let mut default: Box<super::super::types::Money> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "The program under which the medication is reviewed."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMonitoringProgram {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of program under which the medication is monitored."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Name of the reviewing program."]
    pub r#name: Option<super::super::types::String>,
}
impl Default for MedicationKnowledgeMonitoringProgram {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#name: Default::default(),
        }
    }
}
#[doc = "Dosage for the medication for the specific guidelines."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeAdministrationGuidelinesDosage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of dosage (for example, prophylaxis, maintenance, therapeutic, etc.)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Dosage for the medication for the specific guidelines."]
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
}
impl Default for MedicationKnowledgeAdministrationGuidelinesDosage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#dosage: Default::default(),
        }
    }
}
#[doc = "Characteristics of the patient that are relevant to the administration guidelines (for example, height, weight, gender, etc.)."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specific characteristic that is relevant to the administration guideline (e.g. height, weight, gender)."]
    pub r#characteristic:
        MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic,
    #[doc = "The specific characteristic (e.g. height, weight, gender, etc.)."]
    pub r#value: Vec<super::super::types::String>,
}
impl Default for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#characteristic: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "Guidelines for the administration of the medication."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeAdministrationGuidelines {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Dosage for the medication for the specific guidelines."]
    pub r#dosage: Vec<MedicationKnowledgeAdministrationGuidelinesDosage>,
    #[doc = "Indication for use that apply to the specific administration guidelines."]
    pub r#indication: Option<MedicationKnowledgeAdministrationGuidelinesIndication>,
    #[doc = "Characteristics of the patient that are relevant to the administration guidelines (for example, height, weight, gender, etc.)."]
    pub r#patient_characteristics:
        Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>,
}
impl Default for MedicationKnowledgeAdministrationGuidelines {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#dosage: Default::default(),
            r#indication: Default::default(),
            r#patient_characteristics: Default::default(),
        }
    }
}
#[doc = "Categorization of the medication within a formulary or classification system."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMedicineClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of category for the medication (for example, therapeutic classification, therapeutic sub-classification)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Specific category assigned to the medication (e.g. anti-infective, anti-hypertensive, antibiotic, etc.)."]
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
}
impl Default for MedicationKnowledgeMedicineClassification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#classification: Default::default(),
        }
    }
}
#[doc = "Information that only applies to packages (not products)."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgePackaging {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code that defines the specific type of packaging that the medication can be found in (e.g. blister sleeve, tube, bottle)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of product units the package would contain if fully loaded."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
}
impl Default for MedicationKnowledgePackaging {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#quantity: Default::default(),
        }
    }
}
#[doc = "Specifies descriptive properties of the medicine, such as color, shape, imprints, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDrugCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code specifying which characteristic of the medicine is being described (for example, colour, shape, imprint)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Description of the characteristic."]
    pub r#value: Option<MedicationKnowledgeDrugCharacteristicValue>,
}
impl Default for MedicationKnowledgeDrugCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "Specifies if changes are allowed when dispensing a medication from a regulatory perspective."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatorySubstitution {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specifies the type of substitution allowed."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Specifies if regulation allows for changes in the medication when dispensing."]
    pub r#allowed: super::super::types::Boolean,
}
impl Default for MedicationKnowledgeRegulatorySubstitution {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#allowed: {
                let mut default: super::super::types::Boolean = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "Specifies the schedule of a medication in jurisdiction."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatorySchedule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specifies the specific drug schedule."]
    pub r#schedule: Box<super::super::types::CodeableConcept>,
}
impl Default for MedicationKnowledgeRegulatorySchedule {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#schedule: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "The maximum number of units of the medication that can be dispensed in a period."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The maximum number of units of the medication that can be dispensed."]
    pub r#quantity: Box<super::super::types::Quantity>,
    #[doc = "The period that applies to the maximum number of units."]
    pub r#period: Option<Box<super::super::types::Duration>>,
}
impl Default for MedicationKnowledgeRegulatoryMaxDispense {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#quantity: {
                let mut default: Box<super::super::types::Quantity> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: Default::default(),
        }
    }
}
#[doc = "Regulatory information about a medication."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The authority that is specifying the regulations."]
    pub r#regulatory_authority: Box<super::super::types::Reference>,
    #[doc = "Specifies if changes are allowed when dispensing a medication from a regulatory perspective."]
    pub r#substitution: Vec<MedicationKnowledgeRegulatorySubstitution>,
    #[doc = "Specifies the schedule of a medication in jurisdiction."]
    pub r#schedule: Vec<MedicationKnowledgeRegulatorySchedule>,
    #[doc = "The maximum number of units of the medication that can be dispensed in a period."]
    pub r#max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
}
impl Default for MedicationKnowledgeRegulatory {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#regulatory_authority: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#substitution: Default::default(),
            r#schedule: Default::default(),
            r#max_dispense: Default::default(),
        }
    }
}
#[doc = "The time course of drug absorption, distribution, metabolism and excretion of a medication from the body."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeKinetics {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The drug concentration measured at certain discrete points in time."]
    pub r#area_under_curve: Vec<Box<super::super::types::Quantity>>,
    #[doc = "The median lethal dose of a drug."]
    pub r#lethal_dose_50: Vec<Box<super::super::types::Quantity>>,
    #[doc = "The time required for any specified property (e.g., the concentration of a substance in the body) to decrease by half."]
    pub r#half_life_period: Option<Box<super::super::types::Duration>>,
}
impl Default for MedicationKnowledgeKinetics {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#area_under_curve: Default::default(),
            r#lethal_dose_50: Default::default(),
            r#half_life_period: Default::default(),
        }
    }
}
#[doc = "Information about a medication that is used to support knowledge."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationKnowledge {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code that specifies this medication, or a textual description if no code is available. Usage note: This could be a standard medication code such as a code from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary code, optionally with translations to other code systems."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate if the medication is in active use.  The status refers to the validity about the information of the medication and not to its medicinal properties."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Describes the details of the manufacturer of the medication product.  This is not intended to represent the distributor of a medication product."]
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the form of the item.  Powder; tablets; capsule."]
    pub r#dose_form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific amount of the drug in the packaged product.  For example, when specifying a product that has the same strength (For example, Insulin glargine 100 unit per mL solution for injection), this attribute provides additional clarification of the package amount (For example, 3 mL, 10mL, etc.)."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    #[doc = "Additional names for a medication, for example, the name(s) given to a medication in different countries.  For example, acetaminophen and paracetamol or salbutamol and albuterol."]
    pub r#synonym: Vec<super::super::types::String>,
    #[doc = "Associated or related knowledge about a medication."]
    pub r#related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,
    #[doc = "Associated or related medications.  For example, if the medication is a branded product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if this is a generic medication (e.g. Rosuvastatin), this would link to a branded product (e.g. Crestor)."]
    pub r#associated_medication: Vec<Box<super::super::types::Reference>>,
    #[doc = "Category of the medication or product (e.g. branded product, therapeutic moeity, generic product, innovator product, etc.)."]
    pub r#product_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Associated documentation about the medication."]
    pub r#monograph: Vec<MedicationKnowledgeMonograph>,
    #[doc = "Identifies a particular constituent of interest in the product."]
    pub r#ingredient: Vec<MedicationKnowledgeIngredient>,
    #[doc = "The instructions for preparing the medication."]
    pub r#preparation_instruction: Option<super::super::types::Markdown>,
    #[doc = "The intended or approved route of administration."]
    pub r#intended_route: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The price of the medication."]
    pub r#cost: Vec<MedicationKnowledgeCost>,
    #[doc = "The program under which the medication is reviewed."]
    pub r#monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,
    #[doc = "Guidelines for the administration of the medication."]
    pub r#administration_guidelines: Vec<MedicationKnowledgeAdministrationGuidelines>,
    #[doc = "Categorization of the medication within a formulary or classification system."]
    pub r#medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,
    #[doc = "Information that only applies to packages (not products)."]
    pub r#packaging: Option<MedicationKnowledgePackaging>,
    #[doc = "Specifies descriptive properties of the medicine, such as color, shape, imprints, etc."]
    pub r#drug_characteristic: Vec<MedicationKnowledgeDrugCharacteristic>,
    #[doc = "Potential clinical issue with or between medication(s) (for example, drug-drug interaction, drug-disease contraindication, drug-allergy interaction, etc.)."]
    pub r#contraindication: Vec<Box<super::super::types::Reference>>,
    #[doc = "Regulatory information about a medication."]
    pub r#regulatory: Vec<MedicationKnowledgeRegulatory>,
    #[doc = "The time course of drug absorption, distribution, metabolism and excretion of a medication from the body."]
    pub r#kinetics: Vec<MedicationKnowledgeKinetics>,
}
impl Default for MedicationKnowledge {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#status: Default::default(),
            r#manufacturer: Default::default(),
            r#dose_form: Default::default(),
            r#amount: Default::default(),
            r#synonym: Default::default(),
            r#related_medication_knowledge: Default::default(),
            r#associated_medication: Default::default(),
            r#product_type: Default::default(),
            r#monograph: Default::default(),
            r#ingredient: Default::default(),
            r#preparation_instruction: Default::default(),
            r#intended_route: Default::default(),
            r#cost: Default::default(),
            r#monitoring_program: Default::default(),
            r#administration_guidelines: Default::default(),
            r#medicine_classification: Default::default(),
            r#packaging: Default::default(),
            r#drug_characteristic: Default::default(),
            r#contraindication: Default::default(),
            r#regulatory: Default::default(),
            r#kinetics: Default::default(),
        }
    }
}
