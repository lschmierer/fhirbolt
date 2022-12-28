// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "A collection of times the practitioner is available or performing this role at the location and/or healthcareservice."]
#[derive(Default, Debug, Clone)]
pub struct PractitionerRoleAvailableTime {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates which days of the week are available between the start and end Times."]
    pub r#days_of_week: Vec<super::super::types::Code>,
    #[doc = "Is this always available? (hence times are irrelevant) e.g. 24 hour service."]
    pub r#all_day: Option<super::super::types::Boolean>,
    #[doc = "The opening time of day. Note: If the AllDay flag is set, then this time is ignored."]
    pub r#available_start_time: Option<super::super::types::Time>,
    #[doc = "The closing time of day. Note: If the AllDay flag is set, then this time is ignored."]
    pub r#available_end_time: Option<super::super::types::Time>,
}
impl serde::ser::Serialize for PractitionerRoleAvailableTime {
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
                if !self.r#days_of_week.is_empty() {
                    let values = self
                        .r#days_of_week
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#days_of_week.is_empty() {
                    state.serialize_entry("daysOfWeek", &self.r#days_of_week)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#all_day.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("allDay", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_allDay", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#all_day.as_ref() {
                    state.serialize_entry("allDay", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#available_start_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("availableStartTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_availableStartTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#available_start_time.as_ref() {
                    state.serialize_entry("availableStartTime", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#available_end_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("availableEndTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_availableEndTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#available_end_time.as_ref() {
                    state.serialize_entry("availableEndTime", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for PractitionerRoleAvailableTime {
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
            #[serde(rename = "daysOfWeek")]
            DaysOfWeek,
            #[serde(rename = "_daysOfWeek")]
            DaysOfWeekPrimitiveElement,
            #[serde(rename = "allDay")]
            AllDay,
            #[serde(rename = "_allDay")]
            AllDayPrimitiveElement,
            #[serde(rename = "availableStartTime")]
            AvailableStartTime,
            #[serde(rename = "_availableStartTime")]
            AvailableStartTimePrimitiveElement,
            #[serde(rename = "availableEndTime")]
            AvailableEndTime,
            #[serde(rename = "_availableEndTime")]
            AvailableEndTimePrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PractitionerRoleAvailableTime;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PractitionerRoleAvailableTime")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PractitionerRoleAvailableTime, V::Error>
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
                            Field::DaysOfWeek => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#days_of_week.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(values.len())
                                            .collect::<Vec<_>>(),
                                    );
                                    if vec.len() != values.len() {
                                        return Err(serde::de::Error::invalid_length(
                                            values.len(),
                                            &"primitive elements length",
                                        ));
                                    }
                                    if vec.iter().any(|v| v.value.is_some()) {
                                        return Err(serde::de::Error::duplicate_field(
                                            "daysOfWeek",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#days_of_week.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "daysOfWeek",
                                        ));
                                    }
                                    r#days_of_week = Some(map_access.next_value()?);
                                }
                            }
                            Field::DaysOfWeekPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#days_of_week.get_or_insert(
                                        std::iter::repeat(Default::default())
                                            .take(elements.len())
                                            .collect::<Vec<_>>(),
                                    );
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_daysOfWeek",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "daysOfWeek",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "daysOfWeek",
                                            "allDay",
                                            "availableStartTime",
                                            "availableEndTime",
                                        ],
                                    ));
                                }
                            }
                            Field::AllDay => {
                                if _ctx.from_json {
                                    let some = r#all_day.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("allDay"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#all_day.is_some() {
                                        return Err(serde::de::Error::duplicate_field("allDay"));
                                    }
                                    r#all_day = Some(map_access.next_value()?);
                                }
                            }
                            Field::AllDayPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "allDay",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "daysOfWeek",
                                            "allDay",
                                            "availableStartTime",
                                            "availableEndTime",
                                        ],
                                    ));
                                }
                            }
                            Field::AvailableStartTime => {
                                if _ctx.from_json {
                                    let some =
                                        r#available_start_time.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "availableStartTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#available_start_time.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "availableStartTime",
                                        ));
                                    }
                                    r#available_start_time = Some(map_access.next_value()?);
                                }
                            }
                            Field::AvailableStartTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#available_start_time.get_or_insert(Default::default());
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "availableStartTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "daysOfWeek",
                                            "allDay",
                                            "availableStartTime",
                                            "availableEndTime",
                                        ],
                                    ));
                                }
                            }
                            Field::AvailableEndTime => {
                                if _ctx.from_json {
                                    let some =
                                        r#available_end_time.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "availableEndTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#available_end_time.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "availableEndTime",
                                        ));
                                    }
                                    r#available_end_time = Some(map_access.next_value()?);
                                }
                            }
                            Field::AvailableEndTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#available_end_time.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_availableEndTime",
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
                                        "availableEndTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "daysOfWeek",
                                            "allDay",
                                            "availableStartTime",
                                            "availableEndTime",
                                        ],
                                    ));
                                }
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
                                        "daysOfWeek",
                                        "allDay",
                                        "availableStartTime",
                                        "availableEndTime",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PractitionerRoleAvailableTime {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#days_of_week: r#days_of_week.unwrap_or(vec![]),
                        r#all_day,
                        r#available_start_time,
                        r#available_end_time,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The practitioner is not available or performing this role during this period of time due to the provided reason."]
#[derive(Default, Debug, Clone)]
pub struct PractitionerRoleNotAvailable {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The reason that can be presented to the user as to why this time is not available."]
    pub r#description: super::super::types::String,
    #[doc = "Service is not available (seasonally or for a public holiday) from this date."]
    pub r#during: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for PractitionerRoleNotAvailable {
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
                if let Some(some) = self.r#description.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("description", &some)?;
                }
                if self.r#description.id.is_some() || !self.r#description.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#description.id.as_ref(),
                        extension: &self.r#description.extension,
                    };
                    state.serialize_entry("_description", &primitive_element)?;
                }
            } else {
                state.serialize_entry("description", &self.r#description)?;
            }
            if let Some(some) = self.r#during.as_ref() {
                state.serialize_entry("during", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for PractitionerRoleNotAvailable {
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
            #[serde(rename = "during")]
            During,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PractitionerRoleNotAvailable;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PractitionerRoleNotAvailable")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PractitionerRoleNotAvailable, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#during: Option<Box<super::super::types::Period>> = None;
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
                                            "during",
                                        ],
                                    ));
                                }
                            }
                            Field::During => {
                                if r#during.is_some() {
                                    return Err(serde::de::Error::duplicate_field("during"));
                                }
                                r#during = Some(map_access.next_value()?);
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
                                        "during",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PractitionerRoleNotAvailable {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#description.unwrap_or(Default::default())
                        } else {
                            r#description.ok_or(serde::de::Error::missing_field("description"))?
                        },
                        r#during,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A specific set of Roles/Locations/specialties/services that a practitioner may perform at an organization for a period of time.\n\nNeed to track services that a healthcare provider is able to provide at an organization's location, and the services that they can perform there."]
