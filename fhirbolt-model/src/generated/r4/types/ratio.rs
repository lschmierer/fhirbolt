// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.\n\nNeed to able to capture ratios for some measurements (titers) and some rates (costs)."]
#[derive(Default, Debug, Clone)]
pub struct Ratio {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The value of the numerator."]
    pub r#numerator: Option<Box<super::super::types::Quantity>>,
    #[doc = "The value of the denominator."]
    pub r#denominator: Option<Box<super::super::types::Quantity>>,
}
impl crate::AnyResource for Ratio {}
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "numerator")]
            Numerator,
            #[serde(rename = "denominator")]
            Denominator,
            Unknown(String),
        }
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Numerator => {
                                if r#numerator.is_some() {
                                    return Err(serde::de::Error::duplicate_field("numerator"));
                                }
                                r#numerator = Some(map_access.next_value()?);
                            }
                            Field::Denominator => {
                                if r#denominator.is_some() {
                                    return Err(serde::de::Error::duplicate_field("denominator"));
                                }
                                r#denominator = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &["id", "extension", "numerator", "denominator"],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(Ratio {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#numerator,
                        r#denominator,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
