// Generated on 2023-04-08 by fhirbolt-codegen v0.1.0
#[doc = "Does this service have specific eligibility requirements that need to be met in order to use the service?"]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthcareServiceEligibility {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded value for the eligibility."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the eligibility conditions for the service."]
    pub r#comment: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for HealthcareServiceEligibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for HealthcareServiceEligibility {
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
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &["id", "extension", "modifierExtension", "code", "comment"],
            ))
        }
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Comment => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return unknown_field_error("comment");
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "code", "comment"],
                                ));
                            },
                        }
                    }
                    Ok(HealthcareServiceEligibility {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code,
                        r#comment,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A collection of times that the Service Site is available."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthcareServiceAvailableTime {
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
impl serde::ser::Serialize for HealthcareServiceAvailableTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
impl<'de> serde::de::Deserialize<'de> for HealthcareServiceAvailableTime {
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
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &[
                    "id",
                    "extension",
                    "modifierExtension",
                    "daysOfWeek",
                    "allDay",
                    "availableStartTime",
                    "availableEndTime",
                ],
            ))
        }
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                    let vec = r#days_of_week.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                    return unknown_field_error("daysOfWeek");
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
                                    return unknown_field_error("allDay");
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
                                    return unknown_field_error("availableStartTime");
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
                                    return unknown_field_error("availableEndTime");
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
                    Ok(HealthcareServiceAvailableTime {
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
#[doc = "The HealthcareService is not available during this period of time due to the provided reason."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthcareServiceNotAvailable {
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
impl serde::ser::Serialize for HealthcareServiceNotAvailable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
impl<'de> serde::de::Deserialize<'de> for HealthcareServiceNotAvailable {
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
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &[
                    "id",
                    "extension",
                    "modifierExtension",
                    "description",
                    "during",
                ],
            ))
        }
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                    return unknown_field_error("description");
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
                    Ok(HealthcareServiceNotAvailable {
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
#[doc = "The details of a healthcare service available at a location."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthcareService {
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
    #[doc = "External identifiers for this item."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "This flag is used to mark the record to not be used. This is not used when a center is closed for maintenance, or for holidays, the notAvailable period is to be used for this."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The organization that provides this healthcare service."]
    pub r#provided_by: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the broad category of service being performed or delivered."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific type of service that may be delivered or performed."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Collection of specialties handled by the service site. This is more of a medical term."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The location(s) where this healthcare service may be provided."]
    pub r#location: Vec<Box<super::super::types::Reference>>,
    #[doc = "Further description of the service as it would be presented to a consumer while searching."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Any additional description of the service and/or any specific issues not covered by the other attributes, which can be displayed as further detail under the serviceName."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Extra details about the service that can't be placed in the other fields."]
    pub r#extra_details: Option<super::super::types::Markdown>,
    #[doc = "If there is a photo/symbol associated with this HealthcareService, it may be included here to facilitate quick identification of the service in a list."]
    pub r#photo: Option<Box<super::super::types::Attachment>>,
    #[doc = "List of contacts related to this specific healthcare service."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "The location(s) that this service is available to (not where the service is provided)."]
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    #[doc = "The code(s) that detail the conditions under which the healthcare service is available/offered."]
    pub r#service_provision_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Does this service have specific eligibility requirements that need to be met in order to use the service?"]
    pub r#eligibility: Vec<HealthcareServiceEligibility>,
    #[doc = "Programs that this service is applicable to."]
    pub r#program: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Collection of characteristics (attributes)."]
    pub r#characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Some services are specifically made available in multiple languages, this property permits a directory to declare the languages this is offered in. Typically this is only provided where a service operates in communities with mixed languages used."]
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Ways that the service accepts referrals, if this is not provided then it is implied that no referral is required."]
    pub r#referral_method: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates whether or not a prospective consumer will require an appointment for a particular service at a site to be provided by the Organization. Indicates if an appointment is required for access to this service."]
    pub r#appointment_required: Option<super::super::types::Boolean>,
    #[doc = "A collection of times that the Service Site is available."]
    pub r#available_time: Vec<HealthcareServiceAvailableTime>,
    #[doc = "The HealthcareService is not available during this period of time due to the provided reason."]
    pub r#not_available: Vec<HealthcareServiceNotAvailable>,
    #[doc = "A description of site availability exceptions, e.g. public holiday availability. Succinctly describing all possible exceptions to normal site availability as details in the available Times and not available Times."]
    pub r#availability_exceptions: Option<super::super::types::String>,
    #[doc = "Technical endpoints providing access to services operated for the specific healthcare services defined at this resource."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for HealthcareService {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for HealthcareService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "HealthcareService")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#extra_details.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("extraDetails", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_extraDetails", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#extra_details.as_ref() {
                    state.serialize_entry("extraDetails", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#appointment_required.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("appointmentRequired", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_appointmentRequired", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#appointment_required.as_ref() {
                    state.serialize_entry("appointmentRequired", some)?;
                }
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
impl<'de> serde::de::Deserialize<'de> for HealthcareService {
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
            #[serde(rename = "providedBy")]
            ProvidedBy,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "specialty")]
            Specialty,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            #[serde(rename = "extraDetails")]
            ExtraDetails,
            #[serde(rename = "_extraDetails")]
            ExtraDetailsPrimitiveElement,
            #[serde(rename = "photo")]
            Photo,
            #[serde(rename = "telecom")]
            Telecom,
            #[serde(rename = "coverageArea")]
            CoverageArea,
            #[serde(rename = "serviceProvisionCode")]
            ServiceProvisionCode,
            #[serde(rename = "eligibility")]
            Eligibility,
            #[serde(rename = "program")]
            Program,
            #[serde(rename = "characteristic")]
            Characteristic,
            #[serde(rename = "communication")]
            Communication,
            #[serde(rename = "referralMethod")]
            ReferralMethod,
            #[serde(rename = "appointmentRequired")]
            AppointmentRequired,
            #[serde(rename = "_appointmentRequired")]
            AppointmentRequiredPrimitiveElement,
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
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
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
                    "providedBy",
                    "category",
                    "type",
                    "specialty",
                    "location",
                    "name",
                    "comment",
                    "extraDetails",
                    "photo",
                    "telecom",
                    "coverageArea",
                    "serviceProvisionCode",
                    "eligibility",
                    "program",
                    "characteristic",
                    "communication",
                    "referralMethod",
                    "appointmentRequired",
                    "availableTime",
                    "notAvailable",
                    "availabilityExceptions",
                    "endpoint",
                ],
            ))
        }
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "HealthcareService" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"HealthcareService",
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
                                    return unknown_field_error("implicitRules");
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
                                    return unknown_field_error("language");
                                }
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Identifier => {
                                if _ctx.from_json {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#identifier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                    return unknown_field_error("active");
                                }
                            }
                            Field::ProvidedBy => {
                                if r#provided_by.is_some() {
                                    return Err(serde::de::Error::duplicate_field("providedBy"));
                                }
                                r#provided_by = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if _ctx.from_json {
                                    if r#category.is_some() {
                                        return Err(serde::de::Error::duplicate_field("category"));
                                    }
                                    r#category = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#category.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#type.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Specialty => {
                                if _ctx.from_json {
                                    if r#specialty.is_some() {
                                        return Err(serde::de::Error::duplicate_field("specialty"));
                                    }
                                    r#specialty = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#specialty.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Location => {
                                if _ctx.from_json {
                                    if r#location.is_some() {
                                        return Err(serde::de::Error::duplicate_field("location"));
                                    }
                                    r#location = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#location.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return unknown_field_error("name");
                                }
                            }
                            Field::Comment => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return unknown_field_error("comment");
                                }
                            }
                            Field::ExtraDetails => {
                                if _ctx.from_json {
                                    let some = r#extra_details.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "extraDetails",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#extra_details.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "extraDetails",
                                        ));
                                    }
                                    r#extra_details = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExtraDetailsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#extra_details.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_extraDetails",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("extraDetails");
                                }
                            }
                            Field::Photo => {
                                if r#photo.is_some() {
                                    return Err(serde::de::Error::duplicate_field("photo"));
                                }
                                r#photo = Some(map_access.next_value()?);
                            }
                            Field::Telecom => {
                                if _ctx.from_json {
                                    if r#telecom.is_some() {
                                        return Err(serde::de::Error::duplicate_field("telecom"));
                                    }
                                    r#telecom = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#telecom.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::CoverageArea => {
                                if _ctx.from_json {
                                    if r#coverage_area.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "coverageArea",
                                        ));
                                    }
                                    r#coverage_area = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#coverage_area.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ServiceProvisionCode => {
                                if _ctx.from_json {
                                    if r#service_provision_code.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "serviceProvisionCode",
                                        ));
                                    }
                                    r#service_provision_code = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#service_provision_code.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Eligibility => {
                                if _ctx.from_json {
                                    if r#eligibility.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "eligibility",
                                        ));
                                    }
                                    r#eligibility = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#eligibility.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Program => {
                                if _ctx.from_json {
                                    if r#program.is_some() {
                                        return Err(serde::de::Error::duplicate_field("program"));
                                    }
                                    r#program = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#program.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Characteristic => {
                                if _ctx.from_json {
                                    if r#characteristic.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "characteristic",
                                        ));
                                    }
                                    r#characteristic = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#characteristic.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Communication => {
                                if _ctx.from_json {
                                    if r#communication.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "communication",
                                        ));
                                    }
                                    r#communication = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#communication.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ReferralMethod => {
                                if _ctx.from_json {
                                    if r#referral_method.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "referralMethod",
                                        ));
                                    }
                                    r#referral_method = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#referral_method.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::AppointmentRequired => {
                                if _ctx.from_json {
                                    let some =
                                        r#appointment_required.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "appointmentRequired",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#appointment_required.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "appointmentRequired",
                                        ));
                                    }
                                    r#appointment_required = Some(map_access.next_value()?);
                                }
                            }
                            Field::AppointmentRequiredPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#appointment_required.get_or_insert(Default::default());
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
                                } else {
                                    return unknown_field_error("appointmentRequired");
                                }
                            }
                            Field::AvailableTime => {
                                if _ctx.from_json {
                                    if r#available_time.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "availableTime",
                                        ));
                                    }
                                    r#available_time = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#available_time.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::NotAvailable => {
                                if _ctx.from_json {
                                    if r#not_available.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "notAvailable",
                                        ));
                                    }
                                    r#not_available = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#not_available.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                    return unknown_field_error("availabilityExceptions");
                                }
                            }
                            Field::Endpoint => {
                                if _ctx.from_json {
                                    if r#endpoint.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    r#endpoint = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#endpoint.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                        "providedBy",
                                        "category",
                                        "type",
                                        "specialty",
                                        "location",
                                        "name",
                                        "comment",
                                        "extraDetails",
                                        "photo",
                                        "telecom",
                                        "coverageArea",
                                        "serviceProvisionCode",
                                        "eligibility",
                                        "program",
                                        "characteristic",
                                        "communication",
                                        "referralMethod",
                                        "appointmentRequired",
                                        "availableTime",
                                        "notAvailable",
                                        "availabilityExceptions",
                                        "endpoint",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
