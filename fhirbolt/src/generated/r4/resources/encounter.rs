// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct EncounterStatusHistory {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#period: Box<super::super::types::Period>,
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
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
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
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "status", "period"],
                            ))
                        }
                    }
                }
                Ok(EncounterStatusHistory {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#period: r#period.ok_or(serde::de::Error::missing_field("period"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct EncounterClassHistory {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#class: Box<super::super::types::Coding>,
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
                        "class" => {
                            if r#class.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            r#class = Some(map_access.next_value()?);
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "class", "period"],
                            ))
                        }
                    }
                }
                Ok(EncounterClassHistory {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#class: r#class.ok_or(serde::de::Error::missing_field("class"))?,
                    r#period: r#period.ok_or(serde::de::Error::missing_field("period"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct EncounterParticipant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        "individual" => {
                            if r#individual.is_some() {
                                return Err(serde::de::Error::duplicate_field("individual"));
                            }
                            r#individual = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "period",
                                    "individual",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct EncounterDiagnosis {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#condition: Box<super::super::types::Reference>,
    pub r#use: Option<Box<super::super::types::CodeableConcept>>,
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
                state.serialize_entry("rank", some)?;
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
                        "condition" => {
                            if r#condition.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            r#condition = Some(map_access.next_value()?);
                        }
                        "use" => {
                            if r#use.is_some() {
                                return Err(serde::de::Error::duplicate_field("use"));
                            }
                            r#use = Some(map_access.next_value()?);
                        }
                        "rank" => {
                            let some = r#rank.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("rank"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_rank" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#rank.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_rank"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "condition",
                                    "use",
                                    "rank",
                                ],
                            ))
                        }
                    }
                }
                Ok(EncounterDiagnosis {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#condition: r#condition.ok_or(serde::de::Error::missing_field("condition"))?,
                    r#use,
                    r#rank,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct EncounterHospitalization {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#pre_admission_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#origin: Option<Box<super::super::types::Reference>>,
    pub r#admit_source: Option<Box<super::super::types::CodeableConcept>>,
    pub r#re_admission: Option<Box<super::super::types::CodeableConcept>>,
    pub r#diet_preference: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#special_courtesy: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#special_arrangement: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#destination: Option<Box<super::super::types::Reference>>,
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
                        "preAdmissionIdentifier" => {
                            if r#pre_admission_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "preAdmissionIdentifier",
                                ));
                            }
                            r#pre_admission_identifier = Some(map_access.next_value()?);
                        }
                        "origin" => {
                            if r#origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            r#origin = Some(map_access.next_value()?);
                        }
                        "admitSource" => {
                            if r#admit_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("admitSource"));
                            }
                            r#admit_source = Some(map_access.next_value()?);
                        }
                        "reAdmission" => {
                            if r#re_admission.is_some() {
                                return Err(serde::de::Error::duplicate_field("reAdmission"));
                            }
                            r#re_admission = Some(map_access.next_value()?);
                        }
                        "dietPreference" => {
                            if r#diet_preference.is_some() {
                                return Err(serde::de::Error::duplicate_field("dietPreference"));
                            }
                            r#diet_preference = Some(map_access.next_value()?);
                        }
                        "specialCourtesy" => {
                            if r#special_courtesy.is_some() {
                                return Err(serde::de::Error::duplicate_field("specialCourtesy"));
                            }
                            r#special_courtesy = Some(map_access.next_value()?);
                        }
                        "specialArrangement" => {
                            if r#special_arrangement.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "specialArrangement",
                                ));
                            }
                            r#special_arrangement = Some(map_access.next_value()?);
                        }
                        "destination" => {
                            if r#destination.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            r#destination = Some(map_access.next_value()?);
                        }
                        "dischargeDisposition" => {
                            if r#discharge_disposition.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "dischargeDisposition",
                                ));
                            }
                            r#discharge_disposition = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "pre_admission_identifier",
                                    "origin",
                                    "admit_source",
                                    "re_admission",
                                    "diet_preference",
                                    "special_courtesy",
                                    "special_arrangement",
                                    "destination",
                                    "discharge_disposition",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct EncounterLocation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#location: Box<super::super::types::Reference>,
    pub r#status: Option<super::super::types::Code>,
    pub r#physical_type: Option<Box<super::super::types::CodeableConcept>>,
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
                state.serialize_entry("status", some)?;
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
                        "location" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "physicalType" => {
                            if r#physical_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("physicalType"));
                            }
                            r#physical_type = Some(map_access.next_value()?);
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "location",
                                    "status",
                                    "physical_type",
                                    "period",
                                ],
                            ))
                        }
                    }
                }
                Ok(EncounterLocation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#location: r#location.ok_or(serde::de::Error::missing_field("location"))?,
                    r#status,
                    r#physical_type,
                    r#period,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Encounter {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#status_history: Vec<EncounterStatusHistory>,
    pub r#class: Box<super::super::types::Coding>,
    pub r#class_history: Vec<EncounterClassHistory>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#service_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#episode_of_care: Vec<Box<super::super::types::Reference>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#participant: Vec<EncounterParticipant>,
    pub r#appointment: Vec<Box<super::super::types::Reference>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#length: Option<Box<super::super::types::Duration>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#diagnosis: Vec<EncounterDiagnosis>,
    pub r#account: Vec<Box<super::super::types::Reference>>,
    pub r#hospitalization: Option<EncounterHospitalization>,
    pub r#location: Vec<EncounterLocation>,
    pub r#service_provider: Option<Box<super::super::types::Reference>>,
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "statusHistory" => {
                            if r#status_history.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusHistory"));
                            }
                            r#status_history = Some(map_access.next_value()?);
                        }
                        "class" => {
                            if r#class.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            r#class = Some(map_access.next_value()?);
                        }
                        "classHistory" => {
                            if r#class_history.is_some() {
                                return Err(serde::de::Error::duplicate_field("classHistory"));
                            }
                            r#class_history = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "serviceType" => {
                            if r#service_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceType"));
                            }
                            r#service_type = Some(map_access.next_value()?);
                        }
                        "priority" => {
                            if r#priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            r#priority = Some(map_access.next_value()?);
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "episodeOfCare" => {
                            if r#episode_of_care.is_some() {
                                return Err(serde::de::Error::duplicate_field("episodeOfCare"));
                            }
                            r#episode_of_care = Some(map_access.next_value()?);
                        }
                        "basedOn" => {
                            if r#based_on.is_some() {
                                return Err(serde::de::Error::duplicate_field("basedOn"));
                            }
                            r#based_on = Some(map_access.next_value()?);
                        }
                        "participant" => {
                            if r#participant.is_some() {
                                return Err(serde::de::Error::duplicate_field("participant"));
                            }
                            r#participant = Some(map_access.next_value()?);
                        }
                        "appointment" => {
                            if r#appointment.is_some() {
                                return Err(serde::de::Error::duplicate_field("appointment"));
                            }
                            r#appointment = Some(map_access.next_value()?);
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        "length" => {
                            if r#length.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            r#length = Some(map_access.next_value()?);
                        }
                        "reasonCode" => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        "reasonReference" => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        "diagnosis" => {
                            if r#diagnosis.is_some() {
                                return Err(serde::de::Error::duplicate_field("diagnosis"));
                            }
                            r#diagnosis = Some(map_access.next_value()?);
                        }
                        "account" => {
                            if r#account.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            r#account = Some(map_access.next_value()?);
                        }
                        "hospitalization" => {
                            if r#hospitalization.is_some() {
                                return Err(serde::de::Error::duplicate_field("hospitalization"));
                            }
                            r#hospitalization = Some(map_access.next_value()?);
                        }
                        "location" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        "serviceProvider" => {
                            if r#service_provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceProvider"));
                            }
                            r#service_provider = Some(map_access.next_value()?);
                        }
                        "partOf" => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            r#part_of = Some(map_access.next_value()?);
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
                                    "identifier",
                                    "status",
                                    "status_history",
                                    "class",
                                    "class_history",
                                    "type",
                                    "service_type",
                                    "priority",
                                    "subject",
                                    "episode_of_care",
                                    "based_on",
                                    "participant",
                                    "appointment",
                                    "period",
                                    "length",
                                    "reason_code",
                                    "reason_reference",
                                    "diagnosis",
                                    "account",
                                    "hospitalization",
                                    "location",
                                    "service_provider",
                                    "part_of",
                                ],
                            ))
                        }
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
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#status_history: r#status_history.unwrap_or(vec![]),
                    r#class: r#class.ok_or(serde::de::Error::missing_field("class"))?,
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
