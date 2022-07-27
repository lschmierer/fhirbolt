// Generated on 2022-07-27 by fhirbolt-codegen v0.1.0
#[doc = "Nominal position of the recommended dose in a series (e.g. dose 2 is the next recommended dose)."]
#[derive(Debug, Clone)]
pub enum ImmunizationRecommendationRecommendationDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ImmunizationRecommendationRecommendationDoseNumber {
    fn default() -> ImmunizationRecommendationRecommendationDoseNumber {
        ImmunizationRecommendationRecommendationDoseNumber::Invalid
    }
}
#[doc = "The recommended number of doses to achieve immunity."]
#[derive(Debug, Clone)]
pub enum ImmunizationRecommendationRecommendationSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ImmunizationRecommendationRecommendationSeriesDoses {
    fn default() -> ImmunizationRecommendationRecommendationSeriesDoses {
        ImmunizationRecommendationRecommendationSeriesDoses::Invalid
    }
}
#[doc = "Vaccine date recommendations.  For example, earliest date to administer, latest date to administer, etc."]
#[derive(Default, Debug, Clone)]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Date classification of recommendation.  For example, earliest date to give, latest date to give, etc."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The date whose meaning is specified by dateCriterion.code."]
    pub r#value: super::super::types::DateTime,
}
impl serde::ser::Serialize for ImmunizationRecommendationRecommendationDateCriterion {
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
        state.serialize_entry("code", &self.r#code)?;
        if let Some(some) = self.r#value.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("value", &some)?;
        }
        if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#value.id,
                extension: &self.r#value.extension,
            };
            state.serialize_entry("_value", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationRecommendationRecommendationDateCriterion {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationRecommendationRecommendationDateCriterion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationRecommendationRecommendationDateCriterion")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImmunizationRecommendationRecommendationDateCriterion, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<super::super::types::DateTime> = None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ValuePrimitiveElement => {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &["id", "extension", "modifierExtension", "code", "value"],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ImmunizationRecommendationRecommendationDateCriterion {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#value: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Vaccine administration recommendations."]
#[derive(Default, Debug, Clone)]
pub struct ImmunizationRecommendationRecommendation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Vaccine(s) or vaccine group that pertain to the recommendation."]
    pub r#vaccine_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The targeted disease for the recommendation."]
    pub r#target_disease: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Vaccine(s) which should not be used to fulfill the recommendation."]
    pub r#contraindicated_vaccine_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the patient status with respect to the path to immunity for the target disease."]
    pub r#forecast_status: Box<super::super::types::CodeableConcept>,
    #[doc = "The reason for the assigned forecast status."]
    pub r#forecast_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Vaccine date recommendations.  For example, earliest date to administer, latest date to administer, etc."]
    pub r#date_criterion: Vec<ImmunizationRecommendationRecommendationDateCriterion>,
    #[doc = "Contains the description about the protocol under which the vaccine was administered."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "One possible path to achieve presumed immunity against a disease - within the context of an authority."]
    pub r#series: Option<super::super::types::String>,
    #[doc = "Nominal position of the recommended dose in a series (e.g. dose 2 is the next recommended dose)."]
    pub r#dose_number: Option<ImmunizationRecommendationRecommendationDoseNumber>,
    #[doc = "The recommended number of doses to achieve immunity."]
    pub r#series_doses: Option<ImmunizationRecommendationRecommendationSeriesDoses>,
    #[doc = "Immunization event history and/or evaluation that supports the status and recommendation."]
    pub r#supporting_immunization: Vec<Box<super::super::types::Reference>>,
    #[doc = "Patient Information that supports the status and recommendation.  This includes patient observations, adverse reactions and allergy/intolerance information."]
    pub r#supporting_patient_information: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ImmunizationRecommendationRecommendation {
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
        if !self.r#vaccine_code.is_empty() {
            state.serialize_entry("vaccineCode", &self.r#vaccine_code)?;
        }
        if let Some(some) = self.r#target_disease.as_ref() {
            state.serialize_entry("targetDisease", some)?;
        }
        if !self.r#contraindicated_vaccine_code.is_empty() {
            state.serialize_entry(
                "contraindicatedVaccineCode",
                &self.r#contraindicated_vaccine_code,
            )?;
        }
        state.serialize_entry("forecastStatus", &self.r#forecast_status)?;
        if !self.r#forecast_reason.is_empty() {
            state.serialize_entry("forecastReason", &self.r#forecast_reason)?;
        }
        if !self.r#date_criterion.is_empty() {
            state.serialize_entry("dateCriterion", &self.r#date_criterion)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("description", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#series.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("series", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_series", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#dose_number.as_ref() {
            match some {
                ImmunizationRecommendationRecommendationDoseNumber::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doseNumberPositiveInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_doseNumberPositiveInt", &primitive_element)?;
                    }
                }
                ImmunizationRecommendationRecommendationDoseNumber::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doseNumberString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_doseNumberString", &primitive_element)?;
                    }
                }
                ImmunizationRecommendationRecommendationDoseNumber::Invalid => {
                    return Err(serde::ser::Error::custom("dose_number is invalid"))
                }
            }
        }
        if let Some(some) = self.r#series_doses.as_ref() {
            match some {
                ImmunizationRecommendationRecommendationSeriesDoses::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("seriesDosesPositiveInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_seriesDosesPositiveInt", &primitive_element)?;
                    }
                }
                ImmunizationRecommendationRecommendationSeriesDoses::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("seriesDosesString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_seriesDosesString", &primitive_element)?;
                    }
                }
                ImmunizationRecommendationRecommendationSeriesDoses::Invalid => {
                    return Err(serde::ser::Error::custom("series_doses is invalid"))
                }
            }
        }
        if !self.r#supporting_immunization.is_empty() {
            state.serialize_entry("supportingImmunization", &self.r#supporting_immunization)?;
        }
        if !self.r#supporting_patient_information.is_empty() {
            state.serialize_entry(
                "supportingPatientInformation",
                &self.r#supporting_patient_information,
            )?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationRecommendationRecommendation {
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
            #[serde(rename = "vaccineCode")]
            VaccineCode,
            #[serde(rename = "targetDisease")]
            TargetDisease,
            #[serde(rename = "contraindicatedVaccineCode")]
            ContraindicatedVaccineCode,
            #[serde(rename = "forecastStatus")]
            ForecastStatus,
            #[serde(rename = "forecastReason")]
            ForecastReason,
            #[serde(rename = "dateCriterion")]
            DateCriterion,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "series")]
            Series,
            #[serde(rename = "_series")]
            SeriesPrimitiveElement,
            #[serde(rename = "doseNumberPositiveInt")]
            DoseNumberPositiveInt,
            #[serde(rename = "_doseNumberPositiveInt")]
            DoseNumberPositiveIntPrimitiveElement,
            #[serde(rename = "doseNumberString")]
            DoseNumberString,
            #[serde(rename = "_doseNumberString")]
            DoseNumberStringPrimitiveElement,
            #[serde(rename = "seriesDosesPositiveInt")]
            SeriesDosesPositiveInt,
            #[serde(rename = "_seriesDosesPositiveInt")]
            SeriesDosesPositiveIntPrimitiveElement,
            #[serde(rename = "seriesDosesString")]
            SeriesDosesString,
            #[serde(rename = "_seriesDosesString")]
            SeriesDosesStringPrimitiveElement,
            #[serde(rename = "supportingImmunization")]
            SupportingImmunization,
            #[serde(rename = "supportingPatientInformation")]
            SupportingPatientInformation,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationRecommendationRecommendation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationRecommendationRecommendation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImmunizationRecommendationRecommendation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#vaccine_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#target_disease: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#contraindicated_vaccine_code: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#forecast_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#forecast_reason: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#date_criterion: Option<
                    Vec<ImmunizationRecommendationRecommendationDateCriterion>,
                > = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#series: Option<super::super::types::String> = None;
                let mut r#dose_number: Option<ImmunizationRecommendationRecommendationDoseNumber> =
                    None;
                let mut r#series_doses: Option<
                    ImmunizationRecommendationRecommendationSeriesDoses,
                > = None;
                let mut r#supporting_immunization: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                let mut r#supporting_patient_information: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                crate :: json :: de :: DESERIALIZATION_CONFIG . with (| config | { let config = config . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: VaccineCode => { if r#vaccine_code . is_some () { return Err (serde :: de :: Error :: duplicate_field ("vaccineCode")) ; } r#vaccine_code = Some (map_access . next_value () ?) ; } , Field :: TargetDisease => { if r#target_disease . is_some () { return Err (serde :: de :: Error :: duplicate_field ("targetDisease")) ; } r#target_disease = Some (map_access . next_value () ?) ; } , Field :: ContraindicatedVaccineCode => { if r#contraindicated_vaccine_code . is_some () { return Err (serde :: de :: Error :: duplicate_field ("contraindicatedVaccineCode")) ; } r#contraindicated_vaccine_code = Some (map_access . next_value () ?) ; } , Field :: ForecastStatus => { if r#forecast_status . is_some () { return Err (serde :: de :: Error :: duplicate_field ("forecastStatus")) ; } r#forecast_status = Some (map_access . next_value () ?) ; } , Field :: ForecastReason => { if r#forecast_reason . is_some () { return Err (serde :: de :: Error :: duplicate_field ("forecastReason")) ; } r#forecast_reason = Some (map_access . next_value () ?) ; } , Field :: DateCriterion => { if r#date_criterion . is_some () { return Err (serde :: de :: Error :: duplicate_field ("dateCriterion")) ; } r#date_criterion = Some (map_access . next_value () ?) ; } , Field :: Description => { let some = r#description . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("description")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } , Field :: DescriptionPrimitiveElement => { let some = r#description . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_description")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } , Field :: Series => { let some = r#series . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("series")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } , Field :: SeriesPrimitiveElement => { let some = r#series . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_series")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } , Field :: DoseNumberPositiveInt => { let r#enum = r#dose_number . get_or_insert (ImmunizationRecommendationRecommendationDoseNumber :: PositiveInt (Default :: default ())) ; if let ImmunizationRecommendationRecommendationDoseNumber :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("doseNumberPositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("doseNumber[x]")) ; } } , Field :: DoseNumberPositiveIntPrimitiveElement => { let r#enum = r#dose_number . get_or_insert (ImmunizationRecommendationRecommendationDoseNumber :: PositiveInt (Default :: default ())) ; if let ImmunizationRecommendationRecommendationDoseNumber :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_doseNumberPositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_doseNumber[x]")) ; } } , Field :: DoseNumberString => { let r#enum = r#dose_number . get_or_insert (ImmunizationRecommendationRecommendationDoseNumber :: String (Default :: default ())) ; if let ImmunizationRecommendationRecommendationDoseNumber :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("doseNumberString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("doseNumber[x]")) ; } } , Field :: DoseNumberStringPrimitiveElement => { let r#enum = r#dose_number . get_or_insert (ImmunizationRecommendationRecommendationDoseNumber :: String (Default :: default ())) ; if let ImmunizationRecommendationRecommendationDoseNumber :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_doseNumberString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_doseNumber[x]")) ; } } , Field :: SeriesDosesPositiveInt => { let r#enum = r#series_doses . get_or_insert (ImmunizationRecommendationRecommendationSeriesDoses :: PositiveInt (Default :: default ())) ; if let ImmunizationRecommendationRecommendationSeriesDoses :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("seriesDosesPositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("seriesDoses[x]")) ; } } , Field :: SeriesDosesPositiveIntPrimitiveElement => { let r#enum = r#series_doses . get_or_insert (ImmunizationRecommendationRecommendationSeriesDoses :: PositiveInt (Default :: default ())) ; if let ImmunizationRecommendationRecommendationSeriesDoses :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_seriesDosesPositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_seriesDoses[x]")) ; } } , Field :: SeriesDosesString => { let r#enum = r#series_doses . get_or_insert (ImmunizationRecommendationRecommendationSeriesDoses :: String (Default :: default ())) ; if let ImmunizationRecommendationRecommendationSeriesDoses :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("seriesDosesString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("seriesDoses[x]")) ; } } , Field :: SeriesDosesStringPrimitiveElement => { let r#enum = r#series_doses . get_or_insert (ImmunizationRecommendationRecommendationSeriesDoses :: String (Default :: default ())) ; if let ImmunizationRecommendationRecommendationSeriesDoses :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_seriesDosesString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_seriesDoses[x]")) ; } } , Field :: SupportingImmunization => { if r#supporting_immunization . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingImmunization")) ; } r#supporting_immunization = Some (map_access . next_value () ?) ; } , Field :: SupportingPatientInformation => { if r#supporting_patient_information . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingPatientInformation")) ; } r#supporting_patient_information = Some (map_access . next_value () ?) ; } , Field :: Unknown (key) => if config . mode == crate :: json :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "vaccineCode" , "targetDisease" , "contraindicatedVaccineCode" , "forecastStatus" , "forecastReason" , "dateCriterion" , "description" , "series" , "doseNumberPositiveInt" , "doseNumberString" , "seriesDosesPositiveInt" , "seriesDosesString" , "supportingImmunization" , "supportingPatientInformation" ,])) ; } } } Ok (ImmunizationRecommendationRecommendation { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#vaccine_code : r#vaccine_code . unwrap_or (vec ! []) , r#target_disease , r#contraindicated_vaccine_code : r#contraindicated_vaccine_code . unwrap_or (vec ! []) , r#forecast_status : if config . mode == crate :: json :: de :: DeserializationMode :: Lax { r#forecast_status . unwrap_or (Default :: default ()) } else { r#forecast_status . ok_or (serde :: de :: Error :: missing_field ("forecastStatus")) ? } , r#forecast_reason : r#forecast_reason . unwrap_or (vec ! []) , r#date_criterion : r#date_criterion . unwrap_or (vec ! []) , r#description , r#series , r#dose_number , r#series_doses , r#supporting_immunization : r#supporting_immunization . unwrap_or (vec ! []) , r#supporting_patient_information : r#supporting_patient_information . unwrap_or (vec ! []) , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification."]
