// Generated on 2023-04-08 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.\n\nNeed to track phone, fax, mobile, sms numbers, email addresses, twitter tags, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContactPoint {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Telecommunications form for contact point - what communications system is required to make use of the contact."]
    pub r#system: Option<super::super::types::Code>,
    #[doc = "The actual contact point details, in a form that is meaningful to the designated communication system (i.e. phone number or email address)."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "Identifies the purpose for the contact point."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "Specifies a preferred order in which to use a set of contacts. ContactPoints with lower rank values are more preferred than those with higher rank values."]
    pub r#rank: Option<super::super::types::PositiveInt>,
    #[doc = "Time period when the contact point was/is in use."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for ContactPoint {
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
            if _ctx.output_json {
                if let Some(some) = self.r#system.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("system", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_system", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#system.as_ref() {
                    state.serialize_entry("system", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("value", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_value", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value.as_ref() {
                    state.serialize_entry("value", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#use.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("use", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_use", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#use.as_ref() {
                    state.serialize_entry("use", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#rank.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("rank", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_rank", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#rank.as_ref() {
                    state.serialize_entry("rank", some)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContactPoint {
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
            #[serde(rename = "system")]
            System,
            #[serde(rename = "_system")]
            SystemPrimitiveElement,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "_use")]
            UsePrimitiveElement,
            #[serde(rename = "rank")]
            Rank,
            #[serde(rename = "_rank")]
            RankPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &[
                    "id",
                    "extension",
                    "system",
                    "value",
                    "use",
                    "rank",
                    "period",
                ],
            ))
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContactPoint;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContactPoint")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContactPoint, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#system: Option<super::super::types::Code> = None;
                let mut r#value: Option<super::super::types::String> = None;
                let mut r#use: Option<super::super::types::Code> = None;
                let mut r#rank: Option<super::super::types::PositiveInt> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                            Field::System => {
                                if _ctx.from_json {
                                    let some = r#system.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("system"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#system.is_some() {
                                        return Err(serde::de::Error::duplicate_field("system"));
                                    }
                                    r#system = Some(map_access.next_value()?);
                                }
                            }
                            Field::SystemPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#system.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_system"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("system");
                                }
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("value");
                                }
                            }
                            Field::Use => {
                                if _ctx.from_json {
                                    let some = r#use.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("use"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#use.is_some() {
                                        return Err(serde::de::Error::duplicate_field("use"));
                                    }
                                    r#use = Some(map_access.next_value()?);
                                }
                            }
                            Field::UsePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#use.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_use"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("use");
                                }
                            }
                            Field::Rank => {
                                if _ctx.from_json {
                                    let some = r#rank.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("rank"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#rank.is_some() {
                                        return Err(serde::de::Error::duplicate_field("rank"));
                                    }
                                    r#rank = Some(map_access.next_value()?);
                                }
                            }
                            Field::RankPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return unknown_field_error("rank");
                                }
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "system",
                                        "value",
                                        "use",
                                        "rank",
                                        "period",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContactPoint {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#system,
                        r#value,
                        r#use,
                        r#rank,
                        r#period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
