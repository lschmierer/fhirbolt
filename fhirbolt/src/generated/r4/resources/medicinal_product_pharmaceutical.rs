// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceuticalCharacteristics {
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
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPharmaceuticalCharacteristics {
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
            #[serde(rename = "status")]
            Status,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductPharmaceuticalCharacteristics;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPharmaceuticalCharacteristics")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductPharmaceuticalCharacteristics, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Status => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductPharmaceuticalCharacteristics {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#status,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#tissue: Box<super::super::types::CodeableConcept>,
    pub r#value: Box<super::super::types::Quantity>,
    pub r#supporting_information: Option<super::super::types::String>,
}
impl serde::ser::Serialize
    for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod
{
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
        state.serialize_entry("tissue", &self.r#tissue)?;
        state.serialize_entry("value", &self.r#value)?;
        if let Some(some) = self.r#supporting_information.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("supportingInformation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_supportingInformation", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de>
    for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod
{
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
            #[serde(rename = "tissue")]
            Tissue,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "supportingInformation")]
            SupportingInformation,
            #[serde(rename = "_supportingInformation")]
            SupportingInformationPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value =
                MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter . write_str ("MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#tissue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<Box<super::super::types::Quantity>> = None;
                let mut r#supporting_information: Option<super::super::types::String> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Tissue => {
                            if r#tissue.is_some() {
                                return Err(serde::de::Error::duplicate_field("tissue"));
                            }
                            r#tissue = Some(map_access.next_value()?);
                        }
                        Field::Value => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            r#value = Some(map_access.next_value()?);
                        }
                        Field::SupportingInformation => {
                            let some = r#supporting_information.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "supportingInformation",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SupportingInformationPrimitiveElement => {
                            let some = r#supporting_information.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_supportingInformation",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok (MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#tissue : r#tissue . ok_or (serde :: de :: Error :: missing_field ("tissue")) ? , r#value : r#value . ok_or (serde :: de :: Error :: missing_field ("value")) ? , r#supporting_information , })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#withdrawal_period:
        Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
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
        if !self.r#withdrawal_period.is_empty() {
            state.serialize_entry("withdrawalPeriod", &self.r#withdrawal_period)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de>
    for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies
{
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
            #[serde(rename = "withdrawalPeriod")]
            WithdrawalPeriod,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter
                    .write_str("MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#withdrawal_period : Option < Vec < MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > > = None ;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::WithdrawalPeriod => {
                            if r#withdrawal_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawalPeriod"));
                            }
                            r#withdrawal_period = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(
                    MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                        r#withdrawal_period: r#withdrawal_period.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministration {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#first_dose: Option<Box<super::super::types::Quantity>>,
    pub r#max_single_dose: Option<Box<super::super::types::Quantity>>,
    pub r#max_dose_per_day: Option<Box<super::super::types::Quantity>>,
    pub r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>>,
    pub r#max_treatment_period: Option<Box<super::super::types::Duration>>,
    pub r#target_species: Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceuticalRouteOfAdministration {
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
        if let Some(some) = self.r#first_dose.as_ref() {
            state.serialize_entry("firstDose", some)?;
        }
        if let Some(some) = self.r#max_single_dose.as_ref() {
            state.serialize_entry("maxSingleDose", some)?;
        }
        if let Some(some) = self.r#max_dose_per_day.as_ref() {
            state.serialize_entry("maxDosePerDay", some)?;
        }
        if let Some(some) = self.r#max_dose_per_treatment_period.as_ref() {
            state.serialize_entry("maxDosePerTreatmentPeriod", some)?;
        }
        if let Some(some) = self.r#max_treatment_period.as_ref() {
            state.serialize_entry("maxTreatmentPeriod", some)?;
        }
        if !self.r#target_species.is_empty() {
            state.serialize_entry("targetSpecies", &self.r#target_species)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPharmaceuticalRouteOfAdministration {
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
            #[serde(rename = "firstDose")]
            FirstDose,
            #[serde(rename = "maxSingleDose")]
            MaxSingleDose,
            #[serde(rename = "maxDosePerDay")]
            MaxDosePerDay,
            #[serde(rename = "maxDosePerTreatmentPeriod")]
            MaxDosePerTreatmentPeriod,
            #[serde(rename = "maxTreatmentPeriod")]
            MaxTreatmentPeriod,
            #[serde(rename = "targetSpecies")]
            TargetSpecies,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductPharmaceuticalRouteOfAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPharmaceuticalRouteOfAdministration")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductPharmaceuticalRouteOfAdministration, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#first_dose: Option<Box<super::super::types::Quantity>> = None;
                let mut r#max_single_dose: Option<Box<super::super::types::Quantity>> = None;
                let mut r#max_dose_per_day: Option<Box<super::super::types::Quantity>> = None;
                let mut r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>> =
                    None;
                let mut r#max_treatment_period: Option<Box<super::super::types::Duration>> = None;
                let mut r#target_species: Option<
                    Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>,
                > = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::FirstDose => {
                            if r#first_dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstDose"));
                            }
                            r#first_dose = Some(map_access.next_value()?);
                        }
                        Field::MaxSingleDose => {
                            if r#max_single_dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSingleDose"));
                            }
                            r#max_single_dose = Some(map_access.next_value()?);
                        }
                        Field::MaxDosePerDay => {
                            if r#max_dose_per_day.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDosePerDay"));
                            }
                            r#max_dose_per_day = Some(map_access.next_value()?);
                        }
                        Field::MaxDosePerTreatmentPeriod => {
                            if r#max_dose_per_treatment_period.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerTreatmentPeriod",
                                ));
                            }
                            r#max_dose_per_treatment_period = Some(map_access.next_value()?);
                        }
                        Field::MaxTreatmentPeriod => {
                            if r#max_treatment_period.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxTreatmentPeriod",
                                ));
                            }
                            r#max_treatment_period = Some(map_access.next_value()?);
                        }
                        Field::TargetSpecies => {
                            if r#target_species.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetSpecies"));
                            }
                            r#target_species = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductPharmaceuticalRouteOfAdministration {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#first_dose,
                    r#max_single_dose,
                    r#max_dose_per_day,
                    r#max_dose_per_treatment_period,
                    r#max_treatment_period,
                    r#target_species: r#target_species.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPharmaceutical {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#administrable_dose_form: Box<super::super::types::CodeableConcept>,
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#ingredient: Vec<Box<super::super::types::Reference>>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#characteristics: Vec<MedicinalProductPharmaceuticalCharacteristics>,
    pub r#route_of_administration: Vec<MedicinalProductPharmaceuticalRouteOfAdministration>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceutical {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductPharmaceutical")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
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
                state.serialize_entry("language", some)?;
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
        state.serialize_entry("administrableDoseForm", &self.r#administrable_dose_form)?;
        if let Some(some) = self.r#unit_of_presentation.as_ref() {
            state.serialize_entry("unitOfPresentation", some)?;
        }
        if !self.r#ingredient.is_empty() {
            state.serialize_entry("ingredient", &self.r#ingredient)?;
        }
        if !self.r#device.is_empty() {
            state.serialize_entry("device", &self.r#device)?;
        }
        if !self.r#characteristics.is_empty() {
            state.serialize_entry("characteristics", &self.r#characteristics)?;
        }
        if !self.r#route_of_administration.is_empty() {
            state.serialize_entry("routeOfAdministration", &self.r#route_of_administration)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPharmaceutical {
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
            #[serde(rename = "administrableDoseForm")]
            AdministrableDoseForm,
            #[serde(rename = "unitOfPresentation")]
            UnitOfPresentation,
            #[serde(rename = "ingredient")]
            Ingredient,
            #[serde(rename = "device")]
            Device,
            #[serde(rename = "characteristics")]
            Characteristics,
            #[serde(rename = "routeOfAdministration")]
            RouteOfAdministration,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductPharmaceutical;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPharmaceutical")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductPharmaceutical, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#administrable_dose_form: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#ingredient: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#device: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#characteristics: Option<
                    Vec<MedicinalProductPharmaceuticalCharacteristics>,
                > = None;
                let mut r#route_of_administration: Option<
                    Vec<MedicinalProductPharmaceuticalRouteOfAdministration>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProductPharmaceutical" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProductPharmaceutical",
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
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
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
                            some.value = Some(map_access.next_value()?);
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::AdministrableDoseForm => {
                            if r#administrable_dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "administrableDoseForm",
                                ));
                            }
                            r#administrable_dose_form = Some(map_access.next_value()?);
                        }
                        Field::UnitOfPresentation => {
                            if r#unit_of_presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unitOfPresentation",
                                ));
                            }
                            r#unit_of_presentation = Some(map_access.next_value()?);
                        }
                        Field::Ingredient => {
                            if r#ingredient.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingredient"));
                            }
                            r#ingredient = Some(map_access.next_value()?);
                        }
                        Field::Device => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(map_access.next_value()?);
                        }
                        Field::Characteristics => {
                            if r#characteristics.is_some() {
                                return Err(serde::de::Error::duplicate_field("characteristics"));
                            }
                            r#characteristics = Some(map_access.next_value()?);
                        }
                        Field::RouteOfAdministration => {
                            if r#route_of_administration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "routeOfAdministration",
                                ));
                            }
                            r#route_of_administration = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductPharmaceutical {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#administrable_dose_form: r#administrable_dose_form
                        .ok_or(serde::de::Error::missing_field("administrableDoseForm"))?,
                    r#unit_of_presentation,
                    r#ingredient: r#ingredient.unwrap_or(vec![]),
                    r#device: r#device.unwrap_or(vec![]),
                    r#characteristics: r#characteristics.unwrap_or(vec![]),
                    r#route_of_administration: r#route_of_administration.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
