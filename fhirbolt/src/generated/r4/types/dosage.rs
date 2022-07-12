// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum DosageAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub enum DosageDoseAndRateRate {
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub enum DosageDoseAndRateDose {
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub struct DosageDoseAndRate {
    pub r#rate: Option<DosageDoseAndRateRate>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#dose: Option<DosageDoseAndRateDose>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::Serialize for DosageDoseAndRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
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
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#dose.as_ref() {
            match some {
                DosageDoseAndRateDose::Range(ref value) => {
                    state.serialize_entry("doseRange", value)?;
                }
                DosageDoseAndRateDose::Quantity(ref value) => {
                    state.serialize_entry("doseQuantity", value)?;
                }
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct Dosage {
    pub r#additional_instruction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#max_dose_per_administration: Option<Box<super::super::types::Quantity>>,
    pub r#patient_instruction: Option<super::super::types::String>,
    pub r#sequence: Option<super::super::types::Integer>,
    pub r#as_needed: Option<DosageAsNeeded>,
    pub r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#max_dose_per_period: Option<Box<super::super::types::Ratio>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#timing: Option<Box<super::super::types::Timing>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#dose_and_rate: Vec<Box<super::super::types::Element>>,
}
impl serde::Serialize for Dosage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#additional_instruction.is_empty() {
            state.serialize_entry("additionalInstruction", &self.r#additional_instruction)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#max_dose_per_administration.as_ref() {
            state.serialize_entry("maxDosePerAdministration", some)?;
        }
        if let Some(some) = self.r#patient_instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("patientInstruction", some)?;
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
                state.serialize_entry("_patientInstruction", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sequence", some)?;
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
                state.serialize_entry("_sequence", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#as_needed.as_ref() {
            match some {
                DosageAsNeeded::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("asNeededBoolean", some)?;
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
                        state.serialize_entry("_asNeededBoolean", &primitive_element)?;
                    }
                }
                DosageAsNeeded::CodeableConcept(ref value) => {
                    state.serialize_entry("asNeededCodeableConcept", value)?;
                }
            }
        }
        if let Some(some) = self.r#max_dose_per_lifetime.as_ref() {
            state.serialize_entry("maxDosePerLifetime", some)?;
        }
        if let Some(some) = self.r#site.as_ref() {
            state.serialize_entry("site", some)?;
        }
        if let Some(some) = self.r#max_dose_per_period.as_ref() {
            state.serialize_entry("maxDosePerPeriod", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#timing.as_ref() {
            state.serialize_entry("timing", some)?;
        }
        if let Some(some) = self.r#route.as_ref() {
            state.serialize_entry("route", some)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
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
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#method.as_ref() {
            state.serialize_entry("method", some)?;
        }
        if !self.r#dose_and_rate.is_empty() {
            state.serialize_entry("doseAndRate", &self.r#dose_and_rate)?;
        }
        state.end()
    }
}
