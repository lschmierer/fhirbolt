// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option), or it indicates the precondition for taking the Medication (CodeableConcept)."]
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
#[doc = "Amount of medication per dose."]
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
#[doc = "Amount of medication per unit of time."]
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
#[doc = "The amount of medication administered."]
#[derive(Default, Debug, Clone)]
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
impl crate::AnyResource for DosageDoseAndRate {}
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "doseRange")]
            DoseRange,
            #[serde(rename = "doseQuantity")]
            DoseQuantity,
            #[serde(rename = "rateRatio")]
            RateRatio,
            #[serde(rename = "rateRange")]
            RateRange,
            #[serde(rename = "rateQuantity")]
            RateQuantity,
            Unknown(String),
        }
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::DoseRange => {
                                if r#dose.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doseRange"));
                                }
                                r#dose =
                                    Some(DosageDoseAndRateDose::Range(map_access.next_value()?));
                            }
                            Field::DoseQuantity => {
                                if r#dose.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doseQuantity"));
                                }
                                r#dose =
                                    Some(DosageDoseAndRateDose::Quantity(map_access.next_value()?));
                            }
                            Field::RateRatio => {
                                if r#rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rateRatio"));
                                }
                                r#rate =
                                    Some(DosageDoseAndRateRate::Ratio(map_access.next_value()?));
                            }
                            Field::RateRange => {
                                if r#rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rateRange"));
                                }
                                r#rate =
                                    Some(DosageDoseAndRateRate::Range(map_access.next_value()?));
                            }
                            Field::RateQuantity => {
                                if r#rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rateQuantity"));
                                }
                                r#rate =
                                    Some(DosageDoseAndRateRate::Quantity(map_access.next_value()?));
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
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
                                    ));
                                }
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Base StructureDefinition for Dosage Type: Indicates how the medication is/was taken or should be taken by the patient."]
#[derive(Default, Debug, Clone)]
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
                let some = Ok(some)?;
                state.serialize_entry("sequence", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("text", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("patientInstruction", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("asNeededBoolean", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "additionalInstruction")]
            AdditionalInstruction,
            #[serde(rename = "patientInstruction")]
            PatientInstruction,
            #[serde(rename = "_patientInstruction")]
            PatientInstructionPrimitiveElement,
            #[serde(rename = "timing")]
            Timing,
            #[serde(rename = "asNeededBoolean")]
            AsNeededBoolean,
            #[serde(rename = "_asNeededBoolean")]
            AsNeededBooleanPrimitiveElement,
            #[serde(rename = "asNeededCodeableConcept")]
            AsNeededCodeableConcept,
            #[serde(rename = "site")]
            Site,
            #[serde(rename = "route")]
            Route,
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "doseAndRate")]
            DoseAndRate,
            #[serde(rename = "maxDosePerPeriod")]
            MaxDosePerPeriod,
            #[serde(rename = "maxDosePerAdministration")]
            MaxDosePerAdministration,
            #[serde(rename = "maxDosePerLifetime")]
            MaxDosePerLifetime,
            Unknown(String),
        }
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
                let mut r#dose_and_rate: Option<Vec<DosageDoseAndRate>> = None;
                let mut r#max_dose_per_period: Option<Box<super::super::types::Ratio>> = None;
                let mut r#max_dose_per_administration: Option<Box<super::super::types::Quantity>> =
                    None;
                let mut r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
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
                            Field::Text => {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TextPrimitiveElement => {
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
                            Field::AdditionalInstruction => {
                                if r#additional_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalInstruction",
                                    ));
                                }
                                r#additional_instruction = Some(map_access.next_value()?);
                            }
                            Field::PatientInstruction => {
                                let some = r#patient_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientInstruction",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PatientInstructionPrimitiveElement => {
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
                            Field::Timing => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timing"));
                                }
                                r#timing = Some(map_access.next_value()?);
                            }
                            Field::AsNeededBoolean => {
                                let r#enum = r#as_needed
                                    .get_or_insert(DosageAsNeeded::Boolean(Default::default()));
                                if let DosageAsNeeded::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "asNeededBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("asNeeded[x]"));
                                }
                            }
                            Field::AsNeededBooleanPrimitiveElement => {
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
                            Field::AsNeededCodeableConcept => {
                                if r#as_needed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "asNeededCodeableConcept",
                                    ));
                                }
                                r#as_needed =
                                    Some(DosageAsNeeded::CodeableConcept(map_access.next_value()?));
                            }
                            Field::Site => {
                                if r#site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                r#site = Some(map_access.next_value()?);
                            }
                            Field::Route => {
                                if r#route.is_some() {
                                    return Err(serde::de::Error::duplicate_field("route"));
                                }
                                r#route = Some(map_access.next_value()?);
                            }
                            Field::Method => {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                r#method = Some(map_access.next_value()?);
                            }
                            Field::DoseAndRate => {
                                if r#dose_and_rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doseAndRate"));
                                }
                                r#dose_and_rate = Some(map_access.next_value()?);
                            }
                            Field::MaxDosePerPeriod => {
                                if r#max_dose_per_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxDosePerPeriod",
                                    ));
                                }
                                r#max_dose_per_period = Some(map_access.next_value()?);
                            }
                            Field::MaxDosePerAdministration => {
                                if r#max_dose_per_administration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxDosePerAdministration",
                                    ));
                                }
                                r#max_dose_per_administration = Some(map_access.next_value()?);
                            }
                            Field::MaxDosePerLifetime => {
                                if r#max_dose_per_lifetime.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxDosePerLifetime",
                                    ));
                                }
                                r#max_dose_per_lifetime = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
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
                                    ));
                                }
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
