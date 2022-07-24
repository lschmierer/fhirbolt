// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductIndicationOtherTherapyMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicinalProductIndicationOtherTherapyMedication {
    fn default() -> MedicinalProductIndicationOtherTherapyMedication {
        MedicinalProductIndicationOtherTherapyMedication::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductIndicationOtherTherapy {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#therapy_relationship_type: Box<super::super::types::CodeableConcept>,
    pub r#medication: MedicinalProductIndicationOtherTherapyMedication,
}
impl serde::ser::Serialize for MedicinalProductIndicationOtherTherapy {
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
        state.serialize_entry("therapyRelationshipType", &self.r#therapy_relationship_type)?;
        match self.r#medication {
            MedicinalProductIndicationOtherTherapyMedication::CodeableConcept(ref value) => {
                state.serialize_entry("medicationCodeableConcept", value)?;
            }
            MedicinalProductIndicationOtherTherapyMedication::Reference(ref value) => {
                state.serialize_entry("medicationReference", value)?;
            }
            MedicinalProductIndicationOtherTherapyMedication::Invalid => {
                return Err(serde::ser::Error::custom("medication is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductIndicationOtherTherapy {
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
            #[serde(rename = "therapyRelationshipType")]
            TherapyRelationshipType,
            #[serde(rename = "medicationCodeableConcept")]
            MedicationCodeableConcept,
            #[serde(rename = "medicationReference")]
            MedicationReference,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductIndicationOtherTherapy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIndicationOtherTherapy")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductIndicationOtherTherapy, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#therapy_relationship_type: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#medication: Option<MedicinalProductIndicationOtherTherapyMedication> =
                    None;
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
                        Field::TherapyRelationshipType => {
                            if r#therapy_relationship_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "therapyRelationshipType",
                                ));
                            }
                            r#therapy_relationship_type = Some(map_access.next_value()?);
                        }
                        Field::MedicationCodeableConcept => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationCodeableConcept",
                                ));
                            }
                            r#medication = Some(
                                MedicinalProductIndicationOtherTherapyMedication::CodeableConcept(
                                    map_access.next_value()?,
                                ),
                            );
                        }
                        Field::MedicationReference => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationReference",
                                ));
                            }
                            r#medication =
                                Some(MedicinalProductIndicationOtherTherapyMedication::Reference(
                                    map_access.next_value()?,
                                ));
                        }
                    }
                }
                Ok(MedicinalProductIndicationOtherTherapy {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#therapy_relationship_type: r#therapy_relationship_type
                        .ok_or(serde::de::Error::missing_field("therapy_relationship_type"))?,
                    r#medication: r#medication
                        .ok_or(serde::de::Error::missing_field("medication"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductIndication {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#disease_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#comorbidity: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#intended_effect: Option<Box<super::super::types::CodeableConcept>>,
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    pub r#other_therapy: Vec<MedicinalProductIndicationOtherTherapy>,
    pub r#undesirable_effect: Vec<Box<super::super::types::Reference>>,
    pub r#population: Vec<Box<super::super::types::Population>>,
}
impl serde::ser::Serialize for MedicinalProductIndication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductIndication")?;
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
        if !self.r#subject.is_empty() {
            state.serialize_entry("subject", &self.r#subject)?;
        }
        if let Some(some) = self.r#disease_symptom_procedure.as_ref() {
            state.serialize_entry("diseaseSymptomProcedure", some)?;
        }
        if let Some(some) = self.r#disease_status.as_ref() {
            state.serialize_entry("diseaseStatus", some)?;
        }
        if !self.r#comorbidity.is_empty() {
            state.serialize_entry("comorbidity", &self.r#comorbidity)?;
        }
        if let Some(some) = self.r#intended_effect.as_ref() {
            state.serialize_entry("intendedEffect", some)?;
        }
        if let Some(some) = self.r#duration.as_ref() {
            state.serialize_entry("duration", some)?;
        }
        if !self.r#other_therapy.is_empty() {
            state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
        }
        if !self.r#undesirable_effect.is_empty() {
            state.serialize_entry("undesirableEffect", &self.r#undesirable_effect)?;
        }
        if !self.r#population.is_empty() {
            state.serialize_entry("population", &self.r#population)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductIndication {
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
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "diseaseSymptomProcedure")]
            DiseaseSymptomProcedure,
            #[serde(rename = "diseaseStatus")]
            DiseaseStatus,
            #[serde(rename = "comorbidity")]
            Comorbidity,
            #[serde(rename = "intendedEffect")]
            IntendedEffect,
            #[serde(rename = "duration")]
            Duration,
            #[serde(rename = "otherTherapy")]
            OtherTherapy,
            #[serde(rename = "undesirableEffect")]
            UndesirableEffect,
            #[serde(rename = "population")]
            Population,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductIndication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIndication")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicinalProductIndication, V::Error>
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
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#disease_symptom_procedure: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#disease_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#comorbidity: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#intended_effect: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#duration: Option<Box<super::super::types::Quantity>> = None;
                let mut r#other_therapy: Option<Vec<MedicinalProductIndicationOtherTherapy>> = None;
                let mut r#undesirable_effect: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#population: Option<Vec<Box<super::super::types::Population>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProductIndication" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProductIndication",
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
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        Field::DiseaseSymptomProcedure => {
                            if r#disease_symptom_procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "diseaseSymptomProcedure",
                                ));
                            }
                            r#disease_symptom_procedure = Some(map_access.next_value()?);
                        }
                        Field::DiseaseStatus => {
                            if r#disease_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("diseaseStatus"));
                            }
                            r#disease_status = Some(map_access.next_value()?);
                        }
                        Field::Comorbidity => {
                            if r#comorbidity.is_some() {
                                return Err(serde::de::Error::duplicate_field("comorbidity"));
                            }
                            r#comorbidity = Some(map_access.next_value()?);
                        }
                        Field::IntendedEffect => {
                            if r#intended_effect.is_some() {
                                return Err(serde::de::Error::duplicate_field("intendedEffect"));
                            }
                            r#intended_effect = Some(map_access.next_value()?);
                        }
                        Field::Duration => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            r#duration = Some(map_access.next_value()?);
                        }
                        Field::OtherTherapy => {
                            if r#other_therapy.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherTherapy"));
                            }
                            r#other_therapy = Some(map_access.next_value()?);
                        }
                        Field::UndesirableEffect => {
                            if r#undesirable_effect.is_some() {
                                return Err(serde::de::Error::duplicate_field("undesirableEffect"));
                            }
                            r#undesirable_effect = Some(map_access.next_value()?);
                        }
                        Field::Population => {
                            if r#population.is_some() {
                                return Err(serde::de::Error::duplicate_field("population"));
                            }
                            r#population = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductIndication {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#subject: r#subject.unwrap_or(vec![]),
                    r#disease_symptom_procedure,
                    r#disease_status,
                    r#comorbidity: r#comorbidity.unwrap_or(vec![]),
                    r#intended_effect,
                    r#duration,
                    r#other_therapy: r#other_therapy.unwrap_or(vec![]),
                    r#undesirable_effect: r#undesirable_effect.unwrap_or(vec![]),
                    r#population: r#population.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
