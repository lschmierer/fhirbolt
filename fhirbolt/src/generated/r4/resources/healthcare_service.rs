// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct HealthcareServiceEligibility {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#comment: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for HealthcareServiceEligibility {
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
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
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
impl<'de> serde::de::Deserialize<'de> for HealthcareServiceEligibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HealthcareServiceEligibility;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("HealthcareServiceEligibility")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<HealthcareServiceEligibility, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#comment: Option<super::super::types::Markdown> = None;
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
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
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
                                &["id", "extension", "modifier_extension", "code", "comment"],
                            ))
                        }
                    }
                }
                Ok(HealthcareServiceEligibility {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code,
                    r#comment,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct HealthcareServiceAvailableTime {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#days_of_week: Vec<super::super::types::Code>,
    pub r#all_day: Option<super::super::types::Boolean>,
    pub r#available_start_time: Option<super::super::types::Time>,
    pub r#available_end_time: Option<super::super::types::Time>,
}
impl serde::ser::Serialize for HealthcareServiceAvailableTime {
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
        if !self.r#days_of_week.is_empty() {
            let values: Vec<_> = self.r#days_of_week.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("daysOfWeek", &values)?;
            }
            let requires_elements = self
                .r#days_of_week
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#days_of_week
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_daysOfWeek", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#all_day.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("allDay", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_allDay", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#available_start_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("availableStartTime", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_availableStartTime", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#available_end_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("availableEndTime", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_availableEndTime", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for HealthcareServiceAvailableTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HealthcareServiceAvailableTime;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("HealthcareServiceAvailableTime")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<HealthcareServiceAvailableTime, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#days_of_week: Option<Vec<super::super::types::Code>> = None;
                let mut r#all_day: Option<super::super::types::Boolean> = None;
                let mut r#available_start_time: Option<super::super::types::Time> = None;
                let mut r#available_end_time: Option<super::super::types::Time> = None;
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
                        "daysOfWeek" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#days_of_week.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("daysOfWeek"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_daysOfWeek" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#days_of_week.get_or_insert(Vec::with_capacity(elements.len()));
                            if vec.len() != elements.len() {
                                return Err(serde::de::Error::invalid_length(
                                    elements.len(),
                                    &"primitive values length",
                                ));
                            }
                            if vec
                                .iter()
                                .any(|e| e.id.is_some() || !e.extension.is_empty())
                            {
                                return Err(serde::de::Error::duplicate_field("_daysOfWeek"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "allDay" => {
                            let some = r#all_day.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("allDay"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_allDay" => {
                            let some = r#all_day.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_allDay"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "availableStartTime" => {
                            let some = r#available_start_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "availableStartTime",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_availableStartTime" => {
                            let some = r#available_start_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_availableStartTime",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "availableEndTime" => {
                            let some = r#available_end_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("availableEndTime"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_availableEndTime" => {
                            let some = r#available_end_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_availableEndTime"));
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
                                    "extension",
                                    "modifier_extension",
                                    "days_of_week",
                                    "all_day",
                                    "available_start_time",
                                    "available_end_time",
                                ],
                            ))
                        }
                    }
                }
                Ok(HealthcareServiceAvailableTime {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#days_of_week: r#days_of_week.unwrap_or(vec![]),
                    r#all_day,
                    r#available_start_time,
                    r#available_end_time,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct HealthcareServiceNotAvailable {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: super::super::types::String,
    pub r#during: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for HealthcareServiceNotAvailable {
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
        if let Some(some) = self.r#description.value.as_ref() {
            state.serialize_entry("description", some)?;
        }
        if self.r#description.id.is_some() || !self.r#description.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#description.id,
                extension: &self.r#description.extension,
            };
            state.serialize_entry("_description", &primitive_element)?;
        }
        if let Some(some) = self.r#during.as_ref() {
            state.serialize_entry("during", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for HealthcareServiceNotAvailable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HealthcareServiceNotAvailable;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("HealthcareServiceNotAvailable")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<HealthcareServiceNotAvailable, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#during: Option<Box<super::super::types::Period>> = None;
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
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "during" => {
                            if r#during.is_some() {
                                return Err(serde::de::Error::duplicate_field("during"));
                            }
                            r#during = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "description",
                                    "during",
                                ],
                            ))
                        }
                    }
                }
                Ok(HealthcareServiceNotAvailable {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#description: r#description
                        .ok_or(serde::de::Error::missing_field("description"))?,
                    r#during,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct HealthcareService {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#provided_by: Option<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Vec<Box<super::super::types::Reference>>,
    pub r#name: Option<super::super::types::String>,
    pub r#comment: Option<super::super::types::String>,
    pub r#extra_details: Option<super::super::types::Markdown>,
    pub r#photo: Option<Box<super::super::types::Attachment>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    pub r#service_provision_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#eligibility: Vec<HealthcareServiceEligibility>,
    pub r#program: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#referral_method: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#appointment_required: Option<super::super::types::Boolean>,
    pub r#available_time: Vec<HealthcareServiceAvailableTime>,
    pub r#not_available: Vec<HealthcareServiceNotAvailable>,
    pub r#availability_exceptions: Option<super::super::types::String>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for HealthcareService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "HealthcareService")?;
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
        if let Some(some) = self.r#active.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("active", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_active", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#provided_by.as_ref() {
            state.serialize_entry("providedBy", some)?;
        }
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if !self.r#specialty.is_empty() {
            state.serialize_entry("specialty", &self.r#specialty)?;
        }
        if !self.r#location.is_empty() {
            state.serialize_entry("location", &self.r#location)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
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
        if let Some(some) = self.r#extra_details.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("extraDetails", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_extraDetails", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#photo.as_ref() {
            state.serialize_entry("photo", some)?;
        }
        if !self.r#telecom.is_empty() {
            state.serialize_entry("telecom", &self.r#telecom)?;
        }
        if !self.r#coverage_area.is_empty() {
            state.serialize_entry("coverageArea", &self.r#coverage_area)?;
        }
        if !self.r#service_provision_code.is_empty() {
            state.serialize_entry("serviceProvisionCode", &self.r#service_provision_code)?;
        }
        if !self.r#eligibility.is_empty() {
            state.serialize_entry("eligibility", &self.r#eligibility)?;
        }
        if !self.r#program.is_empty() {
            state.serialize_entry("program", &self.r#program)?;
        }
        if !self.r#characteristic.is_empty() {
            state.serialize_entry("characteristic", &self.r#characteristic)?;
        }
        if !self.r#communication.is_empty() {
            state.serialize_entry("communication", &self.r#communication)?;
        }
        if !self.r#referral_method.is_empty() {
            state.serialize_entry("referralMethod", &self.r#referral_method)?;
        }
        if let Some(some) = self.r#appointment_required.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("appointmentRequired", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_appointmentRequired", &primitive_element)?;
            }
        }
        if !self.r#available_time.is_empty() {
            state.serialize_entry("availableTime", &self.r#available_time)?;
        }
        if !self.r#not_available.is_empty() {
            state.serialize_entry("notAvailable", &self.r#not_available)?;
        }
        if let Some(some) = self.r#availability_exceptions.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("availabilityExceptions", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_availabilityExceptions", &primitive_element)?;
            }
        }
        if !self.r#endpoint.is_empty() {
            state.serialize_entry("endpoint", &self.r#endpoint)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for HealthcareService {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HealthcareService;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("HealthcareService")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<HealthcareService, V::Error>
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
                let mut r#active: Option<super::super::types::Boolean> = None;
                let mut r#provided_by: Option<Box<super::super::types::Reference>> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#specialty: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#location: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#comment: Option<super::super::types::String> = None;
                let mut r#extra_details: Option<super::super::types::Markdown> = None;
                let mut r#photo: Option<Box<super::super::types::Attachment>> = None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#coverage_area: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#service_provision_code: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#eligibility: Option<Vec<HealthcareServiceEligibility>> = None;
                let mut r#program: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#characteristic: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#communication: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#referral_method: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#appointment_required: Option<super::super::types::Boolean> = None;
                let mut r#available_time: Option<Vec<HealthcareServiceAvailableTime>> = None;
                let mut r#not_available: Option<Vec<HealthcareServiceNotAvailable>> = None;
                let mut r#availability_exceptions: Option<super::super::types::String> = None;
                let mut r#endpoint: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                        "active" => {
                            let some = r#active.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_active" => {
                            let some = r#active.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_active"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "providedBy" => {
                            if r#provided_by.is_some() {
                                return Err(serde::de::Error::duplicate_field("providedBy"));
                            }
                            r#provided_by = Some(map_access.next_value()?);
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "specialty" => {
                            if r#specialty.is_some() {
                                return Err(serde::de::Error::duplicate_field("specialty"));
                            }
                            r#specialty = Some(map_access.next_value()?);
                        }
                        "location" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
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
                        "extraDetails" => {
                            let some = r#extra_details.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraDetails"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_extraDetails" => {
                            let some = r#extra_details.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_extraDetails"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "photo" => {
                            if r#photo.is_some() {
                                return Err(serde::de::Error::duplicate_field("photo"));
                            }
                            r#photo = Some(map_access.next_value()?);
                        }
                        "telecom" => {
                            if r#telecom.is_some() {
                                return Err(serde::de::Error::duplicate_field("telecom"));
                            }
                            r#telecom = Some(map_access.next_value()?);
                        }
                        "coverageArea" => {
                            if r#coverage_area.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverageArea"));
                            }
                            r#coverage_area = Some(map_access.next_value()?);
                        }
                        "serviceProvisionCode" => {
                            if r#service_provision_code.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "serviceProvisionCode",
                                ));
                            }
                            r#service_provision_code = Some(map_access.next_value()?);
                        }
                        "eligibility" => {
                            if r#eligibility.is_some() {
                                return Err(serde::de::Error::duplicate_field("eligibility"));
                            }
                            r#eligibility = Some(map_access.next_value()?);
                        }
                        "program" => {
                            if r#program.is_some() {
                                return Err(serde::de::Error::duplicate_field("program"));
                            }
                            r#program = Some(map_access.next_value()?);
                        }
                        "characteristic" => {
                            if r#characteristic.is_some() {
                                return Err(serde::de::Error::duplicate_field("characteristic"));
                            }
                            r#characteristic = Some(map_access.next_value()?);
                        }
                        "communication" => {
                            if r#communication.is_some() {
                                return Err(serde::de::Error::duplicate_field("communication"));
                            }
                            r#communication = Some(map_access.next_value()?);
                        }
                        "referralMethod" => {
                            if r#referral_method.is_some() {
                                return Err(serde::de::Error::duplicate_field("referralMethod"));
                            }
                            r#referral_method = Some(map_access.next_value()?);
                        }
                        "appointmentRequired" => {
                            let some = r#appointment_required.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "appointmentRequired",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_appointmentRequired" => {
                            let some = r#appointment_required.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_appointmentRequired",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "availableTime" => {
                            if r#available_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("availableTime"));
                            }
                            r#available_time = Some(map_access.next_value()?);
                        }
                        "notAvailable" => {
                            if r#not_available.is_some() {
                                return Err(serde::de::Error::duplicate_field("notAvailable"));
                            }
                            r#not_available = Some(map_access.next_value()?);
                        }
                        "availabilityExceptions" => {
                            let some = r#availability_exceptions.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "availabilityExceptions",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_availabilityExceptions" => {
                            let some = r#availability_exceptions.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_availabilityExceptions",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "endpoint" => {
                            if r#endpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            r#endpoint = Some(map_access.next_value()?);
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
                                    "active",
                                    "provided_by",
                                    "category",
                                    "type",
                                    "specialty",
                                    "location",
                                    "name",
                                    "comment",
                                    "extra_details",
                                    "photo",
                                    "telecom",
                                    "coverage_area",
                                    "service_provision_code",
                                    "eligibility",
                                    "program",
                                    "characteristic",
                                    "communication",
                                    "referral_method",
                                    "appointment_required",
                                    "available_time",
                                    "not_available",
                                    "availability_exceptions",
                                    "endpoint",
                                ],
                            ))
                        }
                    }
                }
                Ok(HealthcareService {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#active,
                    r#provided_by,
                    r#category: r#category.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#specialty: r#specialty.unwrap_or(vec![]),
                    r#location: r#location.unwrap_or(vec![]),
                    r#name,
                    r#comment,
                    r#extra_details,
                    r#photo,
                    r#telecom: r#telecom.unwrap_or(vec![]),
                    r#coverage_area: r#coverage_area.unwrap_or(vec![]),
                    r#service_provision_code: r#service_provision_code.unwrap_or(vec![]),
                    r#eligibility: r#eligibility.unwrap_or(vec![]),
                    r#program: r#program.unwrap_or(vec![]),
                    r#characteristic: r#characteristic.unwrap_or(vec![]),
                    r#communication: r#communication.unwrap_or(vec![]),
                    r#referral_method: r#referral_method.unwrap_or(vec![]),
                    r#appointment_required,
                    r#available_time: r#available_time.unwrap_or(vec![]),
                    r#not_available: r#not_available.unwrap_or(vec![]),
                    r#availability_exceptions,
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
