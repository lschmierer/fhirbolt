// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Timing or duration information, that may be associated with use with the indicated condition e.g. Adult patients suffering from myocardial infarction (from a few days until less than 35 days), ischaemic stroke (from 7 days until less than 6 months)."]
#[derive(Debug, Clone)]
pub enum ClinicalUseDefinitionIndicationDuration {
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ClinicalUseDefinitionIndicationDuration {
    fn default() -> ClinicalUseDefinitionIndicationDuration {
        ClinicalUseDefinitionIndicationDuration::Invalid
    }
}
#[doc = "The specific medication, food or laboratory test that interacts."]
#[derive(Debug, Clone)]
pub enum ClinicalUseDefinitionInteractionInteractantItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for ClinicalUseDefinitionInteractionInteractantItem {
    fn default() -> ClinicalUseDefinitionInteractionInteractantItem {
        ClinicalUseDefinitionInteractionInteractantItem::Invalid
    }
}
#[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the contraindication."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinitionContraindicationOtherTherapy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship between the medicinal product indication or contraindication and another therapy."]
    pub r#relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
    pub r#therapy: Box<super::super::types::CodeableReference>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionContraindicationOtherTherapy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            state.serialize_entry("relationshipType", &self.r#relationship_type)?;
            state.serialize_entry("therapy", &self.r#therapy)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinitionContraindicationOtherTherapy {
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
            #[serde(rename = "relationshipType")]
            RelationshipType,
            #[serde(rename = "therapy")]
            Therapy,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinitionContraindicationOtherTherapy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinitionContraindicationOtherTherapy")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalUseDefinitionContraindicationOtherTherapy, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#relationship_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#therapy: Option<Box<super::super::types::CodeableReference>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::RelationshipType => {
                                if r#relationship_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relationshipType",
                                    ));
                                }
                                r#relationship_type = Some(map_access.next_value()?);
                            }
                            Field::Therapy => {
                                if r#therapy.is_some() {
                                    return Err(serde::de::Error::duplicate_field("therapy"));
                                }
                                r#therapy = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "relationshipType",
                                        "therapy",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalUseDefinitionContraindicationOtherTherapy {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#relationship_type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#relationship_type.unwrap_or(Default::default())
                        } else {
                            r#relationship_type
                                .ok_or(serde::de::Error::missing_field("relationshipType"))?
                        },
                        r#therapy: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#therapy.unwrap_or(Default::default())
                        } else {
                            r#therapy.ok_or(serde::de::Error::missing_field("therapy"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specifics for when this is a contraindication."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinitionContraindication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation that is being documented as contraindicating against this item."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The status of the disease or symptom for the contraindication, for example \"chronic\" or \"metastatic\"."]
    pub r#disease_status: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "A comorbidity (concurrent condition) or coinfection."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The indication which this is a contraidication for."]
    pub r#indication: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the contraindication."]
    pub r#other_therapy: Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionContraindication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#disease_symptom_procedure.as_ref() {
                state.serialize_entry("diseaseSymptomProcedure", some)?;
            }
            if let Some(some) = self.r#disease_status.as_ref() {
                state.serialize_entry("diseaseStatus", some)?;
            }
            if !self.r#comorbidity.is_empty() {
                state.serialize_entry("comorbidity", &self.r#comorbidity)?;
            }
            if !self.r#indication.is_empty() {
                state.serialize_entry("indication", &self.r#indication)?;
            }
            if !self.r#other_therapy.is_empty() {
                state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinitionContraindication {
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
            #[serde(rename = "diseaseSymptomProcedure")]
            DiseaseSymptomProcedure,
            #[serde(rename = "diseaseStatus")]
            DiseaseStatus,
            #[serde(rename = "comorbidity")]
            Comorbidity,
            #[serde(rename = "indication")]
            Indication,
            #[serde(rename = "otherTherapy")]
            OtherTherapy,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinitionContraindication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinitionContraindication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalUseDefinitionContraindication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#disease_symptom_procedure: Option<
                    Box<super::super::types::CodeableReference>,
                > = None;
                let mut r#disease_status: Option<Box<super::super::types::CodeableReference>> =
                    None;
                let mut r#comorbidity: Option<Vec<Box<super::super::types::CodeableReference>>> =
                    None;
                let mut r#indication: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#other_therapy: Option<
                    Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
                > = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Indication => {
                                if r#indication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                r#indication = Some(map_access.next_value()?);
                            }
                            Field::OtherTherapy => {
                                if r#other_therapy.is_some() {
                                    return Err(serde::de::Error::duplicate_field("otherTherapy"));
                                }
                                r#other_therapy = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "diseaseSymptomProcedure",
                                        "diseaseStatus",
                                        "comorbidity",
                                        "indication",
                                        "otherTherapy",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalUseDefinitionContraindication {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#disease_symptom_procedure,
                        r#disease_status,
                        r#comorbidity: r#comorbidity.unwrap_or(vec![]),
                        r#indication: r#indication.unwrap_or(vec![]),
                        r#other_therapy: r#other_therapy.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specifics for when this is an indication."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinitionIndication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation that is being documented as an indicaton for this item."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The status of the disease or symptom for the indication, for example \"chronic\" or \"metastatic\"."]
    pub r#disease_status: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "A comorbidity (concurrent condition) or coinfection as part of the indication."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The intended effect, aim or strategy to be achieved."]
    pub r#intended_effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Timing or duration information, that may be associated with use with the indicated condition e.g. Adult patients suffering from myocardial infarction (from a few days until less than 35 days), ischaemic stroke (from 7 days until less than 6 months)."]
    pub r#duration: Option<ClinicalUseDefinitionIndicationDuration>,
    #[doc = "An unwanted side effect or negative outcome that may happen if you use the drug (or other subject of this resource) for this indication."]
    pub r#undesirable_effect: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
    pub r#other_therapy: Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionIndication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
                match some {
                    ClinicalUseDefinitionIndicationDuration::Range(ref value) => {
                        state.serialize_entry("durationRange", value)?;
                    }
                    ClinicalUseDefinitionIndicationDuration::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("durationString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_durationString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("durationString", value)?;
                        }
                    }
                    ClinicalUseDefinitionIndicationDuration::Invalid => {
                        return Err(serde::ser::Error::custom("duration is invalid"))
                    }
                }
            }
            if !self.r#undesirable_effect.is_empty() {
                state.serialize_entry("undesirableEffect", &self.r#undesirable_effect)?;
            }
            if !self.r#other_therapy.is_empty() {
                state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinitionIndication {
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
            #[serde(rename = "diseaseSymptomProcedure")]
            DiseaseSymptomProcedure,
            #[serde(rename = "diseaseStatus")]
            DiseaseStatus,
            #[serde(rename = "comorbidity")]
            Comorbidity,
            #[serde(rename = "intendedEffect")]
            IntendedEffect,
            #[serde(rename = "durationRange")]
            DurationRange,
            #[serde(rename = "durationString")]
            DurationString,
            #[serde(rename = "_durationString")]
            DurationStringPrimitiveElement,
            #[serde(rename = "undesirableEffect")]
            UndesirableEffect,
            #[serde(rename = "otherTherapy")]
            OtherTherapy,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinitionIndication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinitionIndication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalUseDefinitionIndication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#disease_symptom_procedure: Option<
                    Box<super::super::types::CodeableReference>,
                > = None;
                let mut r#disease_status: Option<Box<super::super::types::CodeableReference>> =
                    None;
                let mut r#comorbidity: Option<Vec<Box<super::super::types::CodeableReference>>> =
                    None;
                let mut r#intended_effect: Option<Box<super::super::types::CodeableReference>> =
                    None;
                let mut r#duration: Option<ClinicalUseDefinitionIndicationDuration> = None;
                let mut r#undesirable_effect: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#other_therapy: Option<
                    Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
                > = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "intendedEffect",
                                    ));
                                }
                                r#intended_effect = Some(map_access.next_value()?);
                            }
                            Field::DurationRange => {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("durationRange"));
                                }
                                r#duration = Some(ClinicalUseDefinitionIndicationDuration::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DurationString => {
                                if _ctx.from_json {
                                    let r#enum = r#duration.get_or_insert(
                                        ClinicalUseDefinitionIndicationDuration::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let ClinicalUseDefinitionIndicationDuration::String(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "durationString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "duration[x]",
                                        ));
                                    }
                                } else {
                                    if r#duration.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "durationString",
                                        ));
                                    }
                                    r#duration =
                                        Some(ClinicalUseDefinitionIndicationDuration::String(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::DurationStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#duration.get_or_insert(
                                        ClinicalUseDefinitionIndicationDuration::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let ClinicalUseDefinitionIndicationDuration::String(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_durationString",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_duration[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "durationString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "diseaseSymptomProcedure",
                                            "diseaseStatus",
                                            "comorbidity",
                                            "intendedEffect",
                                            "durationRange",
                                            "durationString",
                                            "undesirableEffect",
                                            "otherTherapy",
                                        ],
                                    ));
                                }
                            }
                            Field::UndesirableEffect => {
                                if r#undesirable_effect.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "undesirableEffect",
                                    ));
                                }
                                r#undesirable_effect = Some(map_access.next_value()?);
                            }
                            Field::OtherTherapy => {
                                if r#other_therapy.is_some() {
                                    return Err(serde::de::Error::duplicate_field("otherTherapy"));
                                }
                                r#other_therapy = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "diseaseSymptomProcedure",
                                        "diseaseStatus",
                                        "comorbidity",
                                        "intendedEffect",
                                        "durationRange",
                                        "durationString",
                                        "undesirableEffect",
                                        "otherTherapy",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalUseDefinitionIndication {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#disease_symptom_procedure,
                        r#disease_status,
                        r#comorbidity: r#comorbidity.unwrap_or(vec![]),
                        r#intended_effect,
                        r#duration,
                        r#undesirable_effect: r#undesirable_effect.unwrap_or(vec![]),
                        r#other_therapy: r#other_therapy.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The specific medication, food, substance or laboratory test that interacts."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinitionInteractionInteractant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific medication, food or laboratory test that interacts."]
    pub r#item: ClinicalUseDefinitionInteractionInteractantItem,
}
impl serde::ser::Serialize for ClinicalUseDefinitionInteractionInteractant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
                ClinicalUseDefinitionInteractionInteractantItem::Reference(ref value) => {
                    state.serialize_entry("itemReference", value)?;
                }
                ClinicalUseDefinitionInteractionInteractantItem::CodeableConcept(ref value) => {
                    state.serialize_entry("itemCodeableConcept", value)?;
                }
                ClinicalUseDefinitionInteractionInteractantItem::Invalid => {
                    return Err(serde::ser::Error::custom("item is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinitionInteractionInteractant {
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
            #[serde(rename = "itemReference")]
            ItemReference,
            #[serde(rename = "itemCodeableConcept")]
            ItemCodeableConcept,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinitionInteractionInteractant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinitionInteractionInteractant")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalUseDefinitionInteractionInteractant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item: Option<ClinicalUseDefinitionInteractionInteractantItem> = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: ItemReference => { if r#item . is_some () { return Err (serde :: de :: Error :: duplicate_field ("itemReference")) ; } r#item = Some (ClinicalUseDefinitionInteractionInteractantItem :: Reference (map_access . next_value () ?)) ; } , Field :: ItemCodeableConcept => { if r#item . is_some () { return Err (serde :: de :: Error :: duplicate_field ("itemCodeableConcept")) ; } r#item = Some (ClinicalUseDefinitionInteractionInteractantItem :: CodeableConcept (map_access . next_value () ?)) ; } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "itemReference" , "itemCodeableConcept" ,])) ; } } } Ok (ClinicalUseDefinitionInteractionInteractant { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#item : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#item . unwrap_or (Default :: default ()) } else { r#item . ok_or (serde :: de :: Error :: missing_field ("item[x]")) ? } , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specifics for when this is an interaction."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinitionInteraction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific medication, food, substance or laboratory test that interacts."]
    pub r#interactant: Vec<ClinicalUseDefinitionInteractionInteractant>,
    #[doc = "The type of the interaction e.g. drug-drug interaction, drug-food interaction, drug-lab test interaction."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The effect of the interaction, for example \"reduced gastric absorption of primary medication\"."]
    pub r#effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The incidence of the interaction, e.g. theoretical, observed."]
    pub r#incidence: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Actions for managing the interaction."]
    pub r#management: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionInteraction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if !self.r#interactant.is_empty() {
                state.serialize_entry("interactant", &self.r#interactant)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#effect.as_ref() {
                state.serialize_entry("effect", some)?;
            }
            if let Some(some) = self.r#incidence.as_ref() {
                state.serialize_entry("incidence", some)?;
            }
            if !self.r#management.is_empty() {
                state.serialize_entry("management", &self.r#management)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinitionInteraction {
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
            #[serde(rename = "interactant")]
            Interactant,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "effect")]
            Effect,
            #[serde(rename = "incidence")]
            Incidence,
            #[serde(rename = "management")]
            Management,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinitionInteraction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinitionInteraction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalUseDefinitionInteraction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#interactant: Option<Vec<ClinicalUseDefinitionInteractionInteractant>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#effect: Option<Box<super::super::types::CodeableReference>> = None;
                let mut r#incidence: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#management: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Interactant => {
                                if r#interactant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("interactant"));
                                }
                                r#interactant = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Effect => {
                                if r#effect.is_some() {
                                    return Err(serde::de::Error::duplicate_field("effect"));
                                }
                                r#effect = Some(map_access.next_value()?);
                            }
                            Field::Incidence => {
                                if r#incidence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("incidence"));
                                }
                                r#incidence = Some(map_access.next_value()?);
                            }
                            Field::Management => {
                                if r#management.is_some() {
                                    return Err(serde::de::Error::duplicate_field("management"));
                                }
                                r#management = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "interactant",
                                        "type",
                                        "effect",
                                        "incidence",
                                        "management",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalUseDefinitionInteraction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#interactant: r#interactant.unwrap_or(vec![]),
                        r#type,
                        r#effect,
                        r#incidence,
                        r#management: r#management.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Describe the possible undesirable effects (negative outcomes) from the use of the medicinal product as treatment."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinitionUndesirableEffect {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation in which the undesirable effect may manifest."]
    pub r#symptom_condition_effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "High level classification of the effect."]
    pub r#classification: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How often the effect is seen."]
    pub r#frequency_of_occurrence: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionUndesirableEffect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#symptom_condition_effect.as_ref() {
                state.serialize_entry("symptomConditionEffect", some)?;
            }
            if let Some(some) = self.r#classification.as_ref() {
                state.serialize_entry("classification", some)?;
            }
            if let Some(some) = self.r#frequency_of_occurrence.as_ref() {
                state.serialize_entry("frequencyOfOccurrence", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinitionUndesirableEffect {
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
            #[serde(rename = "symptomConditionEffect")]
            SymptomConditionEffect,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "frequencyOfOccurrence")]
            FrequencyOfOccurrence,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinitionUndesirableEffect;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinitionUndesirableEffect")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalUseDefinitionUndesirableEffect, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#symptom_condition_effect: Option<
                    Box<super::super::types::CodeableReference>,
                > = None;
                let mut r#classification: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#frequency_of_occurrence: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::SymptomConditionEffect => {
                                if r#symptom_condition_effect.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "symptomConditionEffect",
                                    ));
                                }
                                r#symptom_condition_effect = Some(map_access.next_value()?);
                            }
                            Field::Classification => {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some(map_access.next_value()?);
                            }
                            Field::FrequencyOfOccurrence => {
                                if r#frequency_of_occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "frequencyOfOccurrence",
                                    ));
                                }
                                r#frequency_of_occurrence = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "symptomConditionEffect",
                                        "classification",
                                        "frequencyOfOccurrence",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalUseDefinitionUndesirableEffect {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#symptom_condition_effect,
                        r#classification,
                        r#frequency_of_occurrence,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A critical piece of information about environmental, health or physical risks or hazards that serve as caution to the user. For example 'Do not operate heavy machinery', 'May cause drowsiness', or 'Get medical advice/attention if you feel unwell'."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinitionWarning {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A textual definition of this warning, with formatting."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A coded or unformatted textual definition of this warning."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionWarning {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinitionWarning {
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
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinitionWarning;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinitionWarning")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalUseDefinitionWarning, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "description",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "code",
                                        ],
                                    ));
                                }
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "description",
                                        "code",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalUseDefinitionWarning {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#code,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure."]
#[derive(Default, Debug, Clone)]
pub struct ClinicalUseDefinition {
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
    #[doc = "Business identifier for this issue."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "indication | contraindication | interaction | undesirable-effect | warning."]
    pub r#type: super::super::types::Code,
    #[doc = "A categorisation of the issue, primarily for dividing warnings into subject heading areas such as \"Pregnancy and Lactation\", \"Overdose\", \"Effects on Ability to Drive and Use Machines\"."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The medication or procedure for which this is an indication."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "Whether this is a current issue or one that has been retired etc."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifics for when this is a contraindication."]
    pub r#contraindication: Option<ClinicalUseDefinitionContraindication>,
    #[doc = "Specifics for when this is an indication."]
    pub r#indication: Option<ClinicalUseDefinitionIndication>,
    #[doc = "Specifics for when this is an interaction."]
    pub r#interaction: Option<ClinicalUseDefinitionInteraction>,
    #[doc = "The population group to which this applies."]
    pub r#population: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describe the possible undesirable effects (negative outcomes) from the use of the medicinal product as treatment."]
    pub r#undesirable_effect: Option<ClinicalUseDefinitionUndesirableEffect>,
    #[doc = "A critical piece of information about environmental, health or physical risks or hazards that serve as caution to the user. For example 'Do not operate heavy machinery', 'May cause drowsiness', or 'Get medical advice/attention if you feel unwell'."]
    pub r#warning: Option<ClinicalUseDefinitionWarning>,
}
impl crate::AnyResource for ClinicalUseDefinition {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for ClinicalUseDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ClinicalUseDefinition")?;
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if let Some(some) = self.r#contraindication.as_ref() {
                state.serialize_entry("contraindication", some)?;
            }
            if let Some(some) = self.r#indication.as_ref() {
                state.serialize_entry("indication", some)?;
            }
            if let Some(some) = self.r#interaction.as_ref() {
                state.serialize_entry("interaction", some)?;
            }
            if !self.r#population.is_empty() {
                state.serialize_entry("population", &self.r#population)?;
            }
            if let Some(some) = self.r#undesirable_effect.as_ref() {
                state.serialize_entry("undesirableEffect", some)?;
            }
            if let Some(some) = self.r#warning.as_ref() {
                state.serialize_entry("warning", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalUseDefinition {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "contraindication")]
            Contraindication,
            #[serde(rename = "indication")]
            Indication,
            #[serde(rename = "interaction")]
            Interaction,
            #[serde(rename = "population")]
            Population,
            #[serde(rename = "undesirableEffect")]
            UndesirableEffect,
            #[serde(rename = "warning")]
            Warning,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalUseDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalUseDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClinicalUseDefinition, V::Error>
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
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#contraindication: Option<ClinicalUseDefinitionContraindication> = None;
                let mut r#indication: Option<ClinicalUseDefinitionIndication> = None;
                let mut r#interaction: Option<ClinicalUseDefinitionInteraction> = None;
                let mut r#population: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#undesirable_effect: Option<ClinicalUseDefinitionUndesirableEffect> = None;
                let mut r#warning: Option<ClinicalUseDefinitionWarning> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ClinicalUseDefinition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ClinicalUseDefinition",
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
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implicitRules",
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
                                            "type",
                                            "category",
                                            "subject",
                                            "status",
                                            "contraindication",
                                            "indication",
                                            "interaction",
                                            "population",
                                            "undesirableEffect",
                                            "warning",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
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
                                            "type",
                                            "category",
                                            "subject",
                                            "status",
                                            "contraindication",
                                            "indication",
                                            "interaction",
                                            "population",
                                            "undesirableEffect",
                                            "warning",
                                        ],
                                    ));
                                }
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
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_type"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
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
                                            "type",
                                            "category",
                                            "subject",
                                            "status",
                                            "contraindication",
                                            "indication",
                                            "interaction",
                                            "population",
                                            "undesirableEffect",
                                            "warning",
                                        ],
                                    ));
                                }
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::Contraindication => {
                                if r#contraindication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contraindication",
                                    ));
                                }
                                r#contraindication = Some(map_access.next_value()?);
                            }
                            Field::Indication => {
                                if r#indication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                r#indication = Some(map_access.next_value()?);
                            }
                            Field::Interaction => {
                                if r#interaction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("interaction"));
                                }
                                r#interaction = Some(map_access.next_value()?);
                            }
                            Field::Population => {
                                if r#population.is_some() {
                                    return Err(serde::de::Error::duplicate_field("population"));
                                }
                                r#population = Some(map_access.next_value()?);
                            }
                            Field::UndesirableEffect => {
                                if r#undesirable_effect.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "undesirableEffect",
                                    ));
                                }
                                r#undesirable_effect = Some(map_access.next_value()?);
                            }
                            Field::Warning => {
                                if r#warning.is_some() {
                                    return Err(serde::de::Error::duplicate_field("warning"));
                                }
                                r#warning = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
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
                                        "type",
                                        "category",
                                        "subject",
                                        "status",
                                        "contraindication",
                                        "indication",
                                        "interaction",
                                        "population",
                                        "undesirableEffect",
                                        "warning",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalUseDefinition {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#subject: r#subject.unwrap_or(vec![]),
                        r#status,
                        r#contraindication,
                        r#indication,
                        r#interaction,
                        r#population: r#population.unwrap_or(vec![]),
                        r#undesirable_effect,
                        r#warning,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