#[derive(Default, Debug, Clone)]
pub struct PractitionerRole {
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
    #[doc = "Business Identifiers that are specific to a role/location."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this practitioner role record is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The period during which the person is authorized to act as a practitioner in these role(s) for the organization."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Practitioner that is able to provide the defined services for the organization."]
    pub r#practitioner: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization where the Practitioner performs the roles associated."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Roles which this practitioner is authorized to perform for the organization."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific specialty of the practitioner."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The location(s) at which this practitioner provides care."]
    pub r#location: Vec<Box<super::super::types::Reference>>,
    #[doc = "The list of healthcare services that this worker provides for this role's Organization/Location(s)."]
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    #[doc = "Contact details that are specific to the role/location/service."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "A collection of times the practitioner is available or performing this role at the location and/or healthcareservice."]
    pub r#available_time: Vec<PractitionerRoleAvailableTime>,
    #[doc = "The practitioner is not available or performing this role during this period of time due to the provided reason."]
    pub r#not_available: Vec<PractitionerRoleNotAvailable>,
    #[doc = "A description of site availability exceptions, e.g. public holiday availability. Succinctly describing all possible exceptions to normal site availability as details in the available Times and not available Times."]
    pub r#availability_exceptions: Option<super::super::types::String>,
    #[doc = "Technical endpoints providing access to services operated for the practitioner with this role."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for PractitionerRole {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for PractitionerRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "PractitionerRole")?;
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
                if let Some(some) = self.r#active.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("active", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_active", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#active.as_ref() {
                    state.serialize_entry("active", some)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if let Some(some) = self.r#practitioner.as_ref() {
                state.serialize_entry("practitioner", some)?;
            }
            if let Some(some) = self.r#organization.as_ref() {
                state.serialize_entry("organization", some)?;
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if !self.r#specialty.is_empty() {
                state.serialize_entry("specialty", &self.r#specialty)?;
            }
            if !self.r#location.is_empty() {
                state.serialize_entry("location", &self.r#location)?;
            }
            if !self.r#healthcare_service.is_empty() {
                state.serialize_entry("healthcareService", &self.r#healthcare_service)?;
            }
            if !self.r#telecom.is_empty() {
                state.serialize_entry("telecom", &self.r#telecom)?;
            }
            if !self.r#available_time.is_empty() {
                state.serialize_entry("availableTime", &self.r#available_time)?;
            }
            if !self.r#not_available.is_empty() {
                state.serialize_entry("notAvailable", &self.r#not_available)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#availability_exceptions.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("availabilityExceptions", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_availabilityExceptions", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#availability_exceptions.as_ref() {
                    state.serialize_entry("availabilityExceptions", some)?;
                }
            }
            if !self.r#endpoint.is_empty() {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for PractitionerRole {
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
            #[serde(rename = "active")]
            Active,
            #[serde(rename = "_active")]
            ActivePrimitiveElement,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "practitioner")]
            Practitioner,
            #[serde(rename = "organization")]
            Organization,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "specialty")]
            Specialty,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "healthcareService")]
            HealthcareService,
            #[serde(rename = "telecom")]
            Telecom,
            #[serde(rename = "availableTime")]
            AvailableTime,
            #[serde(rename = "notAvailable")]
            NotAvailable,
            #[serde(rename = "availabilityExceptions")]
            AvailabilityExceptions,
            #[serde(rename = "_availabilityExceptions")]
            AvailabilityExceptionsPrimitiveElement,
            #[serde(rename = "endpoint")]
            Endpoint,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PractitionerRole;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PractitionerRole")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PractitionerRole, V::Error>
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
                let mut r#active: Option<super::super::types::Boolean> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#practitioner: Option<Box<super::super::types::Reference>> = None;
                let mut r#organization: Option<Box<super::super::types::Reference>> = None;
                let mut r#code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#specialty: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#location: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#healthcare_service: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#available_time: Option<Vec<PractitionerRoleAvailableTime>> = None;
                let mut r#not_available: Option<Vec<PractitionerRoleNotAvailable>> = None;
                let mut r#availability_exceptions: Option<super::super::types::String> = None;
                let mut r#endpoint: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "PractitionerRole" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"PractitionerRole",
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
                                            "active",
                                            "period",
                                            "practitioner",
                                            "organization",
                                            "code",
                                            "specialty",
                                            "location",
                                            "healthcareService",
                                            "telecom",
                                            "availableTime",
                                            "notAvailable",
                                            "availabilityExceptions",
                                            "endpoint",
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
                                            "active",
                                            "period",
                                            "practitioner",
                                            "organization",
                                            "code",
                                            "specialty",
                                            "location",
                                            "healthcareService",
                                            "telecom",
                                            "availableTime",
                                            "notAvailable",
                                            "availabilityExceptions",
                                            "endpoint",
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
                            Field::Active => {
                                if _ctx.from_json {
                                    let some = r#active.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("active"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#active.is_some() {
                                        return Err(serde::de::Error::duplicate_field("active"));
                                    }
                                    r#active = Some(map_access.next_value()?);
                                }
                            }
                            Field::ActivePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "active",
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
                                            "active",
                                            "period",
                                            "practitioner",
                                            "organization",
                                            "code",
                                            "specialty",
                                            "location",
                                            "healthcareService",
                                            "telecom",
                                            "availableTime",
                                            "notAvailable",
                                            "availabilityExceptions",
                                            "endpoint",
                                        ],
                                    ));
                                }
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Practitioner => {
                                if r#practitioner.is_some() {
                                    return Err(serde::de::Error::duplicate_field("practitioner"));
                                }
                                r#practitioner = Some(map_access.next_value()?);
                            }
                            Field::Organization => {
                                if r#organization.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organization"));
                                }
                                r#organization = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Specialty => {
                                if r#specialty.is_some() {
                                    return Err(serde::de::Error::duplicate_field("specialty"));
                                }
                                r#specialty = Some(map_access.next_value()?);
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::HealthcareService => {
                                if r#healthcare_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "healthcareService",
                                    ));
                                }
                                r#healthcare_service = Some(map_access.next_value()?);
                            }
                            Field::Telecom => {
                                if r#telecom.is_some() {
                                    return Err(serde::de::Error::duplicate_field("telecom"));
                                }
                                r#telecom = Some(map_access.next_value()?);
                            }
                            Field::AvailableTime => {
                                if r#available_time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("availableTime"));
                                }
                                r#available_time = Some(map_access.next_value()?);
                            }
                            Field::NotAvailable => {
                                if r#not_available.is_some() {
                                    return Err(serde::de::Error::duplicate_field("notAvailable"));
                                }
                                r#not_available = Some(map_access.next_value()?);
                            }
                            Field::AvailabilityExceptions => {
                                if _ctx.from_json {
                                    let some =
                                        r#availability_exceptions.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "availabilityExceptions",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#availability_exceptions.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "availabilityExceptions",
                                        ));
                                    }
                                    r#availability_exceptions = Some(map_access.next_value()?);
                                }
                            }
                            Field::AvailabilityExceptionsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#availability_exceptions.get_or_insert(Default::default());
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "availabilityExceptions",
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
                                            "active",
                                            "period",
                                            "practitioner",
                                            "organization",
                                            "code",
                                            "specialty",
                                            "location",
                                            "healthcareService",
                                            "telecom",
                                            "availableTime",
                                            "notAvailable",
                                            "availabilityExceptions",
                                            "endpoint",
                                        ],
                                    ));
                                }
                            }
                            Field::Endpoint => {
                                if r#endpoint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endpoint"));
                                }
                                r#endpoint = Some(map_access.next_value()?);
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
                                        "active",
                                        "period",
                                        "practitioner",
                                        "organization",
                                        "code",
                                        "specialty",
                                        "location",
                                        "healthcareService",
                                        "telecom",
                                        "availableTime",
                                        "notAvailable",
                                        "availabilityExceptions",
                                        "endpoint",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PractitionerRole {
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
                        r#period,
                        r#practitioner,
                        r#organization,
                        r#code: r#code.unwrap_or(vec![]),
                        r#specialty: r#specialty.unwrap_or(vec![]),
                        r#location: r#location.unwrap_or(vec![]),
                        r#healthcare_service: r#healthcare_service.unwrap_or(vec![]),
                        r#telecom: r#telecom.unwrap_or(vec![]),
                        r#available_time: r#available_time.unwrap_or(vec![]),
                        r#not_available: r#not_available.unwrap_or(vec![]),
                        r#availability_exceptions,
                        r#endpoint: r#endpoint.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
