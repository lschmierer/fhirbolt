// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct SampledData {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#origin: Box<super::super::types::Quantity>,
    pub r#period: super::super::types::Decimal,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#lower_limit: Option<super::super::types::Decimal>,
    pub r#upper_limit: Option<super::super::types::Decimal>,
    pub r#dimensions: super::super::types::PositiveInt,
    pub r#data: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SampledData {
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
        state.serialize_entry("origin", &self.r#origin)?;
        if let Some(some) = self.r#period.value.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if self.r#period.id.is_some() || !self.r#period.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#period.id,
                extension: &self.r#period.extension,
            };
            state.serialize_entry("_period", &primitive_element)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#lower_limit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lowerLimit", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lowerLimit", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#upper_limit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("upperLimit", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_upperLimit", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#dimensions.value.as_ref() {
            state.serialize_entry("dimensions", some)?;
        }
        if self.r#dimensions.id.is_some() || !self.r#dimensions.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#dimensions.id,
                extension: &self.r#dimensions.extension,
            };
            state.serialize_entry("_dimensions", &primitive_element)?;
        }
        if let Some(some) = self.r#data.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("data", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_data", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SampledData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                        "origin" => {
                            if r#origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            r#origin = Some(map_access.next_value()?);
                        }
                        "period" => {
                            let some = r#period.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_period" => {
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
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
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
                        }
                        "lowerLimit" => {
                            let some = r#lower_limit.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lowerLimit"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_lowerLimit" => {
                            let some = r#lower_limit.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lowerLimit"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "upperLimit" => {
                            let some = r#upper_limit.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("upperLimit"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_upperLimit" => {
                            let some = r#upper_limit.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_upperLimit"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "dimensions" => {
                            let some = r#dimensions.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_dimensions" => {
                            let some = r#dimensions.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_dimensions"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "data" => {
                            let some = r#data.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_data" => {
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
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "origin",
                                    "period",
                                    "factor",
                                    "lower_limit",
                                    "upper_limit",
                                    "dimensions",
                                    "data",
                                ],
                            ))
                        }
                    }
                }
                Ok(SampledData {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#origin: r#origin.ok_or(serde::de::Error::missing_field("origin"))?,
                    r#period: r#period.ok_or(serde::de::Error::missing_field("period"))?,
                    r#factor,
                    r#lower_limit,
                    r#upper_limit,
                    r#dimensions: r#dimensions
                        .ok_or(serde::de::Error::missing_field("dimensions"))?,
                    r#data,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
