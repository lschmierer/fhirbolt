// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "The actual ingredient - either a substance (simple ingredient) or another medication."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationKnowledgeIngredientItem {
    fn default() -> MedicationKnowledgeIngredientItem {
        MedicationKnowledgeIngredientItem::Invalid
    }
}
#[doc = "Indication for use that apply to the specific administration guidelines."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeAdministrationGuidelinesIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationKnowledgeAdministrationGuidelinesIndication {
    fn default() -> MedicationKnowledgeAdministrationGuidelinesIndication {
        MedicationKnowledgeAdministrationGuidelinesIndication::Invalid
    }
}
#[doc = "Specific characteristic that is relevant to the administration guideline (e.g. height, weight, gender)."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    fn default() -> MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic
    {
        MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic::Invalid
    }
}
#[doc = "Description of the characteristic."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicationKnowledgeDrugCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    Invalid,
}
impl Default for MedicationKnowledgeDrugCharacteristicValue {
    fn default() -> MedicationKnowledgeDrugCharacteristicValue {
        MedicationKnowledgeDrugCharacteristicValue::Invalid
    }
}
#[doc = "Associated or related knowledge about a medication."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeRelatedMedicationKnowledge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if !self.r#reference.is_empty() {
                state.serialize_entry("reference", &self.r#reference)?;
            }
            state.end()
        })
    }
}
#[doc = "Associated documentation about the medication."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeMonograph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
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
            state.end()
        })
    }
}
#[doc = "Identifies a particular constituent of interest in the product."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeIngredient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            match self.r#item {
                MedicationKnowledgeIngredientItem::CodeableConcept(ref value) => {
                    state.serialize_entry("itemCodeableConcept", value)?;
                }
                MedicationKnowledgeIngredientItem::Reference(ref value) => {
                    state.serialize_entry("itemReference", value)?;
                }
                MedicationKnowledgeIngredientItem::Invalid => {
                    return Err(serde::ser::Error::custom("item is a required field"))
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#is_active.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isActive", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isActive", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_active.as_ref() {
                    state.serialize_entry("isActive", some)?;
                }
            }
            if let Some(some) = self.r#strength.as_ref() {
                state.serialize_entry("strength", some)?;
            }
            state.end()
        })
    }
}
#[doc = "The price of the medication."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeCost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if _ctx.output_json {
                if let Some(some) = self.r#source.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("source", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_source", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source.as_ref() {
                    state.serialize_entry("source", some)?;
                }
            }
            state.serialize_entry("cost", &self.r#cost)?;
            state.end()
        })
    }
}
#[doc = "The program under which the medication is reviewed."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeMonitoringProgram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Dosage for the medication for the specific guidelines."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeAdministrationGuidelinesDosage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if !self.r#dosage.is_empty() {
                state.serialize_entry("dosage", &self.r#dosage)?;
            }
            state.end()
        })
    }
}
#[doc = "Characteristics of the patient that are relevant to the administration guidelines (for example, height, weight, gender, etc.)."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared :: serde_context :: ser :: SERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . borrow () ; let mut state = serializer . serialize_map (None) ? ; if let Some (some) = self . r#id . as_ref () { state . serialize_entry ("id" , some) ? ; } if ! self . r#extension . is_empty () { state . serialize_entry ("extension" , & self . r#extension) ? ; } if ! self . r#modifier_extension . is_empty () { state . serialize_entry ("modifierExtension" , & self . r#modifier_extension) ? ; } match self . r#characteristic { MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: CodeableConcept (ref value) => { state . serialize_entry ("characteristicCodeableConcept" , value) ? ; } , MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: Quantity (ref value) => { state . serialize_entry ("characteristicQuantity" , value) ? ; } , MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: Invalid => { return Err (serde :: ser :: Error :: custom ("characteristic is a required field")) } } if _ctx . output_json { if ! self . r#value . is_empty () { let values = self . r#value . iter () . map (| v | & v . value) . map (| v | v . as_ref () . map (| some | Ok (some)) . transpose ()) . collect :: < Result < Vec < _ > , _ >> () ? ; if values . iter () . any (| v | v . is_some ()) { state . serialize_entry ("value" , & values) ? ; } let requires_elements = self . r#value . iter () . any (| e | e . id . is_some () || ! e . extension . is_empty ()) ; if requires_elements { let primitive_elements : Vec < _ > = self . r#value . iter () . map (| e | if e . id . is_some () || ! e . extension . is_empty () { Some (super :: super :: serde_helpers :: PrimitiveElement { id : e . id . as_ref () , extension : & e . extension , }) } else { None }) . collect () ; state . serialize_entry ("_value" , & primitive_elements) ? ; } } } else { if ! self . r#value . is_empty () { state . serialize_entry ("value" , & self . r#value) ? ; } } state . end () })
    }
}
#[doc = "Guidelines for the administration of the medication."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeAdministrationGuidelines {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if !self.r#dosage.is_empty() {
                state.serialize_entry("dosage", &self.r#dosage)?;
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
                    MedicationKnowledgeAdministrationGuidelinesIndication::Invalid => {
                        return Err(serde::ser::Error::custom("indication is invalid"))
                    }
                }
            }
            if !self.r#patient_characteristics.is_empty() {
                state.serialize_entry("patientCharacteristics", &self.r#patient_characteristics)?;
            }
            state.end()
        })
    }
}
#[doc = "Categorization of the medication within a formulary or classification system."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeMedicineClassification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if !self.r#classification.is_empty() {
                state.serialize_entry("classification", &self.r#classification)?;
            }
            state.end()
        })
    }
}
#[doc = "Information that only applies to packages (not products)."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgePackaging {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Specifies descriptive properties of the medicine, such as color, shape, imprints, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeDrugCharacteristic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
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
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueString", value)?;
                        }
                    }
                    MedicationKnowledgeDrugCharacteristicValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    MedicationKnowledgeDrugCharacteristicValue::Base64Binary(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueBase64Binary", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueBase64Binary", value)?;
                        }
                    }
                    MedicationKnowledgeDrugCharacteristicValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Specifies if changes are allowed when dispensing a medication from a regulatory perspective."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeRegulatorySubstitution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if _ctx.output_json {
                if let Some(some) = self.r#allowed.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("allowed", &some)?;
                }
                if self.r#allowed.id.is_some() || !self.r#allowed.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#allowed.id.as_ref(),
                        extension: &self.r#allowed.extension,
                    };
                    state.serialize_entry("_allowed", &primitive_element)?;
                }
            } else {
                state.serialize_entry("allowed", &self.r#allowed)?;
            }
            state.end()
        })
    }
}
#[doc = "Specifies the schedule of a medication in jurisdiction."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeRegulatorySchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("schedule", &self.r#schedule)?;
            state.end()
        })
    }
}
#[doc = "The maximum number of units of the medication that can be dispensed in a period."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeRegulatoryMaxDispense {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("quantity", &self.r#quantity)?;
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Regulatory information about a medication."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeRegulatory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("regulatoryAuthority", &self.r#regulatory_authority)?;
            if !self.r#substitution.is_empty() {
                state.serialize_entry("substitution", &self.r#substitution)?;
            }
            if !self.r#schedule.is_empty() {
                state.serialize_entry("schedule", &self.r#schedule)?;
            }
            if let Some(some) = self.r#max_dispense.as_ref() {
                state.serialize_entry("maxDispense", some)?;
            }
            state.end()
        })
    }
}
#[doc = "The time course of drug absorption, distribution, metabolism and excretion of a medication from the body."]
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for MedicationKnowledgeKinetics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if !self.r#area_under_curve.is_empty() {
                state.serialize_entry("areaUnderCurve", &self.r#area_under_curve)?;
            }
            if !self.r#lethal_dose_50.is_empty() {
                state.serialize_entry("lethalDose50", &self.r#lethal_dose_50)?;
            }
            if let Some(some) = self.r#half_life_period.as_ref() {
                state.serialize_entry("halfLifePeriod", some)?;
            }
            state.end()
        })
    }
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
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
impl crate::AnyResource for MedicationKnowledge {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for MedicationKnowledge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicationKnowledge")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
                }
            }
            if let Some(some) = self.r#text.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if !self.r#contained.is_empty() {
                state.serialize_entry("contained", &self.r#contained)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if let Some(some) = self.r#manufacturer.as_ref() {
                state.serialize_entry("manufacturer", some)?;
            }
            if let Some(some) = self.r#dose_form.as_ref() {
                state.serialize_entry("doseForm", some)?;
            }
            if let Some(some) = self.r#amount.as_ref() {
                state.serialize_entry("amount", some)?;
            }
            if _ctx.output_json {
                if !self.r#synonym.is_empty() {
                    let values = self
                        .r#synonym
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("synonym", &values)?;
                    }
                    let requires_elements = self
                        .r#synonym
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#synonym
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#synonym.is_empty() {
                    state.serialize_entry("synonym", &self.r#synonym)?;
                }
            }
            if !self.r#related_medication_knowledge.is_empty() {
                state.serialize_entry(
                    "relatedMedicationKnowledge",
                    &self.r#related_medication_knowledge,
                )?;
            }
            if !self.r#associated_medication.is_empty() {
                state.serialize_entry("associatedMedication", &self.r#associated_medication)?;
            }
            if !self.r#product_type.is_empty() {
                state.serialize_entry("productType", &self.r#product_type)?;
            }
            if !self.r#monograph.is_empty() {
                state.serialize_entry("monograph", &self.r#monograph)?;
            }
            if !self.r#ingredient.is_empty() {
                state.serialize_entry("ingredient", &self.r#ingredient)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preparation_instruction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preparationInstruction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preparationInstruction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#preparation_instruction.as_ref() {
                    state.serialize_entry("preparationInstruction", some)?;
                }
            }
            if !self.r#intended_route.is_empty() {
                state.serialize_entry("intendedRoute", &self.r#intended_route)?;
            }
            if !self.r#cost.is_empty() {
                state.serialize_entry("cost", &self.r#cost)?;
            }
            if !self.r#monitoring_program.is_empty() {
                state.serialize_entry("monitoringProgram", &self.r#monitoring_program)?;
            }
            if !self.r#administration_guidelines.is_empty() {
                state.serialize_entry(
                    "administrationGuidelines",
                    &self.r#administration_guidelines,
                )?;
            }
            if !self.r#medicine_classification.is_empty() {
                state.serialize_entry("medicineClassification", &self.r#medicine_classification)?;
            }
            if let Some(some) = self.r#packaging.as_ref() {
                state.serialize_entry("packaging", some)?;
            }
            if !self.r#drug_characteristic.is_empty() {
                state.serialize_entry("drugCharacteristic", &self.r#drug_characteristic)?;
            }
            if !self.r#contraindication.is_empty() {
                state.serialize_entry("contraindication", &self.r#contraindication)?;
            }
            if !self.r#regulatory.is_empty() {
                state.serialize_entry("regulatory", &self.r#regulatory)?;
            }
            if !self.r#kinetics.is_empty() {
                state.serialize_entry("kinetics", &self.r#kinetics)?;
            }
            state.end()
        })
    }
}
