// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.\n\nThere are a number of places where content must be signed in healthcare."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Signature {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An indication of the reason that the entity signed this document. This may be explicitly included as part of the signature information and can be used when determining accountability for various actions concerning the document."]
    pub r#type: Vec<Box<super::super::types::Coding>>,
    #[doc = "When the digital signature was signed."]
    pub r#when: super::super::types::Instant,
    #[doc = "A reference to an application-usable description of the identity that signed  (e.g. the signature used their private key)."]
    pub r#who: Box<super::super::types::Reference>,
    #[doc = "A reference to an application-usable description of the identity that is represented by the signature."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "A mime type that indicates the technical format of the target resources signed by the signature."]
    pub r#target_format: Option<super::super::types::Code>,
    #[doc = "A mime type that indicates the technical format of the signature. Important mime types are application/signature+xml for X ML DigSig, application/jose for JWS, and image/* for a graphical image of a signature, etc."]
    pub r#sig_format: Option<super::super::types::Code>,
    #[doc = "The base64 encoding of the Signature content. When signature is not recorded electronically this element would be empty."]
    pub r#data: Option<super::super::types::Base64Binary>,
}
impl serde::ser::Serialize for Signature {
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
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#when.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("when", &some)?;
                }
                if self.r#when.id.is_some() || !self.r#when.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#when.id.as_ref(),
                        extension: &self.r#when.extension,
                    };
                    state.serialize_entry("_when", &primitive_element)?;
                }
            } else {
                state.serialize_entry("when", &self.r#when)?;
            }
            state.serialize_entry("who", &self.r#who)?;
            if let Some(some) = self.r#on_behalf_of.as_ref() {
                state.serialize_entry("onBehalfOf", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#target_format.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("targetFormat", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_targetFormat", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#target_format.as_ref() {
                    state.serialize_entry("targetFormat", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#sig_format.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sigFormat", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sigFormat", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sig_format.as_ref() {
                    state.serialize_entry("sigFormat", some)?;
                }
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
impl<'de> serde::de::Deserialize<'de> for Signature {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "when")]
            When,
            #[serde(rename = "_when")]
            WhenPrimitiveElement,
            #[serde(rename = "who")]
            Who,
            #[serde(rename = "onBehalfOf")]
            OnBehalfOf,
            #[serde(rename = "targetFormat")]
            TargetFormat,
            #[serde(rename = "_targetFormat")]
            TargetFormatPrimitiveElement,
            #[serde(rename = "sigFormat")]
            SigFormat,
            #[serde(rename = "_sigFormat")]
            SigFormatPrimitiveElement,
            #[serde(rename = "data")]
            Data,
            #[serde(rename = "_data")]
            DataPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Signature;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Signature")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Signature, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#when: Option<super::super::types::Instant> = None;
                let mut r#who: Option<Box<super::super::types::Reference>> = None;
                let mut r#on_behalf_of: Option<Box<super::super::types::Reference>> = None;
                let mut r#target_format: Option<super::super::types::Code> = None;
                let mut r#sig_format: Option<super::super::types::Code> = None;
                let mut r#data: Option<super::super::types::Base64Binary> = None;
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
                            Field::Type => {
                                if _ctx.from_json {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#type.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::When => {
                                if _ctx.from_json {
                                    let some = r#when.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("when"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#when.is_some() {
                                        return Err(serde::de::Error::duplicate_field("when"));
                                    }
                                    r#when = Some(map_access.next_value()?);
                                }
                            }
                            Field::WhenPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#when.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_when"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "when",
                                        &[
                                            "id",
                                            "extension",
                                            "type",
                                            "when",
                                            "who",
                                            "onBehalfOf",
                                            "targetFormat",
                                            "sigFormat",
                                            "data",
                                        ],
                                    ));
                                }
                            }
                            Field::Who => {
                                if r#who.is_some() {
                                    return Err(serde::de::Error::duplicate_field("who"));
                                }
                                r#who = Some(map_access.next_value()?);
                            }
                            Field::OnBehalfOf => {
                                if r#on_behalf_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                                }
                                r#on_behalf_of = Some(map_access.next_value()?);
                            }
                            Field::TargetFormat => {
                                if _ctx.from_json {
                                    let some = r#target_format.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "targetFormat",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#target_format.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "targetFormat",
                                        ));
                                    }
                                    r#target_format = Some(map_access.next_value()?);
                                }
                            }
                            Field::TargetFormatPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#target_format.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_targetFormat",
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
                                        "targetFormat",
                                        &[
                                            "id",
                                            "extension",
                                            "type",
                                            "when",
                                            "who",
                                            "onBehalfOf",
                                            "targetFormat",
                                            "sigFormat",
                                            "data",
                                        ],
                                    ));
                                }
                            }
                            Field::SigFormat => {
                                if _ctx.from_json {
                                    let some = r#sig_format.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sigFormat"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sig_format.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sigFormat"));
                                    }
                                    r#sig_format = Some(map_access.next_value()?);
                                }
                            }
                            Field::SigFormatPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#sig_format.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_sigFormat",
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
                                        "sigFormat",
                                        &[
                                            "id",
                                            "extension",
                                            "type",
                                            "when",
                                            "who",
                                            "onBehalfOf",
                                            "targetFormat",
                                            "sigFormat",
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
                                            "type",
                                            "when",
                                            "who",
                                            "onBehalfOf",
                                            "targetFormat",
                                            "sigFormat",
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
                                        "type",
                                        "when",
                                        "who",
                                        "onBehalfOf",
                                        "targetFormat",
                                        "sigFormat",
                                        "data",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Signature {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#type: r#type.unwrap_or(vec![]),
                        r#when: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#when.unwrap_or(Default::default())
                        } else {
                            r#when.ok_or(serde::de::Error::missing_field("when"))?
                        },
                        r#who: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#who.unwrap_or(Default::default())
                        } else {
                            r#who.ok_or(serde::de::Error::missing_field("who"))?
                        },
                        r#on_behalf_of,
                        r#target_format,
                        r#sig_format,
                        r#data,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
