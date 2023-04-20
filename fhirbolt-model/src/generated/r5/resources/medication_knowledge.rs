// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "The price or representation of the cost (for example, Band A, Band B or $, $$) of the medication."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeCostCost {
    Money(Box<super::super::types::Money>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for MedicationKnowledgeCostCost {
    fn default() -> MedicationKnowledgeCostCost {
        MedicationKnowledgeCostCost::Invalid
    }
}
#[doc = "The specific characteristic (e.g. height, weight, gender, etc.)."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue {
    fn default() -> MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue
    {
        MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue::Invalid
    }
}
#[doc = "Either a textual source of the classification or a reference to an online source."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeMedicineClassificationSource {
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Invalid,
}
impl Default for MedicationKnowledgeMedicineClassificationSource {
    fn default() -> MedicationKnowledgeMedicineClassificationSource {
        MedicationKnowledgeMedicineClassificationSource::Invalid
    }
}
#[doc = "Value associated to the setting. E.g., 40° – 50°F for temperature."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue {
    fn default() -> MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue {
        MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue::Invalid
    }
}
#[doc = "Specifies how many (or how much) of the items there are in this Medication.  For example, 250 mg per tablet.  This is expressed as a ratio where the numerator is 250mg and the denominator is 1 tablet but can also be expressed a quantity when the denominator is assumed to be 1 tablet."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeDefinitionalIngredientStrength {
    Ratio(Box<super::super::types::Ratio>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for MedicationKnowledgeDefinitionalIngredientStrength {
    fn default() -> MedicationKnowledgeDefinitionalIngredientStrength {
        MedicationKnowledgeDefinitionalIngredientStrength::Invalid
    }
}
#[doc = "Description of the characteristic."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeDefinitionalDrugCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    Attachment(Box<super::super::types::Attachment>),
    Invalid,
}
impl Default for MedicationKnowledgeDefinitionalDrugCharacteristicValue {
    fn default() -> MedicationKnowledgeDefinitionalDrugCharacteristicValue {
        MedicationKnowledgeDefinitionalDrugCharacteristicValue::Invalid
    }
}
#[doc = "Associated or related medications. For example, if the medication is a branded product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if this is a generic medication (e.g. Rosuvastatin), this would link to a branded product (e.g. Crestor."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The category of the associated medication knowledge reference."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Associated documentation about the associated medication knowledge."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
}
#[doc = "Associated documentation about the medication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMonograph {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The category of documentation about the medication. (e.g. professional monograph, patient education monograph)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Associated documentation about the medication."]
    pub r#source: Option<Box<super::super::types::Reference>>,
}
#[doc = "The price of the medication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeCost {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The date range for which the cost information of the medication is effective."]
    pub r#effective_date: Vec<Box<super::super::types::Period>>,
    #[doc = "The category of the cost information.  For example, manufacturers' cost, patient cost, claim reimbursement cost, actual acquisition cost."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The source or owner that assigns the price to the medication."]
    pub r#source: Option<super::super::types::String>,
    #[doc = "The price or representation of the cost (for example, Band A, Band B or $, $$) of the medication."]
    pub r#cost: MedicationKnowledgeCostCost,
}
#[doc = "The program under which the medication is reviewed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMonitoringProgram {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of program under which the medication is monitored."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Name of the reviewing program."]
    pub r#name: Option<super::super::types::String>,
}
#[doc = "Dosage for the medication for the specific guidelines."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type or category of dosage for a given medication (for example, prophylaxis, maintenance, therapeutic, etc.)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Dosage for the medication for the specific guidelines."]
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
}
#[doc = "Characteristics of the patient that are relevant to the administration guidelines (for example, height, weight, gender, etc.)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The categorization of the specific characteristic that is relevant to the administration guideline (e.g. height, weight, gender)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The specific characteristic (e.g. height, weight, gender, etc.)."]
    pub r#value:
        Option<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue>,
}
#[doc = "The guidelines for the dosage of the medication for the indication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuideline {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The overall intention of the treatment, for example, prophylactic, supporative, curative, etc."]
    pub r#treatment_intent: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Dosage for the medication for the specific guidelines."]
    pub r#dosage: Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage>,
    #[doc = "The type of the treatment that the guideline applies to, for example, long term therapy, first line treatment, etc."]
    pub r#administration_treatment: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Characteristics of the patient that are relevant to the administration guidelines (for example, height, weight, gender, etc.)."]
    pub r#patient_characteristic:
        Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic>,
}
#[doc = "Guidelines or protocols that are applicable for the administration of the medication based on indication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeIndicationGuideline {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indication or reason for use of the medication that applies to the specific administration guideline."]
    pub r#indication: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The guidelines for the dosage of the medication for the indication."]
    pub r#dosing_guideline: Vec<MedicationKnowledgeIndicationGuidelineDosingGuideline>,
}
#[doc = "Categorization of the medication within a formulary or classification system."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeMedicineClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of category for the medication (for example, therapeutic classification, therapeutic sub-classification)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Either a textual source of the classification or a reference to an online source."]
    pub r#source: Option<MedicationKnowledgeMedicineClassificationSource>,
    #[doc = "Specific category assigned to the medication (e.g. anti-infective, anti-hypertensive, antibiotic, etc.)."]
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Information that only applies to packages (not products)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgePackaging {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The cost of the packaged medication."]
    pub r#cost: Vec<MedicationKnowledgeCost>,
    #[doc = "A reference to a PackagedProductDefinition that provides the details of the product that is in the packaging and is being priced."]
    pub r#packaged_product: Option<Box<super::super::types::Reference>>,
}
#[doc = "Describes a setting/value on the environment for the adequate storage of the medication and other substances.  Environment settings may involve temperature, humidity, or exposure to light."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeStorageGuidelineEnvironmentalSetting {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the category or type of setting (e.g., type of location, temperature, humidity)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Value associated to the setting. E.g., 40° – 50°F for temperature."]
    pub r#value: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue,
}
#[doc = "Information on how the medication should be stored, for example, refrigeration temperatures and length of stability at a given temperature."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeStorageGuideline {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to additional information about the storage guidelines."]
    pub r#reference: Option<super::super::types::Uri>,
    #[doc = "Additional notes about the storage."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Duration that the medication remains stable if the environmentalSetting is respected."]
    pub r#stability_duration: Option<Box<super::super::types::Duration>>,
    #[doc = "Describes a setting/value on the environment for the adequate storage of the medication and other substances.  Environment settings may involve temperature, humidity, or exposure to light."]
    pub r#environmental_setting: Vec<MedicationKnowledgeStorageGuidelineEnvironmentalSetting>,
}
#[doc = "Specifies if changes are allowed when dispensing a medication from a regulatory perspective."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatorySubstitution {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specifies the type of substitution allowed."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Specifies if regulation allows for changes in the medication when dispensing."]
    pub r#allowed: super::super::types::Boolean,
}
#[doc = "The maximum number of units of the medication that can be dispensed in a period."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The maximum number of units of the medication that can be dispensed."]
    pub r#quantity: Box<super::super::types::Quantity>,
    #[doc = "The period that applies to the maximum number of units."]
    pub r#period: Option<Box<super::super::types::Duration>>,
}
#[doc = "Regulatory information about a medication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeRegulatory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The authority that is specifying the regulations."]
    pub r#regulatory_authority: Box<super::super::types::Reference>,
    #[doc = "Specifies if changes are allowed when dispensing a medication from a regulatory perspective."]
    pub r#substitution: Vec<MedicationKnowledgeRegulatorySubstitution>,
    #[doc = "Specifies the schedule of a medication in jurisdiction."]
    pub r#schedule: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The maximum number of units of the medication that can be dispensed in a period."]
    pub r#max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
}
#[doc = "Identifies a particular constituent of interest in the product."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDefinitionalIngredient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A reference to the resource that provides information about the ingredient."]
    pub r#item: Box<super::super::types::CodeableReference>,
    #[doc = "Indication of whether this ingredient affects the therapeutic action of the drug."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifies how many (or how much) of the items there are in this Medication.  For example, 250 mg per tablet.  This is expressed as a ratio where the numerator is 250mg and the denominator is 1 tablet but can also be expressed a quantity when the denominator is assumed to be 1 tablet."]
    pub r#strength: Option<MedicationKnowledgeDefinitionalIngredientStrength>,
}
#[doc = "Specifies descriptive properties of the medicine, such as color, shape, imprints, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDefinitionalDrugCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code specifying which characteristic of the medicine is being described (for example, colour, shape, imprint)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Description of the characteristic."]
    pub r#value: Option<MedicationKnowledgeDefinitionalDrugCharacteristicValue>,
}
#[doc = "Along with the link to a Medicinal Product Definition resource, this information provides common definitional elements that are needed to understand the specific medication that is being described."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicationKnowledgeDefinitional {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Associated definitions for this medication."]
    pub r#definition: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes the form of the item.  Powder; tablets; capsule."]
    pub r#dose_form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The intended or approved route of administration."]
    pub r#intended_route: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies a particular constituent of interest in the product."]
    pub r#ingredient: Vec<MedicationKnowledgeDefinitionalIngredient>,
    #[doc = "Specifies descriptive properties of the medicine, such as color, shape, imprints, etc."]
    pub r#drug_characteristic: Vec<MedicationKnowledgeDefinitionalDrugCharacteristic>,
}
#[doc = "Information about a medication that is used to support knowledge."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business identifier for this medication."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A code that specifies this medication, or a textual description if no code is available. Usage note: This could be a standard medication code such as a code from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary code, optionally with translations to other code systems."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate if the medication referred to by this MedicationKnowledge is in active use within the drug database or inventory system. The status refers to the validity about the information of the medication and not to its medicinal properties."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The creator or owner of the knowledge or information about the medication."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Lists the jurisdictions that this medication knowledge was written for."]
    pub r#intended_jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "All of the names for a medication, for example, the name(s) given to a medication in different countries.  For example, acetaminophen and paracetamol or salbutamol and albuterol."]
    pub r#name: Vec<super::super::types::String>,
    #[doc = "Associated or related medications. For example, if the medication is a branded product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if this is a generic medication (e.g. Rosuvastatin), this would link to a branded product (e.g. Crestor."]
    pub r#related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,
    #[doc = "Links to associated medications that could be prescribed, dispensed or administered."]
    pub r#associated_medication: Vec<Box<super::super::types::Reference>>,
    #[doc = "Category of the medication or product (e.g. branded product, therapeutic moeity, generic product, innovator product, etc.)."]
    pub r#product_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Associated documentation about the medication."]
    pub r#monograph: Vec<MedicationKnowledgeMonograph>,
    #[doc = "The instructions for preparing the medication."]
    pub r#preparation_instruction: Option<super::super::types::Markdown>,
    #[doc = "The price of the medication."]
    pub r#cost: Vec<MedicationKnowledgeCost>,
    #[doc = "The program under which the medication is reviewed."]
    pub r#monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,
    #[doc = "Guidelines or protocols that are applicable for the administration of the medication based on indication."]
    pub r#indication_guideline: Vec<MedicationKnowledgeIndicationGuideline>,
    #[doc = "Categorization of the medication within a formulary or classification system."]
    pub r#medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,
    #[doc = "Information that only applies to packages (not products)."]
    pub r#packaging: Vec<MedicationKnowledgePackaging>,
    #[doc = "Potential clinical issue with or between medication(s) (for example, drug-drug interaction, drug-disease contraindication, drug-allergy interaction, etc.)."]
    pub r#clinical_use_issue: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information on how the medication should be stored, for example, refrigeration temperatures and length of stability at a given temperature."]
    pub r#storage_guideline: Vec<MedicationKnowledgeStorageGuideline>,
    #[doc = "Regulatory information about a medication."]
    pub r#regulatory: Vec<MedicationKnowledgeRegulatory>,
    #[doc = "Along with the link to a Medicinal Product Definition resource, this information provides common definitional elements that are needed to understand the specific medication that is being described."]
    pub r#definitional: Option<MedicationKnowledgeDefinitional>,
}
