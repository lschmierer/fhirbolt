// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Range {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#low: Option<Box<super::super::types::Quantity>>,
    pub r#high: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for Range {
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
        if let Some(some) = self.r#low.as_ref() {
            state.serialize_entry("low", some)?;
        }
        if let Some(some) = self.r#high.as_ref() {
            state.serialize_entry("high", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Range;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Range")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Range, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#low: Option<Box<super::super::types::Quantity>> = None;
                let mut r#high: Option<Box<super::super::types::Quantity>> = None;
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
                        "low" => {
                            if r#low.is_some() {
                                return Err(serde::de::Error::duplicate_field("low"));
                            }
                            r#low = Some(map_access.next_value()?);
                        }
                        "high" => {
                            if r#high.is_some() {
                                return Err(serde::de::Error::duplicate_field("high"));
                            }
                            r#high = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "low", "high"],
                            ))
                        }
                    }
                }
                Ok(Range {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#low,
                    r#high,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
