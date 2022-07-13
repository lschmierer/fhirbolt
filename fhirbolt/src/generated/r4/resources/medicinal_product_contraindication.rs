// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductContraindicationOtherTherapyMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for MedicinalProductContraindicationOtherTherapyMedication {
    fn default() -> MedicinalProductContraindicationOtherTherapyMedication {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductContraindicationOtherTherapy {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#therapy_relationship_type: Box<super::super::types::CodeableConcept>,
    pub r#medication: MedicinalProductContraindicationOtherTherapyMedication,
}
impl serde::ser::Serialize for MedicinalProductContraindicationOtherTherapy {
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
            MedicinalProductContraindicationOtherTherapyMedication::CodeableConcept(ref value) => {
                state.serialize_entry("medicationCodeableConcept", value)?;
            }
            MedicinalProductContraindicationOtherTherapyMedication::Reference(ref value) => {
                state.serialize_entry("medicationReference", value)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductContraindicationOtherTherapy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductContraindicationOtherTherapy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductContraindicationOtherTherapy")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductContraindicationOtherTherapy, V::Error>
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
                let mut r#medication: Option<
                    MedicinalProductContraindicationOtherTherapyMedication,
                > = None;
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
                        "therapyRelationshipType" => {
                            if r#therapy_relationship_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "therapyRelationshipType",
                                ));
                            }
                            r#therapy_relationship_type = Some(map_access.next_value()?);
                        }
                        "medicationCodeableConcept" => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationCodeableConcept",
                                ));
                            }
                            r#medication = Some (MedicinalProductContraindicationOtherTherapyMedication :: CodeableConcept (map_access . next_value () ?)) ;
                        }
                        "medicationReference" => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationReference",
                                ));
                            }
                            r#medication = Some(
                                MedicinalProductContraindicationOtherTherapyMedication::Reference(
                                    map_access.next_value()?,
                                ),
                            );
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "therapy_relationship_type",
                                    "medication",
                                ],
                            ))
                        }
                    }
                }
                Ok(MedicinalProductContraindicationOtherTherapy {
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
pub struct MedicinalProductContraindication {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#disease: Option<Box<super::super::types::CodeableConcept>>,
    pub r#disease_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#comorbidity: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#therapeutic_indication: Vec<Box<super::super::types::Reference>>,
    pub r#other_therapy: Vec<MedicinalProductContraindicationOtherTherapy>,
    pub r#population: Vec<Box<super::super::types::Population>>,
}
impl serde::ser::Serialize for MedicinalProductContraindication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductContraindication")?;
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
        if let Some(some) = self.r#disease.as_ref() {
            state.serialize_entry("disease", some)?;
        }
        if let Some(some) = self.r#disease_status.as_ref() {
            state.serialize_entry("diseaseStatus", some)?;
        }
        if !self.r#comorbidity.is_empty() {
            state.serialize_entry("comorbidity", &self.r#comorbidity)?;
        }
        if !self.r#therapeutic_indication.is_empty() {
            state.serialize_entry("therapeuticIndication", &self.r#therapeutic_indication)?;
        }
        if !self.r#other_therapy.is_empty() {
            state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
        }
        if !self.r#population.is_empty() {
            state.serialize_entry("population", &self.r#population)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductContraindication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductContraindication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductContraindication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductContraindication, V::Error>
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
                let mut r#disease: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#disease_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#comorbidity: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#therapeutic_indication: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#other_therapy: Option<Vec<MedicinalProductContraindicationOtherTherapy>> =
                    None;
                let mut r#population: Option<Vec<Box<super::super::types::Population>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
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
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "disease" => {
                            if r#disease.is_some() {
                                return Err(serde::de::Error::duplicate_field("disease"));
                            }
                            r#disease = Some(map_access.next_value()?);
                        }
                        "diseaseStatus" => {
                            if r#disease_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("diseaseStatus"));
                            }
                            r#disease_status = Some(map_access.next_value()?);
                        }
                        "comorbidity" => {
                            if r#comorbidity.is_some() {
                                return Err(serde::de::Error::duplicate_field("comorbidity"));
                            }
                            r#comorbidity = Some(map_access.next_value()?);
                        }
                        "therapeuticIndication" => {
                            if r#therapeutic_indication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "therapeuticIndication",
                                ));
                            }
                            r#therapeutic_indication = Some(map_access.next_value()?);
                        }
                        "otherTherapy" => {
                            if r#other_therapy.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherTherapy"));
                            }
                            r#other_therapy = Some(map_access.next_value()?);
                        }
                        "population" => {
                            if r#population.is_some() {
                                return Err(serde::de::Error::duplicate_field("population"));
                            }
                            r#population = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "subject",
                                    "disease",
                                    "disease_status",
                                    "comorbidity",
                                    "therapeutic_indication",
                                    "other_therapy",
                                    "population",
                                ],
                            ))
                        }
                    }
                }
                Ok(MedicinalProductContraindication {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#subject: r#subject.unwrap_or(vec![]),
                    r#disease,
                    r#disease_status,
                    r#comorbidity: r#comorbidity.unwrap_or(vec![]),
                    r#therapeutic_indication: r#therapeutic_indication.unwrap_or(vec![]),
                    r#other_therapy: r#other_therapy.unwrap_or(vec![]),
                    r#population: r#population.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
