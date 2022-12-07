// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "A value that defines the context specified in this context of use. The interpretation of the value is defined by the code."]
#[derive(Debug, Clone)]
pub enum UsageContextValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for UsageContextValue {
    fn default() -> UsageContextValue {
        UsageContextValue::Invalid
    }
}
#[doc = "Base StructureDefinition for UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).\n\nConsumers of the resource must be able to determine the intended applicability for the resource. Ideally, this information would be used programmatically to determine when and how it should be incorporated or exposed."]
#[derive(Default, Debug, Clone)]
pub struct UsageContext {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code that identifies the type of context being specified by this usage context."]
    pub r#code: Box<super::super::types::Coding>,
    #[doc = "A value that defines the context specified in this context of use. The interpretation of the value is defined by the code."]
    pub r#value: UsageContextValue,
}
impl crate::AnyResource for UsageContext {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for UsageContext {
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
        state.serialize_entry("code", &self.r#code)?;
        match self.r#value {
            UsageContextValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            UsageContextValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            UsageContextValue::Range(ref value) => {
                state.serialize_entry("valueRange", value)?;
            }
            UsageContextValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            UsageContextValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for UsageContext {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "valueReference")]
            ValueReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UsageContext;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("UsageContext")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<UsageContext, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#code: Option<Box<super::super::types::Coding>> = None;
                let mut r#value: Option<UsageContextValue> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(UsageContextValue::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value =
                                    Some(UsageContextValue::Quantity(map_access.next_value()?));
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value = Some(UsageContextValue::Range(map_access.next_value()?));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value =
                                    Some(UsageContextValue::Reference(map_access.next_value()?));
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "code",
                                        "valueCodeableConcept",
                                        "valueQuantity",
                                        "valueRange",
                                        "valueReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(UsageContext {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#code: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#value: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
