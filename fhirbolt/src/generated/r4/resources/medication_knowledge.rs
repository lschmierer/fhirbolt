// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeDrugCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeKinetics {
    pub r#half_life_period: Option<Box<super::super::types::Duration>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#area_under_curve: Vec<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#lethal_dose_50: Vec<Box<super::super::types::Quantity>>,
}
impl serde::Serialize for MedicationKnowledgeKinetics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#half_life_period.as_ref() {
            state.serialize_entry("halfLifePeriod", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#area_under_curve.is_empty() {
            state.serialize_entry("areaUnderCurve", &self.r#area_under_curve)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#lethal_dose_50.is_empty() {
            state.serialize_entry("lethalDose50", &self.r#lethal_dose_50)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgePackaging {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::Serialize for MedicationKnowledgePackaging {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMonograph {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for MedicationKnowledgeMonograph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#source.as_ref() {
            state.serialize_entry("source", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeCost {
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#source: Option<super::super::types::String>,
    pub r#cost: Box<super::super::types::Money>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for MedicationKnowledgeCost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#source.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_source", &primitive_element)?;
            }
        }
        state.serialize_entry("cost", &self.r#cost)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesDosage {
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for MedicationKnowledgeAdministrationGuidelinesDosage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#dosage.is_empty() {
            state.serialize_entry("dosage", &self.r#dosage)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#characteristic:
        MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Vec<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        match self . r#characteristic { MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: CodeableConcept (ref value) => { state . serialize_entry ("characteristicCodeableConcept" , value) ? ; } , MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: Quantity (ref value) => { state . serialize_entry ("characteristicQuantity" , value) ? ; } , }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#value.is_empty() {
            let values: Vec<_> = self.r#value.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("value", &values)?;
            }
            let requires_elements = self
                .r#value
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#value
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_value", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelines {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#dosage: Vec<MedicationKnowledgeAdministrationGuidelinesDosage>,
    pub r#patient_characteristics:
        Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#indication: Option<MedicationKnowledgeAdministrationGuidelinesIndication>,
}
impl serde::Serialize for MedicationKnowledgeAdministrationGuidelines {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#dosage.is_empty() {
            state.serialize_entry("dosage", &self.r#dosage)?;
        }
        if !self.r#patient_characteristics.is_empty() {
            state.serialize_entry("patientCharacteristics", &self.r#patient_characteristics)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#indication.as_ref() {
            match some {
                MedicationKnowledgeAdministrationGuidelinesIndication::CodeableConcept(
                    ref value,
                ) => {
                    state.serialize_entry("indicationCodeableConcept", value)?;
                }
                MedicationKnowledgeAdministrationGuidelinesIndication::Reference(ref value) => {
                    state.serialize_entry("indicationReference", value)?;
                }
            }
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for MedicationKnowledgeRelatedMedicationKnowledge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#reference.is_empty() {
            state.serialize_entry("reference", &self.r#reference)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMonitoringProgram {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
}
impl serde::Serialize for MedicationKnowledgeMonitoringProgram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySubstitution {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#allowed: super::super::types::Boolean,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for MedicationKnowledgeRegulatorySubstitution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        {
            if let Some(some) = self.r#allowed.value.as_ref() {
                state.serialize_entry("allowed", some)?;
            }
            if self.r#allowed.id.is_some() || !self.r#allowed.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#allowed.id,
                    extension: &self.r#allowed.extension,
                };
                state.serialize_entry("_allowed", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySchedule {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Box<super::super::types::CodeableConcept>,
}
impl serde::Serialize for MedicationKnowledgeRegulatorySchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("schedule", &self.r#schedule)?;
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    pub r#period: Option<Box<super::super::types::Duration>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for MedicationKnowledgeRegulatoryMaxDispense {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.serialize_entry("quantity", &self.r#quantity)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeRegulatory {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#substitution: Vec<MedicationKnowledgeRegulatorySubstitution>,
    pub r#regulatory_authority: Box<super::super::types::Reference>,
    pub r#schedule: Vec<MedicationKnowledgeRegulatorySchedule>,
    pub r#max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for MedicationKnowledgeRegulatory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#substitution.is_empty() {
            state.serialize_entry("substitution", &self.r#substitution)?;
        }
        state.serialize_entry("regulatoryAuthority", &self.r#regulatory_authority)?;
        if !self.r#schedule.is_empty() {
            state.serialize_entry("schedule", &self.r#schedule)?;
        }
        if let Some(some) = self.r#max_dispense.as_ref() {
            state.serialize_entry("maxDispense", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeIngredient {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#strength: Option<Box<super::super::types::Ratio>>,
    pub r#item: MedicationKnowledgeIngredientItem,
    pub r#id: Option<std::string::String>,
    pub r#is_active: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for MedicationKnowledgeIngredient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#strength.as_ref() {
            state.serialize_entry("strength", some)?;
        }
        match self.r#item {
            MedicationKnowledgeIngredientItem::CodeableConcept(ref value) => {
                state.serialize_entry("itemCodeableConcept", value)?;
            }
            MedicationKnowledgeIngredientItem::Reference(ref value) => {
                state.serialize_entry("itemReference", value)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#is_active.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isActive", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isActive", &primitive_element)?;
            }
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeDrugCharacteristic {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#value: Option<MedicationKnowledgeDrugCharacteristicValue>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for MedicationKnowledgeDrugCharacteristic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                MedicationKnowledgeDrugCharacteristicValue::CodeableConcept(ref value) => {
                    state.serialize_entry("valueCodeableConcept", value)?;
                }
                MedicationKnowledgeDrugCharacteristicValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                MedicationKnowledgeDrugCharacteristicValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                MedicationKnowledgeDrugCharacteristicValue::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                    }
                }
            }
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledgeMedicineClassification {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
}
impl serde::Serialize for MedicationKnowledgeMedicineClassification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#classification.is_empty() {
            state.serialize_entry("classification", &self.r#classification)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicationKnowledge {
    pub r#kinetics: Vec<MedicationKnowledgeKinetics>,
    pub r#packaging: Option<MedicationKnowledgePackaging>,
    pub r#language: Option<super::super::types::Code>,
    pub r#contraindication: Vec<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#synonym: Vec<super::super::types::String>,
    pub r#monograph: Vec<MedicationKnowledgeMonograph>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#associated_medication: Vec<Box<super::super::types::Reference>>,
    pub r#cost: Vec<MedicationKnowledgeCost>,
    pub r#administration_guidelines: Vec<MedicationKnowledgeAdministrationGuidelines>,
    pub r#dose_form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    pub r#related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,
    pub r#monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#preparation_instruction: Option<super::super::types::Markdown>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#product_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#regulatory: Vec<MedicationKnowledgeRegulatory>,
    pub r#intended_route: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#ingredient: Vec<MedicationKnowledgeIngredient>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#drug_characteristic: Vec<MedicationKnowledgeDrugCharacteristic>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,
}
impl serde::Serialize for MedicationKnowledge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationKnowledge")?;
        if !self.r#kinetics.is_empty() {
            state.serialize_entry("kinetics", &self.r#kinetics)?;
        }
        if let Some(some) = self.r#packaging.as_ref() {
            state.serialize_entry("packaging", some)?;
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if !self.r#contraindication.is_empty() {
            state.serialize_entry("contraindication", &self.r#contraindication)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#synonym.is_empty() {
            let values: Vec<_> = self.r#synonym.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("synonym", &values)?;
            }
            let requires_elements = self
                .r#synonym
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#synonym
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_synonym", &primitive_elements)?;
            }
        }
        if !self.r#monograph.is_empty() {
            state.serialize_entry("monograph", &self.r#monograph)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if !self.r#associated_medication.is_empty() {
            state.serialize_entry("associatedMedication", &self.r#associated_medication)?;
        }
        if !self.r#cost.is_empty() {
            state.serialize_entry("cost", &self.r#cost)?;
        }
        if !self.r#administration_guidelines.is_empty() {
            state.serialize_entry(
                "administrationGuidelines",
                &self.r#administration_guidelines,
            )?;
        }
        if let Some(some) = self.r#dose_form.as_ref() {
            state.serialize_entry("doseForm", some)?;
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        if !self.r#related_medication_knowledge.is_empty() {
            state.serialize_entry(
                "relatedMedicationKnowledge",
                &self.r#related_medication_knowledge,
            )?;
        }
        if !self.r#monitoring_program.is_empty() {
            state.serialize_entry("monitoringProgram", &self.r#monitoring_program)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#preparation_instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preparationInstruction", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preparationInstruction", &primitive_element)?;
            }
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#product_type.is_empty() {
            state.serialize_entry("productType", &self.r#product_type)?;
        }
        if !self.r#regulatory.is_empty() {
            state.serialize_entry("regulatory", &self.r#regulatory)?;
        }
        if !self.r#intended_route.is_empty() {
            state.serialize_entry("intendedRoute", &self.r#intended_route)?;
        }
        if !self.r#ingredient.is_empty() {
            state.serialize_entry("ingredient", &self.r#ingredient)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if !self.r#drug_characteristic.is_empty() {
            state.serialize_entry("drugCharacteristic", &self.r#drug_characteristic)?;
        }
        if let Some(some) = self.r#manufacturer.as_ref() {
            state.serialize_entry("manufacturer", some)?;
        }
        if !self.r#medicine_classification.is_empty() {
            state.serialize_entry("medicineClassification", &self.r#medicine_classification)?;
        }
        state.end()
    }
}
