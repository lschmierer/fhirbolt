// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Ratio {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#numerator: Option<Box<super::super::types::Quantity>>,
    pub r#denominator: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for Ratio {
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
        if let Some(some) = self.r#numerator.as_ref() {
            state.serialize_entry("numerator", some)?;
        }
        if let Some(some) = self.r#denominator.as_ref() {
            state.serialize_entry("denominator", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Ratio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Ratio;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Ratio")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Ratio, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#numerator: Option<Box<super::super::types::Quantity>> = None;
                let mut r#denominator: Option<Box<super::super::types::Quantity>> = None;
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
                        "numerator" => {
                            if r#numerator.is_some() {
                                return Err(serde::de::Error::duplicate_field("numerator"));
                            }
                            r#numerator = Some(map_access.next_value()?);
                        }
                        "denominator" => {
                            if r#denominator.is_some() {
                                return Err(serde::de::Error::duplicate_field("denominator"));
                            }
                            r#denominator = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "numerator", "denominator"],
                            ))
                        }
                    }
                }
                Ok(Ratio {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#numerator,
                    r#denominator,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
