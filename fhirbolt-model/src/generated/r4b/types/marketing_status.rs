// Generated on 2023-04-08 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarketingStatus {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The country in which the marketing authorisation has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements."]
    pub r#country: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Where a Medicines Regulatory Agency has granted a marketing authorisation for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlled terminology The controlled term and the controlled term identifier shall be specified."]
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples."]
    pub r#status: Box<super::super::types::CodeableConcept>,
    #[doc = "The date when the Medicinal Product is placed on the market by the Marketing Authorisation Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain."]
    pub r#date_range: Option<Box<super::super::types::Period>>,
    #[doc = "The date when the Medicinal Product is placed on the market by the Marketing Authorisation Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain."]
    pub r#restore_date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for MarketingStatus {
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
            if let Some(some) = self.r#country.as_ref() {
                state.serialize_entry("country", some)?;
            }
            if let Some(some) = self.r#jurisdiction.as_ref() {
                state.serialize_entry("jurisdiction", some)?;
            }
            state.serialize_entry("status", &self.r#status)?;
            if let Some(some) = self.r#date_range.as_ref() {
                state.serialize_entry("dateRange", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#restore_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("restoreDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_restoreDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#restore_date.as_ref() {
                    state.serialize_entry("restoreDate", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MarketingStatus {
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
            #[serde(rename = "country")]
            Country,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "dateRange")]
            DateRange,
            #[serde(rename = "restoreDate")]
            RestoreDate,
            #[serde(rename = "_restoreDate")]
            RestoreDatePrimitiveElement,
            Unknown(std::string::String),
        }
        fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
            Err(E::unknown_field(
                field,
                &[
                    "id",
                    "extension",
                    "modifierExtension",
                    "country",
                    "jurisdiction",
                    "status",
                    "dateRange",
                    "restoreDate",
                ],
            ))
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MarketingStatus;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MarketingStatus")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MarketingStatus, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#country: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#jurisdiction: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#date_range: Option<Box<super::super::types::Period>> = None;
                let mut r#restore_date: Option<super::super::types::DateTime> = None;
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
                            Field::Country => {
                                if r#country.is_some() {
                                    return Err(serde::de::Error::duplicate_field("country"));
                                }
                                r#country = Some(map_access.next_value()?);
                            }
                            Field::Jurisdiction => {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::DateRange => {
                                if r#date_range.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateRange"));
                                }
                                r#date_range = Some(map_access.next_value()?);
                            }
                            Field::RestoreDate => {
                                if _ctx.from_json {
                                    let some = r#restore_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "restoreDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#restore_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "restoreDate",
                                        ));
                                    }
                                    r#restore_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::RestoreDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#restore_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_restoreDate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return unknown_field_error("restoreDate");
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
                                        "country",
                                        "jurisdiction",
                                        "status",
                                        "dateRange",
                                        "restoreDate",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MarketingStatus {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#country,
                        r#jurisdiction,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#date_range,
                        r#restore_date,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
