// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Range Type: A set of ordered Quantities defined by a low and high limit.\n\nNeed to be able to specify ranges of values."]
#[derive(Default, Debug, Clone)]
pub struct Range {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The low limit. The boundary is inclusive."]
    pub r#low: Option<Box<super::super::types::Quantity>>,
    #[doc = "The high limit. The boundary is inclusive."]
    pub r#high: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for Range {
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
            if let Some(some) = self.r#low.as_ref() {
                state.serialize_entry("low", some)?;
            }
            if let Some(some) = self.r#high.as_ref() {
                state.serialize_entry("high", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Range {
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
            #[serde(rename = "low")]
            Low,
            #[serde(rename = "high")]
            High,
            Unknown(std::string::String),
        }
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::Low => {
                                if r#low.is_some() {
                                    return Err(serde::de::Error::duplicate_field("low"));
                                }
                                r#low = Some(map_access.next_value()?);
                            }
                            Field::High => {
                                if r#high.is_some() {
                                    return Err(serde::de::Error::duplicate_field("high"));
                                }
                                r#high = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "low", "high"],
                                ));
                            },
                        }
                    }
                    Ok(Range {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#low,
                        r#high,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
