// Generated on 2022-10-14 by fhirbolt-codegen v0.1.0
#[doc = "The status history permits the encounter resource to contain the status history without needing to read through the historical versions of the resource, or even have the server store them."]
#[derive(Default, Debug, Clone)]
pub struct EncounterStatusHistory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "planned | arrived | triaged | in-progress | onleave | finished | cancelled +."]
    pub r#status: super::super::types::Code,
    #[doc = "The time that the episode was in the specified status."]
    pub r#period: Box<super::super::types::Period>,
}
impl crate::AnyResource for EncounterStatusHistory {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for EncounterStatusHistory {
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
        if let Some(some) = self.r#status.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        state.serialize_entry("period", &self.r#period)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EncounterStatusHistory {
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EncounterStatusHistory;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterStatusHistory")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterStatusHistory, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusPrimitiveElement => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "status", "period"],
                                ));
                            },
                        }
                    }
                    Ok(EncounterStatusHistory {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#period: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#period.unwrap_or(Default::default())
                        } else {
                            r#period.ok_or(serde::de::Error::missing_field("period"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The class history permits the tracking of the encounters transitions without needing to go  through the resource history.  This would be used for a case where an admission starts of as an emergency encounter, then transitions into an inpatient scenario. Doing this and not restarting a new encounter ensures that any lab/diagnostic results can more easily follow the patient and not require re-processing and not get lost or cancelled during a kind of discharge from emergency to inpatient."]
#[derive(Default, Debug, Clone)]
pub struct EncounterClassHistory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "inpatient | outpatient | ambulatory | emergency +."]
    pub r#class: Box<super::super::types::Coding>,
    #[doc = "The time that the episode was in the specified class."]
    pub r#period: Box<super::super::types::Period>,
}
impl serde::ser::Serialize for EncounterClassHistory {
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
        state.serialize_entry("class", &self.r#class)?;
        state.serialize_entry("period", &self.r#period)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EncounterClassHistory {
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
            #[serde(rename = "class")]
            Class,
            #[serde(rename = "period")]
            Period,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EncounterClassHistory;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterClassHistory")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterClassHistory, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#class: Option<Box<super::super::types::Coding>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Class => {
                                if r#class.is_some() {
                                    return Err(serde::de::Error::duplicate_field("class"));
                                }
                                r#class = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "class", "period"],
                                ));
                            },
                        }
                    }
                    Ok(EncounterClassHistory {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#class: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#class.unwrap_or(Default::default())
                        } else {
                            r#class.ok_or(serde::de::Error::missing_field("class"))?
                        },
                        r#period: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#period.unwrap_or(Default::default())
                        } else {
                            r#period.ok_or(serde::de::Error::missing_field("period"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The list of people responsible for providing the service."]
#[derive(Default, Debug, Clone)]
pub struct EncounterParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role of participant in encounter."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The period of time that the specified participant participated in the encounter. These can overlap or be sub-sets of the overall encounter's period."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Persons involved in the encounter other than the patient."]
    pub r#individual: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for EncounterParticipant {
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
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if let Some(some) = self.r#individual.as_ref() {
            state.serialize_entry("individual", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EncounterParticipant {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "individual")]
            Individual,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EncounterParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterParticipant")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterParticipant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#individual: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Individual => {
                                if r#individual.is_some() {
                                    return Err(serde::de::Error::duplicate_field("individual"));
                                }
                                r#individual = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "type",
                                        "period",
                                        "individual",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EncounterParticipant {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: r#type.unwrap_or(vec![]),
                        r#period,
                        r#individual,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The list of diagnosis relevant to this encounter."]
#[derive(Default, Debug, Clone)]
pub struct EncounterDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reason the encounter takes place, as specified using information from another resource. For admissions, this is the admission diagnosis. The indication will typically be a Condition (with other resources referenced in the evidence.detail), or a Procedure."]
    pub r#condition: Box<super::super::types::Reference>,
    #[doc = "Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)."]
    pub r#use: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Ranking of the diagnosis (for each role type)."]
    pub r#rank: Option<super::super::types::PositiveInt>,
}
impl serde::ser::Serialize for EncounterDiagnosis {
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
        state.serialize_entry("condition", &self.r#condition)?;
        if let Some(some) = self.r#use.as_ref() {
            state.serialize_entry("use", some)?;
        }
        if let Some(some) = self.r#rank.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("rank", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_rank", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EncounterDiagnosis {
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
            #[serde(rename = "condition")]
            Condition,
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "rank")]
            Rank,
            #[serde(rename = "_rank")]
            RankPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EncounterDiagnosis;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterDiagnosis")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterDiagnosis, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#condition: Option<Box<super::super::types::Reference>> = None;
                let mut r#use: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#rank: Option<super::super::types::PositiveInt> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Condition => {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some(map_access.next_value()?);
                            }
                            Field::Use => {
                                if r#use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("use"));
                                }
                                r#use = Some(map_access.next_value()?);
                            }
                            Field::Rank => {
                                let some = r#rank.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rank"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RankPrimitiveElement => {
                                let some = r#rank.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_rank"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "condition",
                                        "use",
                                        "rank",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EncounterDiagnosis {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#condition: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#condition.unwrap_or(Default::default())
                        } else {
                            r#condition.ok_or(serde::de::Error::missing_field("condition"))?
                        },
                        r#use,
                        r#rank,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Details about the admission to a healthcare service."]
#[derive(Default, Debug, Clone)]
pub struct EncounterHospitalization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Pre-admission identifier."]
    pub r#pre_admission_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The location/organization from which the patient came before admission."]
    pub r#origin: Option<Box<super::super::types::Reference>>,
    #[doc = "From where patient was admitted (physician referral, transfer)."]
    pub r#admit_source: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether this hospitalization is a readmission and why if known."]
    pub r#re_admission: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Diet preferences reported by the patient."]
    pub r#diet_preference: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Special courtesies (VIP, board member)."]
    pub r#special_courtesy: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Any special requests that have been made for this hospitalization encounter, such as the provision of specific equipment or other things."]
    pub r#special_arrangement: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Location/organization to which the patient is discharged."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Category or kind of location after discharge."]
    pub r#discharge_disposition: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for EncounterHospitalization {
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
        if let Some(some) = self.r#pre_admission_identifier.as_ref() {
            state.serialize_entry("preAdmissionIdentifier", some)?;
        }
        if let Some(some) = self.r#origin.as_ref() {
            state.serialize_entry("origin", some)?;
        }
        if let Some(some) = self.r#admit_source.as_ref() {
            state.serialize_entry("admitSource", some)?;
        }
        if let Some(some) = self.r#re_admission.as_ref() {
            state.serialize_entry("reAdmission", some)?;
        }
        if !self.r#diet_preference.is_empty() {
            state.serialize_entry("dietPreference", &self.r#diet_preference)?;
        }
        if !self.r#special_courtesy.is_empty() {
            state.serialize_entry("specialCourtesy", &self.r#special_courtesy)?;
        }
        if !self.r#special_arrangement.is_empty() {
            state.serialize_entry("specialArrangement", &self.r#special_arrangement)?;
        }
        if let Some(some) = self.r#destination.as_ref() {
            state.serialize_entry("destination", some)?;
        }
        if let Some(some) = self.r#discharge_disposition.as_ref() {
            state.serialize_entry("dischargeDisposition", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EncounterHospitalization {
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
            #[serde(rename = "preAdmissionIdentifier")]
            PreAdmissionIdentifier,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "admitSource")]
            AdmitSource,
            #[serde(rename = "reAdmission")]
            ReAdmission,
            #[serde(rename = "dietPreference")]
            DietPreference,
            #[serde(rename = "specialCourtesy")]
            SpecialCourtesy,
            #[serde(rename = "specialArrangement")]
            SpecialArrangement,
            #[serde(rename = "destination")]
            Destination,
            #[serde(rename = "dischargeDisposition")]
            DischargeDisposition,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EncounterHospitalization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterHospitalization")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterHospitalization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#pre_admission_identifier: Option<Box<super::super::types::Identifier>> =
                    None;
                let mut r#origin: Option<Box<super::super::types::Reference>> = None;
                let mut r#admit_source: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#re_admission: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#diet_preference: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#special_courtesy: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#special_arrangement: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#destination: Option<Box<super::super::types::Reference>> = None;
                let mut r#discharge_disposition: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::PreAdmissionIdentifier => {
                                if r#pre_admission_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preAdmissionIdentifier",
                                    ));
                                }
                                r#pre_admission_identifier = Some(map_access.next_value()?);
                            }
                            Field::Origin => {
                                if r#origin.is_some() {
                                    return Err(serde::de::Error::duplicate_field("origin"));
                                }
                                r#origin = Some(map_access.next_value()?);
                            }
                            Field::AdmitSource => {
                                if r#admit_source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("admitSource"));
                                }
                                r#admit_source = Some(map_access.next_value()?);
                            }
                            Field::ReAdmission => {
                                if r#re_admission.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reAdmission"));
                                }
                                r#re_admission = Some(map_access.next_value()?);
                            }
                            Field::DietPreference => {
                                if r#diet_preference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dietPreference",
                                    ));
                                }
                                r#diet_preference = Some(map_access.next_value()?);
                            }
                            Field::SpecialCourtesy => {
                                if r#special_courtesy.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialCourtesy",
                                    ));
                                }
                                r#special_courtesy = Some(map_access.next_value()?);
                            }
                            Field::SpecialArrangement => {
                                if r#special_arrangement.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialArrangement",
                                    ));
                                }
                                r#special_arrangement = Some(map_access.next_value()?);
                            }
                            Field::Destination => {
                                if r#destination.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                r#destination = Some(map_access.next_value()?);
                            }
                            Field::DischargeDisposition => {
                                if r#discharge_disposition.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dischargeDisposition",
                                    ));
                                }
                                r#discharge_disposition = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "preAdmissionIdentifier",
                                        "origin",
                                        "admitSource",
                                        "reAdmission",
                                        "dietPreference",
                                        "specialCourtesy",
                                        "specialArrangement",
                                        "destination",
                                        "dischargeDisposition",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EncounterHospitalization {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#pre_admission_identifier,
                        r#origin,
                        r#admit_source,
                        r#re_admission,
                        r#diet_preference: r#diet_preference.unwrap_or(vec![]),
                        r#special_courtesy: r#special_courtesy.unwrap_or(vec![]),
                        r#special_arrangement: r#special_arrangement.unwrap_or(vec![]),
                        r#destination,
                        r#discharge_disposition,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "List of locations where  the patient has been during this encounter."]
#[derive(Default, Debug, Clone)]
pub struct EncounterLocation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The location where the encounter takes place."]
    pub r#location: Box<super::super::types::Reference>,
    #[doc = "The status of the participants' presence at the specified location during the period specified. If the participant is no longer at the location, then the period will have an end date/time."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "This will be used to specify the required levels (bed/ward/room/etc.) desired to be recorded to simplify either messaging or query."]
    pub r#physical_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Time period during which the patient was present at the location."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for EncounterLocation {
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
        state.serialize_entry("location", &self.r#location)?;
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("status", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#physical_type.as_ref() {
            state.serialize_entry("physicalType", some)?;
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EncounterLocation {
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
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "physicalType")]
            PhysicalType,
            #[serde(rename = "period")]
            Period,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EncounterLocation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterLocation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterLocation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#physical_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusPrimitiveElement => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::PhysicalType => {
                                if r#physical_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("physicalType"));
                                }
                                r#physical_type = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "location",
                                        "status",
                                        "physicalType",
                                        "period",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EncounterLocation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#location: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#location.unwrap_or(Default::default())
                        } else {
                            r#location.ok_or(serde::de::Error::missing_field("location"))?
                        },
                        r#status,
                        r#physical_type,
                        r#period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An interaction between a patient and healthcare provider(s) for the purpose of providing healthcare service(s) or assessing the health status of a patient."]