#[derive(Default, Debug, Clone)]
pub struct ImmunizationRecommendation {
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
    #[doc = "A unique identifier assigned to this particular recommendation record."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The patient the recommendation(s) are for."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date the immunization recommendation(s) were created."]
    pub r#date: super::super::types::DateTime,
    #[doc = "Indicates the authority who published the protocol (e.g. ACIP)."]
    pub r#authority: Option<Box<super::super::types::Reference>>,
    #[doc = "Vaccine administration recommendations."]
    pub r#recommendation: Vec<ImmunizationRecommendationRecommendation>,
}
impl serde::ser::Serialize for ImmunizationRecommendation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ImmunizationRecommendation")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#date.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("date", &some)?;
        }
        if self.r#date.id.is_some() || !self.r#date.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#date.id,
                extension: &self.r#date.extension,
            };
            state.serialize_entry("_date", &primitive_element)?;
        }
        if let Some(some) = self.r#authority.as_ref() {
            state.serialize_entry("authority", some)?;
        }
        if !self.r#recommendation.is_empty() {
            state.serialize_entry("recommendation", &self.r#recommendation)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationRecommendation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "authority")]
            Authority,
            #[serde(rename = "recommendation")]
            Recommendation,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationRecommendation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationRecommendation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImmunizationRecommendation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#authority: Option<Box<super::super::types::Reference>> = None;
                let mut r#recommendation: Option<Vec<ImmunizationRecommendationRecommendation>> =
                    None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ImmunizationRecommendation" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ImmunizationRecommendation",
                                    ));
                                }
                            }
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Meta => {
                                if r#meta.is_some() {
                                    return Err(serde::de::Error::duplicate_field("meta"));
                                }
                                r#meta = Some(map_access.next_value()?);
                            }
                            Field::ImplicitRules => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Language => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LanguagePrimitiveElement => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value()?);
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::Date => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DatePrimitiveElement => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_date"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Authority => {
                                if r#authority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authority"));
                                }
                                r#authority = Some(map_access.next_value()?);
                            }
                            Field::Recommendation => {
                                if r#recommendation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "recommendation",
                                    ));
                                }
                                r#recommendation = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "patient",
                                            "date",
                                            "authority",
                                            "recommendation",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ImmunizationRecommendation {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#patient: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#date: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#date.unwrap_or(Default::default())
                        } else {
                            r#date.ok_or(serde::de::Error::missing_field("date"))?
                        },
                        r#authority,
                        r#recommendation: r#recommendation.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
