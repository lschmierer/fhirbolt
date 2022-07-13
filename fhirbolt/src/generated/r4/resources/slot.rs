// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Slot {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#service_category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#service_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#appointment_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#schedule: Box<super::super::types::Reference>,
    pub r#status: super::super::types::Code,
    pub r#start: super::super::types::Instant,
    pub r#end: super::super::types::Instant,
    pub r#overbooked: Option<super::super::types::Boolean>,
    pub r#comment: Option<super::super::types::String>,
}
impl serde::ser::Serialize for Slot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Slot")?;
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
        if !self.r#service_category.is_empty() {
            state.serialize_entry("serviceCategory", &self.r#service_category)?;
        }
        if !self.r#service_type.is_empty() {
            state.serialize_entry("serviceType", &self.r#service_type)?;
        }
        if !self.r#specialty.is_empty() {
            state.serialize_entry("specialty", &self.r#specialty)?;
        }
        if let Some(some) = self.r#appointment_type.as_ref() {
            state.serialize_entry("appointmentType", some)?;
        }
        state.serialize_entry("schedule", &self.r#schedule)?;
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#start.value.as_ref() {
            state.serialize_entry("start", some)?;
        }
        if self.r#start.id.is_some() || !self.r#start.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#start.id,
                extension: &self.r#start.extension,
            };
            state.serialize_entry("_start", &primitive_element)?;
        }
        if let Some(some) = self.r#end.value.as_ref() {
            state.serialize_entry("end", some)?;
        }
        if self.r#end.id.is_some() || !self.r#end.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#end.id,
                extension: &self.r#end.extension,
            };
            state.serialize_entry("_end", &primitive_element)?;
        }
        if let Some(some) = self.r#overbooked.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("overbooked", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_overbooked", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#comment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("comment", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comment", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Slot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Slot;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Slot")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Slot, V::Error>
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
                let mut r#service_category: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#service_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#specialty: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#appointment_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#schedule: Option<Box<super::super::types::Reference>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#start: Option<super::super::types::Instant> = None;
                let mut r#end: Option<super::super::types::Instant> = None;
                let mut r#overbooked: Option<super::super::types::Boolean> = None;
                let mut r#comment: Option<super::super::types::String> = None;
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
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
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
                        "serviceCategory" => {
                            if r#service_category.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceCategory"));
                            }
                            r#service_category = Some(map_access.next_value()?);
                        }
                        "serviceType" => {
                            if r#service_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceType"));
                            }
                            r#service_type = Some(map_access.next_value()?);
                        }
                        "specialty" => {
                            if r#specialty.is_some() {
                                return Err(serde::de::Error::duplicate_field("specialty"));
                            }
                            r#specialty = Some(map_access.next_value()?);
                        }
                        "appointmentType" => {
                            if r#appointment_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("appointmentType"));
                            }
                            r#appointment_type = Some(map_access.next_value()?);
                        }
                        "schedule" => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            r#schedule = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
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
                        "start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_start"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_end"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "overbooked" => {
                            let some = r#overbooked.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("overbooked"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_overbooked" => {
                            let some = r#overbooked.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_overbooked"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "comment" => {
                            let some = r#comment.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_comment" => {
                            let some = r#comment.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_comment"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                                    "service_category",
                                    "service_type",
                                    "specialty",
                                    "appointment_type",
                                    "schedule",
                                    "status",
                                    "start",
                                    "end",
                                    "overbooked",
                                    "comment",
                                ],
                            ))
                        }
                    }
                }
                Ok(Slot {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#service_category: r#service_category.unwrap_or(vec![]),
                    r#service_type: r#service_type.unwrap_or(vec![]),
                    r#specialty: r#specialty.unwrap_or(vec![]),
                    r#appointment_type,
                    r#schedule: r#schedule.ok_or(serde::de::Error::missing_field("schedule"))?,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#start: r#start.ok_or(serde::de::Error::missing_field("start"))?,
                    r#end: r#end.ok_or(serde::de::Error::missing_field("end"))?,
                    r#overbooked,
                    r#comment,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
