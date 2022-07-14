// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Default, Debug, Clone)]
pub struct DosageDoseAndRate {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#dose: Option<DosageDoseAndRateDose>,
    pub r#rate: Option<DosageDoseAndRateRate>,
}
impl serde::ser::Serialize for DosageDoseAndRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
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
    }
}
impl<'de> serde::de::Deserialize<'de> for DosageDoseAndRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DosageDoseAndRate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DosageDoseAndRate")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DosageDoseAndRate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#dose: Option<DosageDoseAndRateDose> = None;
                let mut r#rate: Option<DosageDoseAndRateRate> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "doseRange" => {
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseRange"));
                            }
                            r#dose = Some(DosageDoseAndRateDose::Range(map_access.next_value()?));
                        }
                        "doseQuantity" => {
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseQuantity"));
                            }
                            r#dose =
                                Some(DosageDoseAndRateDose::Quantity(map_access.next_value()?));
                        }
                        "rateRatio" => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            r#rate = Some(DosageDoseAndRateRate::Ratio(map_access.next_value()?));
                        }
                        "rateRange" => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRange"));
                            }
                            r#rate = Some(DosageDoseAndRateRate::Range(map_access.next_value()?));
                        }
                        "rateQuantity" => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            r#rate =
                                Some(DosageDoseAndRateRate::Quantity(map_access.next_value()?));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "type",
                                    "doseRange",
                                    "doseQuantity",
                                    "rateRatio",
                                    "rateRange",
                                    "rateQuantity",
                                ],
                            ))
                        }
                    }
                }
                Ok(DosageDoseAndRate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type,
                    r#dose,
                    r#rate,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Dosage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: Option<super::super::types::Integer>,
    pub r#text: Option<super::super::types::String>,
    pub r#additional_instruction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#patient_instruction: Option<super::super::types::String>,
    pub r#timing: Option<Box<super::super::types::Timing>>,
    pub r#as_needed: Option<DosageAsNeeded>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#dose_and_rate: Vec<Box<super::super::types::Element>>,
    pub r#max_dose_per_period: Option<Box<super::super::types::Ratio>>,
    pub r#max_dose_per_administration: Option<Box<super::super::types::Quantity>>,
    pub r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for Dosage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if let Some(some) = self.r#sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sequence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sequence", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if !self.r#additional_instruction.is_empty() {
            state.serialize_entry("additionalInstruction", &self.r#additional_instruction)?;
        }
        if let Some(some) = self.r#patient_instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("patientInstruction", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_patientInstruction", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#timing.as_ref() {
            state.serialize_entry("timing", some)?;
        }
        if let Some(some) = self.r#as_needed.as_ref() {
            match some {
                DosageAsNeeded::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("asNeededBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_asNeededBoolean", &primitive_element)?;
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
    }
}
impl<'de> serde::de::Deserialize<'de> for Dosage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Dosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Dosage")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Dosage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::Integer> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#additional_instruction: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#patient_instruction: Option<super::super::types::String> = None;
                let mut r#timing: Option<Box<super::super::types::Timing>> = None;
                let mut r#as_needed: Option<DosageAsNeeded> = None;
                let mut r#site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#route: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#dose_and_rate: Option<Vec<Box<super::super::types::Element>>> = None;
                let mut r#max_dose_per_period: Option<Box<super::super::types::Ratio>> = None;
                let mut r#max_dose_per_administration: Option<Box<super::super::types::Quantity>> =
                    None;
                let mut r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "additionalInstruction" => {
                            if r#additional_instruction.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additionalInstruction",
                                ));
                            }
                            r#additional_instruction = Some(map_access.next_value()?);
                        }
                        "patientInstruction" => {
                            let some = r#patient_instruction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patientInstruction",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_patientInstruction" => {
                            let some = r#patient_instruction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_patientInstruction",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "timing" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timing"));
                            }
                            r#timing = Some(map_access.next_value()?);
                        }
                        "asNeededBoolean" => {
                            let r#enum = r#as_needed
                                .get_or_insert(DosageAsNeeded::Boolean(Default::default()));
                            if let DosageAsNeeded::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "asNeededBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("asNeeded[x]"));
                            }
                        }
                        "_asNeededBoolean" => {
                            let r#enum = r#as_needed
                                .get_or_insert(DosageAsNeeded::Boolean(Default::default()));
                            if let DosageAsNeeded::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_asNeededBoolean",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_asNeeded[x]"));
                            }
                        }
                        "asNeededCodeableConcept" => {
                            if r#as_needed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "asNeededCodeableConcept",
                                ));
                            }
                            r#as_needed =
                                Some(DosageAsNeeded::CodeableConcept(map_access.next_value()?));
                        }
                        "site" => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            r#site = Some(map_access.next_value()?);
                        }
                        "route" => {
                            if r#route.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            r#route = Some(map_access.next_value()?);
                        }
                        "method" => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some(map_access.next_value()?);
                        }
                        "doseAndRate" => {
                            if r#dose_and_rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseAndRate"));
                            }
                            r#dose_and_rate = Some(map_access.next_value()?);
                        }
                        "maxDosePerPeriod" => {
                            if r#max_dose_per_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDosePerPeriod"));
                            }
                            r#max_dose_per_period = Some(map_access.next_value()?);
                        }
                        "maxDosePerAdministration" => {
                            if r#max_dose_per_administration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerAdministration",
                                ));
                            }
                            r#max_dose_per_administration = Some(map_access.next_value()?);
                        }
                        "maxDosePerLifetime" => {
                            if r#max_dose_per_lifetime.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerLifetime",
                                ));
                            }
                            r#max_dose_per_lifetime = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifierExtension",
                                    "sequence",
                                    "text",
                                    "additionalInstruction",
                                    "patientInstruction",
                                    "timing",
                                    "asNeededBoolean",
                                    "asNeededCodeableConcept",
                                    "site",
                                    "route",
                                    "method",
                                    "doseAndRate",
                                    "maxDosePerPeriod",
                                    "maxDosePerAdministration",
                                    "maxDosePerLifetime",
                                ],
                            ))
                        }
                    }
                }
                Ok(Dosage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence,
                    r#text,
                    r#additional_instruction: r#additional_instruction.unwrap_or(vec![]),
                    r#patient_instruction,
                    r#timing,
                    r#as_needed,
                    r#site,
                    r#route,
                    r#method,
                    r#dose_and_rate: r#dose_and_rate.unwrap_or(vec![]),
                    r#max_dose_per_period,
                    r#max_dose_per_administration,
                    r#max_dose_per_lifetime,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