#[derive(Default, Debug, Clone)]
pub struct Encounter {
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
    #[doc = "Identifier(s) by which this encounter is known."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "planned | arrived | triaged | in-progress | onleave | finished | cancelled +."]
    pub r#status: super::super::types::Code,
    #[doc = "The status history permits the encounter resource to contain the status history without needing to read through the historical versions of the resource, or even have the server store them."]
    pub r#status_history: Vec<EncounterStatusHistory>,
    #[doc = "Concepts representing classification of patient encounter such as ambulatory (outpatient), inpatient, emergency, home health or others due to local variations."]
    pub r#class: Box<super::super::types::Coding>,
    #[doc = "The class history permits the tracking of the encounters transitions without needing to go  through the resource history.  This would be used for a case where an admission starts of as an emergency encounter, then transitions into an inpatient scenario. Doing this and not restarting a new encounter ensures that any lab/diagnostic results can more easily follow the patient and not require re-processing and not get lost or cancelled during a kind of discharge from emergency to inpatient."]
    pub r#class_history: Vec<EncounterClassHistory>,
    #[doc = "Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled nursing, rehabilitation)."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Broad categorization of the service that is to be provided (e.g. cardiology)."]
    pub r#service_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the urgency of the encounter."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient or group present at the encounter."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Where a specific encounter should be classified as a part of a specific episode(s) of care this field should be used. This association can facilitate grouping of related encounters together for a specific purpose, such as government reporting, issue tracking, association via a common problem.  The association is recorded on the encounter as these are typically created after the episode of care and grouped on entry rather than editing the episode of care to append another encounter to it (the episode of care could span years)."]
    pub r#episode_of_care: Vec<Box<super::super::types::Reference>>,
    #[doc = "The request this encounter satisfies (e.g. incoming referral or procedure request)."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The list of people responsible for providing the service."]
    pub r#participant: Vec<EncounterParticipant>,
    #[doc = "The appointment that scheduled this encounter."]
    pub r#appointment: Vec<Box<super::super::types::Reference>>,
    #[doc = "The start and end time of the encounter."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Quantity of time the encounter lasted. This excludes the time during leaves of absence."]
    pub r#length: Option<Box<super::super::types::Duration>>,
    #[doc = "Reason the encounter takes place, expressed as a code. For admissions, this can be used for a coded admission diagnosis."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reason the encounter takes place, expressed as a code. For admissions, this can be used for a coded admission diagnosis."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "The list of diagnosis relevant to this encounter."]
    pub r#diagnosis: Vec<EncounterDiagnosis>,
    #[doc = "The set of accounts that may be used for billing for this Encounter."]
    pub r#account: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details about the admission to a healthcare service."]
    pub r#hospitalization: Option<EncounterHospitalization>,
    #[doc = "List of locations where  the patient has been during this encounter."]
    pub r#location: Vec<EncounterLocation>,
    #[doc = "The organization that is primarily responsible for this Encounter's services. This MAY be the same as the organization on the Patient record, however it could be different, such as if the actor performing the services was from an external organization (which may be billed seperately) for an external consultation.  Refer to the example bundle showing an abbreviated set of Encounters for a colonoscopy."]
    pub r#service_provider: Option<Box<super::super::types::Reference>>,
    #[doc = "Another Encounter of which this encounter is a part of (administratively or in time)."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for Encounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Encounter")?;
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
        if let Some(some) = self.r#status.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if !self.r#status_history.is_empty() {
            state.serialize_entry("statusHistory", &self.r#status_history)?;
        }
        state.serialize_entry("class", &self.r#class)?;
        if !self.r#class_history.is_empty() {
            state.serialize_entry("classHistory", &self.r#class_history)?;
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if let Some(some) = self.r#service_type.as_ref() {
            state.serialize_entry("serviceType", some)?;
        }
        if let Some(some) = self.r#priority.as_ref() {
            state.serialize_entry("priority", some)?;
        }
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if !self.r#episode_of_care.is_empty() {
            state.serialize_entry("episodeOfCare", &self.r#episode_of_care)?;
        }
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
        }
        if !self.r#participant.is_empty() {
            state.serialize_entry("participant", &self.r#participant)?;
        }
        if !self.r#appointment.is_empty() {
            state.serialize_entry("appointment", &self.r#appointment)?;
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if let Some(some) = self.r#length.as_ref() {
            state.serialize_entry("length", some)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if !self.r#diagnosis.is_empty() {
            state.serialize_entry("diagnosis", &self.r#diagnosis)?;
        }
        if !self.r#account.is_empty() {
            state.serialize_entry("account", &self.r#account)?;
        }
        if let Some(some) = self.r#hospitalization.as_ref() {
            state.serialize_entry("hospitalization", some)?;
        }
        if !self.r#location.is_empty() {
            state.serialize_entry("location", &self.r#location)?;
        }
        if let Some(some) = self.r#service_provider.as_ref() {
            state.serialize_entry("serviceProvider", some)?;
        }
        if let Some(some) = self.r#part_of.as_ref() {
            state.serialize_entry("partOf", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Encounter {
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusHistory")]
            StatusHistory,
            #[serde(rename = "class")]
            Class,
            #[serde(rename = "classHistory")]
            ClassHistory,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "serviceType")]
            ServiceType,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "episodeOfCare")]
            EpisodeOfCare,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "participant")]
            Participant,
            #[serde(rename = "appointment")]
            Appointment,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "length")]
            Length,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "diagnosis")]
            Diagnosis,
            #[serde(rename = "account")]
            Account,
            #[serde(rename = "hospitalization")]
            Hospitalization,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "serviceProvider")]
            ServiceProvider,
            #[serde(rename = "partOf")]
            PartOf,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Encounter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Encounter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Encounter, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_history: Option<Vec<EncounterStatusHistory>> = None;
                let mut r#class: Option<Box<super::super::types::Coding>> = None;
                let mut r#class_history: Option<Vec<EncounterClassHistory>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#service_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#priority: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#episode_of_care: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#participant: Option<Vec<EncounterParticipant>> = None;
                let mut r#appointment: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#length: Option<Box<super::super::types::Duration>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#diagnosis: Option<Vec<EncounterDiagnosis>> = None;
                let mut r#account: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#hospitalization: Option<EncounterHospitalization> = None;
                let mut r#location: Option<Vec<EncounterLocation>> = None;
                let mut r#service_provider: Option<Box<super::super::types::Reference>> = None;
                let mut r#part_of: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Encounter" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Encounter",
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
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusPrimitiveElement => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::StatusHistory => {
                                if r#status_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusHistory"));
                                }
                                r#status_history = Some(map_access.next_value()?);
                            }
                            Field::Class => {
                                if r#class.is_some() {
                                    return Err(serde::de::Error::duplicate_field("class"));
                                }
                                r#class = Some(map_access.next_value()?);
                            }
                            Field::ClassHistory => {
                                if r#class_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field("classHistory"));
                                }
                                r#class_history = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::ServiceType => {
                                if r#service_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("serviceType"));
                                }
                                r#service_type = Some(map_access.next_value()?);
                            }
                            Field::Priority => {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::EpisodeOfCare => {
                                if r#episode_of_care.is_some() {
                                    return Err(serde::de::Error::duplicate_field("episodeOfCare"));
                                }
                                r#episode_of_care = Some(map_access.next_value()?);
                            }
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::Participant => {
                                if r#participant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("participant"));
                                }
                                r#participant = Some(map_access.next_value()?);
                            }
                            Field::Appointment => {
                                if r#appointment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("appointment"));
                                }
                                r#appointment = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Length => {
                                if r#length.is_some() {
                                    return Err(serde::de::Error::duplicate_field("length"));
                                }
                                r#length = Some(map_access.next_value()?);
                            }
                            Field::ReasonCode => {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code = Some(map_access.next_value()?);
                            }
                            Field::ReasonReference => {
                                if r#reason_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some(map_access.next_value()?);
                            }
                            Field::Diagnosis => {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diagnosis"));
                                }
                                r#diagnosis = Some(map_access.next_value()?);
                            }
                            Field::Account => {
                                if r#account.is_some() {
                                    return Err(serde::de::Error::duplicate_field("account"));
                                }
                                r#account = Some(map_access.next_value()?);
                            }
                            Field::Hospitalization => {
                                if r#hospitalization.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "hospitalization",
                                    ));
                                }
                                r#hospitalization = Some(map_access.next_value()?);
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::ServiceProvider => {
                                if r#service_provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "serviceProvider",
                                    ));
                                }
                                r#service_provider = Some(map_access.next_value()?);
                            }
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
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
                                        "status",
                                        "statusHistory",
                                        "class",
                                        "classHistory",
                                        "type",
                                        "serviceType",
                                        "priority",
                                        "subject",
                                        "episodeOfCare",
                                        "basedOn",
                                        "participant",
                                        "appointment",
                                        "period",
                                        "length",
                                        "reasonCode",
                                        "reasonReference",
                                        "diagnosis",
                                        "account",
                                        "hospitalization",
                                        "location",
                                        "serviceProvider",
                                        "partOf",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Encounter {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_history: r#status_history.unwrap_or(vec![]),
                        r#class: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#class.unwrap_or(Default::default())
                        } else {
                            r#class.ok_or(serde::de::Error::missing_field("class"))?
                        },
                        r#class_history: r#class_history.unwrap_or(vec![]),
                        r#type: r#type.unwrap_or(vec![]),
                        r#service_type,
                        r#priority,
                        r#subject,
                        r#episode_of_care: r#episode_of_care.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#participant: r#participant.unwrap_or(vec![]),
                        r#appointment: r#appointment.unwrap_or(vec![]),
                        r#period,
                        r#length,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#diagnosis: r#diagnosis.unwrap_or(vec![]),
                        r#account: r#account.unwrap_or(vec![]),
                        r#hospitalization,
                        r#location: r#location.unwrap_or(vec![]),
                        r#service_provider,
                        r#part_of,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
