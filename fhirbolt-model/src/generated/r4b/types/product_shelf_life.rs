// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class."]
#[derive(Default, Debug, Clone)]
pub struct ProductShelfLife {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for the packaged Medicinal Product."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life after the first opening of a bottle, etc. The shelf life type shall be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#period: Box<super::super::types::Quantity>,
    #[doc = "Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified."]
    pub r#special_precautions_for_storage: Vec<Box<super::super::types::CodeableConcept>>,
}
impl crate::AnyResource for ProductShelfLife {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for ProductShelfLife {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        state.serialize_entry("period", &self.r#period)?;
        if !self.r#special_precautions_for_storage.is_empty() {
            state.serialize_entry(
                "specialPrecautionsForStorage",
                &self.r#special_precautions_for_storage,
            )?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ProductShelfLife {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "specialPrecautionsForStorage")]
            SpecialPrecautionsForStorage,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProductShelfLife;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProductShelfLife")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProductShelfLife, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<super::super::types::Quantity>> = None;
                let mut r#special_precautions_for_storage: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::SpecialPrecautionsForStorage => {
                                if r#special_precautions_for_storage.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialPrecautionsForStorage",
                                    ));
                                }
                                r#special_precautions_for_storage = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "identifier",
                                        "type",
                                        "period",
                                        "specialPrecautionsForStorage",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ProductShelfLife {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#type: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#period: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#period.unwrap_or(Default::default())
                        } else {
                            r#period.ok_or(serde::de::Error::missing_field("period"))?
                        },
                        r#special_precautions_for_storage: r#special_precautions_for_storage
                            .unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
