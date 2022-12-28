// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.\n\nThere is a need for a concise way to handle the data produced by devices that sample a physical state at a high frequency."]
#[derive(Default, Debug, Clone)]
pub struct SampledData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The base quantity that a measured value of zero represents. In addition, this provides the units of the entire measurement series."]
    pub r#origin: Box<super::super::types::Quantity>,
    #[doc = "The length of time between sampling times, measured in milliseconds."]
    pub r#period: super::super::types::Decimal,
    #[doc = "A correction factor that is applied to the sampled data points before they are added to the origin."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The lower limit of detection of the measured points. This is needed if any of the data points have the value \"L\" (lower than detection limit)."]
    pub r#lower_limit: Option<super::super::types::Decimal>,
    #[doc = "The upper limit of detection of the measured points. This is needed if any of the data points have the value \"U\" (higher than detection limit)."]
    pub r#upper_limit: Option<super::super::types::Decimal>,
    #[doc = "The number of sample points at each time point. If this value is greater than one, then the dimensions will be interlaced - all the sample points for a point in time will be recorded at once."]
    pub r#dimensions: super::super::types::PositiveInt,
    #[doc = "A series of data points which are decimal values separated by a single space (character u20). The special values \"E\" (error), \"L\" (below detection limit) and \"U\" (above detection limit) can also be used in place of a decimal value."]
    pub r#data: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SampledData {
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
            state.serialize_entry("origin", &self.r#origin)?;
            if _ctx.output_json {
                if let Some(some) = self.r#period.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("period", &some)?;
                }
                if self.r#period.id.is_some() || !self.r#period.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#period.id.as_ref(),
                        extension: &self.r#period.extension,
                    };
                    state.serialize_entry("_period", &primitive_element)?;
                }
            } else {
                state.serialize_entry("period", &self.r#period)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#lower_limit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("lowerLimit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lowerLimit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#lower_limit.as_ref() {
                    state.serialize_entry("lowerLimit", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#upper_limit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("upperLimit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_upperLimit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#upper_limit.as_ref() {
                    state.serialize_entry("upperLimit", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#dimensions.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("dimensions", &some)?;
                }
                if self.r#dimensions.id.is_some() || !self.r#dimensions.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#dimensions.id.as_ref(),
                        extension: &self.r#dimensions.extension,
                    };
                    state.serialize_entry("_dimensions", &primitive_element)?;
                }
            } else {
                state.serialize_entry("dimensions", &self.r#dimensions)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#data.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("data", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_data", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#data.as_ref() {
                    state.serialize_entry("data", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SampledData {
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
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "_period")]
            PeriodPrimitiveElement,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "lowerLimit")]
            LowerLimit,
            #[serde(rename = "_lowerLimit")]
            LowerLimitPrimitiveElement,
            #[serde(rename = "upperLimit")]
            UpperLimit,
            #[serde(rename = "_upperLimit")]
            UpperLimitPrimitiveElement,
            #[serde(rename = "dimensions")]
            Dimensions,
            #[serde(rename = "_dimensions")]
            DimensionsPrimitiveElement,
            #[serde(rename = "data")]
            Data,
            #[serde(rename = "_data")]
            DataPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SampledData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SampledData")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SampledData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#origin: Option<Box<super::super::types::Quantity>> = None;
                let mut r#period: Option<super::super::types::Decimal> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#lower_limit: Option<super::super::types::Decimal> = None;
                let mut r#upper_limit: Option<super::super::types::Decimal> = None;
                let mut r#dimensions: Option<super::super::types::PositiveInt> = None;
                let mut r#data: Option<super::super::types::String> = None;
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
                            Field::Origin => {
                                if r#origin.is_some() {
                                    return Err(serde::de::Error::duplicate_field("origin"));
                                }
                                r#origin = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if _ctx.from_json {
                                    let some = r#period.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("period"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#period.is_some() {
                                        return Err(serde::de::Error::duplicate_field("period"));
                                    }
                                    r#period = Some(map_access.next_value()?);
                                }
                            }
                            Field::PeriodPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#period.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_period"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "period",
                                        &[
                                            "id",
                                            "extension",
                                            "origin",
                                            "period",
                                            "factor",
                                            "lowerLimit",
                                            "upperLimit",
                                            "dimensions",
                                            "data",
                                        ],
                                    ));
                                }
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_factor"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "origin",
                                            "period",
                                            "factor",
                                            "lowerLimit",
                                            "upperLimit",
                                            "dimensions",
                                            "data",
                                        ],
                                    ));
                                }
                            }
                            Field::LowerLimit => {
                                if _ctx.from_json {
                                    let some = r#lower_limit.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lowerLimit",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#lower_limit.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lowerLimit",
                                        ));
                                    }
                                    r#lower_limit = Some(map_access.next_value()?);
                                }
                            }
                            Field::LowerLimitPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#lower_limit.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_lowerLimit",
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
                                        "lowerLimit",
                                        &[
                                            "id",
                                            "extension",
                                            "origin",
                                            "period",
                                            "factor",
                                            "lowerLimit",
                                            "upperLimit",
                                            "dimensions",
                                            "data",
                                        ],
                                    ));
                                }
                            }
                            Field::UpperLimit => {
                                if _ctx.from_json {
                                    let some = r#upper_limit.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "upperLimit",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#upper_limit.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "upperLimit",
                                        ));
                                    }
                                    r#upper_limit = Some(map_access.next_value()?);
                                }
                            }
                            Field::UpperLimitPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#upper_limit.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_upperLimit",
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
                                        "upperLimit",
                                        &[
                                            "id",
                                            "extension",
                                            "origin",
                                            "period",
                                            "factor",
                                            "lowerLimit",
                                            "upperLimit",
                                            "dimensions",
                                            "data",
                                        ],
                                    ));
                                }
                            }
                            Field::Dimensions => {
                                if _ctx.from_json {
                                    let some = r#dimensions.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "dimensions",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#dimensions.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "dimensions",
                                        ));
                                    }
                                    r#dimensions = Some(map_access.next_value()?);
                                }
                            }
                            Field::DimensionsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#dimensions.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_dimensions",
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
                                        "dimensions",
                                        &[
                                            "id",
                                            "extension",
                                            "origin",
                                            "period",
                                            "factor",
                                            "lowerLimit",
                                            "upperLimit",
                                            "dimensions",
                                            "data",
                                        ],
                                    ));
                                }
                            }
                            Field::Data => {
                                if _ctx.from_json {
                                    let some = r#data.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("data"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#data.is_some() {
                                        return Err(serde::de::Error::duplicate_field("data"));
                                    }
                                    r#data = Some(map_access.next_value()?);
                                }
                            }
                            Field::DataPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#data.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_data"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "data",
                                        &[
                                            "id",
                                            "extension",
                                            "origin",
                                            "period",
                                            "factor",
                                            "lowerLimit",
                                            "upperLimit",
                                            "dimensions",
                                            "data",
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
                                        "origin",
                                        "period",
                                        "factor",
                                        "lowerLimit",
                                        "upperLimit",
                                        "dimensions",
                                        "data",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SampledData {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#origin: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#origin.unwrap_or(Default::default())
                        } else {
                            r#origin.ok_or(serde::de::Error::missing_field("origin"))?
                        },
                        r#period: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#period.unwrap_or(Default::default())
                        } else {
                            r#period.ok_or(serde::de::Error::missing_field("period"))?
                        },
                        r#factor,
                        r#lower_limit,
                        r#upper_limit,
                        r#dimensions: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#dimensions.unwrap_or(Default::default())
                        } else {
                            r#dimensions.ok_or(serde::de::Error::missing_field("dimensions"))?
                        },
                        r#data,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
