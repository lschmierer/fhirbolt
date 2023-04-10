// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option), or it indicates the precondition for taking the Medication (CodeableConcept)."]
#[derive(Debug, Clone, PartialEq)]
pub enum DosageAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for DosageAsNeeded {
    fn default() -> DosageAsNeeded {
        DosageAsNeeded::Invalid
    }
}
#[doc = "Amount of medication per dose."]
#[derive(Debug, Clone, PartialEq)]
pub enum DosageDoseAndRateDose {
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for DosageDoseAndRateDose {
    fn default() -> DosageDoseAndRateDose {
        DosageDoseAndRateDose::Invalid
    }
}
#[doc = "Amount of medication per unit of time."]
#[derive(Debug, Clone, PartialEq)]
pub enum DosageDoseAndRateRate {
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for DosageDoseAndRateRate {
    fn default() -> DosageDoseAndRateRate {
        DosageDoseAndRateRate::Invalid
    }
}
#[doc = "The amount of medication administered."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DosageDoseAndRate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of dose or rate specified, for example, ordered or calculated."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Amount of medication per dose."]
    pub r#dose: Option<DosageDoseAndRateDose>,
    #[doc = "Amount of medication per unit of time."]
    pub r#rate: Option<DosageDoseAndRateRate>,
}
impl serde::ser::Serialize for DosageDoseAndRate {
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#dose.as_ref() {
                match some {
                    DosageDoseAndRateDose::Range(ref value) => {
                        state.serialize_entry("doseRange", value)?;
                    }
                    DosageDoseAndRateDose::Quantity(ref value) => {
                        state.serialize_entry("doseQuantity", value)?;
                    }
                    DosageDoseAndRateDose::Invalid => {
                        return Err(serde::ser::Error::custom("dose is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#rate.as_ref() {
                match some {
                    DosageDoseAndRateRate::Ratio(ref value) => {
                        state.serialize_entry("rateRatio", value)?;
                    }
                    DosageDoseAndRateRate::Range(ref value) => {
                        state.serialize_entry("rateRange", value)?;
                    }
                    DosageDoseAndRateRate::Quantity(ref value) => {
                        state.serialize_entry("rateQuantity", value)?;
                    }
                    DosageDoseAndRateRate::Invalid => {
                        return Err(serde::ser::Error::custom("rate is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Base StructureDefinition for Dosage Type: Indicates how the medication is/was taken or should be taken by the patient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dosage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates the order in which the dosage instructions should be applied or interpreted."]
    pub r#sequence: Option<super::super::types::Integer>,
    #[doc = "Free text dosage instructions e.g. SIG."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Supplemental instructions to the patient on how to take the medication  (e.g. \"with meals\" or\"take half to one hour before food\") or warnings for the patient about the medication (e.g. \"may cause drowsiness\" or \"avoid exposure of skin to direct sunlight or sunlamps\")."]
    pub r#additional_instruction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Instructions in terms that are understood by the patient or consumer."]
    pub r#patient_instruction: Option<super::super::types::String>,
    #[doc = "When medication should be administered."]
    pub r#timing: Option<Box<super::super::types::Timing>>,
    #[doc = "Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option), or it indicates the precondition for taking the Medication (CodeableConcept)."]
    pub r#as_needed: Option<DosageAsNeeded>,
    #[doc = "Body site to administer to."]
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How drug should enter body."]
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Technique for administering medication."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The amount of medication administered."]
    pub r#dose_and_rate: Vec<DosageDoseAndRate>,
    #[doc = "Upper limit on medication per unit of time."]
    pub r#max_dose_per_period: Option<Box<super::super::types::Ratio>>,
    #[doc = "Upper limit on medication per administration."]
    pub r#max_dose_per_administration: Option<Box<super::super::types::Quantity>>,
    #[doc = "Upper limit on medication per lifetime of the patient."]
    pub r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for Dosage {
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
            if _ctx.output_json {
                if let Some(some) = self.r#sequence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sequence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sequence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sequence.as_ref() {
                    state.serialize_entry("sequence", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if !self.r#additional_instruction.is_empty() {
                state.serialize_entry("additionalInstruction", &self.r#additional_instruction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#patient_instruction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("patientInstruction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_patientInstruction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#patient_instruction.as_ref() {
                    state.serialize_entry("patientInstruction", some)?;
                }
            }
            if let Some(some) = self.r#timing.as_ref() {
                state.serialize_entry("timing", some)?;
            }
            if let Some(some) = self.r#as_needed.as_ref() {
                match some {
                    DosageAsNeeded::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("asNeededBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_asNeededBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("asNeededBoolean", value)?;
                        }
                    }
                    DosageAsNeeded::CodeableConcept(ref value) => {
                        state.serialize_entry("asNeededCodeableConcept", value)?;
                    }
                    DosageAsNeeded::Invalid => {
                        return Err(serde::ser::Error::custom("as_needed is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#site.as_ref() {
                state.serialize_entry("site", some)?;
            }
            if let Some(some) = self.r#route.as_ref() {
                state.serialize_entry("route", some)?;
            }
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if !self.r#dose_and_rate.is_empty() {
                state.serialize_entry("doseAndRate", &self.r#dose_and_rate)?;
            }
            if let Some(some) = self.r#max_dose_per_period.as_ref() {
                state.serialize_entry("maxDosePerPeriod", some)?;
            }
            if let Some(some) = self.r#max_dose_per_administration.as_ref() {
                state.serialize_entry("maxDosePerAdministration", some)?;
            }
            if let Some(some) = self.r#max_dose_per_lifetime.as_ref() {
                state.serialize_entry("maxDosePerLifetime", some)?;
            }
            state.end()
        })
    }
}
